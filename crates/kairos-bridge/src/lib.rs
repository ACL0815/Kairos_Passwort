//! Context-only loopback service; never receives or returns generated passwords.
//! Modelled after: Syncthing's Host/CSRF boundary and KeePassXC association.
//! See docs/sources/browser.md. Independently implemented, no upstream code copied.

mod http;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use kairos_keepassxc::{Association, AssociationStore, DpapiStore};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq;
use zeroize::Zeroizing;

pub use http::Bridge;
pub const PORT: u16 = 19471;
pub const FRESH_FOR: Duration = Duration::from_secs(5);
const CLOCK_WINDOW: u64 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Io,
    Storage,
    Invalid,
    Unauthorized,
    Replay,
    Expired,
    AlreadyPaired,
    Random,
}

/// Opaque HWND/PID captured locally, never accepted from a webpage or userscript.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserWindow {
    pub hwnd: usize,
    pub process: u32,
}

#[derive(Clone)]
pub struct Context {
    pub url: String,
    pub username: Option<String>,
}

pub trait SecretStore: Send {
    fn load(&mut self) -> Result<Option<Zeroizing<String>>, Error>;
    fn save(&mut self, token: &str) -> Result<(), Error>;
}

/// Separate insert-only DPAPI record, sharing the audited CurrentUser adapter.
/// The marker prevents interpreting a KeePassXC association as a bridge key.
pub struct ProtectedStore(pub DpapiStore);
impl ProtectedStore {
    pub fn for_current_user() -> Result<Self, Error> {
        let store = DpapiStore::for_current_user().map_err(|_| Error::Storage)?;
        Ok(Self(DpapiStore::new(
            store.path().with_file_name("browser-bridge.dpapi"),
        )))
    }
}
impl SecretStore for ProtectedStore {
    fn load(&mut self) -> Result<Option<Zeroizing<String>>, Error> {
        self.0
            .load()
            .map_err(|_| Error::Storage)?
            .map(|a| {
                if a.database_hash() != "kairos-bridge-v1" || a.id() != "browser-context" {
                    return Err(Error::Storage);
                }
                Ok(Zeroizing::new(URL_SAFE_NO_PAD.encode(a.id_key())))
            })
            .transpose()
    }
    fn save(&mut self, token: &str) -> Result<(), Error> {
        let mut raw = Zeroizing::new([0u8; 32]);
        if URL_SAFE_NO_PAD
            .decode_slice(token, raw.as_mut())
            .map_err(|_| Error::Invalid)?
            != 32
        {
            return Err(Error::Invalid);
        }
        let association = Association::new(
            "kairos-bridge-v1".into(),
            Zeroizing::new("browser-context".into()),
            raw,
        )
        .map_err(|_| Error::Storage)?;
        self.0.save(&association).map_err(|_| Error::Storage)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingStatus {
    Unpaired,
    Pending,
    Paired,
}
struct Pending {
    token: Zeroizing<String>,
    until: Instant,
    attempts: u8,
}
struct Active {
    context: Context,
    window: BrowserWindow,
    received: Instant,
}

pub(crate) struct State {
    store: Box<dyn SecretStore>,
    token: Option<Zeroizing<String>>,
    pending: Option<Pending>,
    nonces: HashMap<String, Instant>,
    active: Option<Active>,
}

pub(crate) type Shared = Arc<Mutex<State>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Message {
    timestamp: u64,
    nonce: String,
    url: Option<String>,
    username: Option<String>,
    client_token: Option<Zeroizing<String>>,
}

pub fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Query/fragment are deliberately discarded on both ends. Credentials, controls,
/// Unicode ambiguity and non-HTTP(S) URLs are rejected by the KeePassXC validator.
pub fn normalize_url(input: &str) -> Result<String, Error> {
    kairos_keepassxc::validate_url(input).map_err(|_| Error::Invalid)?;
    if input.len() > 4096 {
        return Err(Error::Invalid);
    }
    let end = input.find(['?', '#']).unwrap_or(input.len());
    Ok(input[..end].to_owned())
}

impl State {
    fn new(mut store: Box<dyn SecretStore>) -> Result<Self, Error> {
        let token = store.load()?;
        if token.as_ref().is_some_and(|key| key.len() != 43) {
            return Err(Error::Storage);
        }
        Ok(Self {
            store,
            token,
            pending: None,
            nonces: HashMap::new(),
            active: None,
        })
    }
    fn status(&self) -> PairingStatus {
        if self.token.is_some() {
            PairingStatus::Paired
        } else if self
            .pending
            .as_ref()
            .is_some_and(|p| Instant::now() < p.until)
        {
            PairingStatus::Pending
        } else {
            PairingStatus::Unpaired
        }
    }
    fn begin_pairing(&mut self) -> Result<Zeroizing<String>, Error> {
        if self.token.is_some() {
            return Err(Error::AlreadyPaired);
        }
        let mut key = Zeroizing::new([0u8; 32]);
        getrandom::getrandom(key.as_mut()).map_err(|_| Error::Random)?;
        let token = Zeroizing::new(URL_SAFE_NO_PAD.encode(key.as_slice()));
        self.pending = Some(Pending {
            token: Zeroizing::new(token.to_string()),
            until: Instant::now() + Duration::from_secs(120),
            attempts: 0,
        });
        Ok(token)
    }
    fn context(&self, window: Option<BrowserWindow>) -> Option<Context> {
        let active = self.active.as_ref()?;
        (window == Some(active.window) && active.received.elapsed() <= FRESH_FOR)
            .then(|| active.context.clone())
    }
    fn handle(
        &mut self,
        path: &str,
        token: &str,
        body: &[u8],
        window: Option<BrowserWindow>,
    ) -> Result<(), Error> {
        let pair = path == "/v1/pair";
        if !matches!(path, "/v1/pair" | "/v1/context" | "/v1/clear") {
            return Err(Error::Invalid);
        }
        if pair {
            if self.token.is_some() {
                return Err(Error::AlreadyPaired);
            }
            let pending = self.pending.as_mut().ok_or(Error::Unauthorized)?;
            if Instant::now() >= pending.until || pending.attempts >= 5 {
                self.pending = None;
                return Err(Error::Expired);
            }
            pending.attempts += 1;
            if !bool::from(pending.token.as_bytes().ct_eq(token.as_bytes())) {
                return Err(Error::Unauthorized);
            }
        } else {
            let expected = self.token.as_ref().ok_or(Error::Unauthorized)?;
            if !bool::from(expected.as_bytes().ct_eq(token.as_bytes())) {
                return Err(Error::Unauthorized);
            }
        }
        let message: Message = serde_json::from_slice(body).map_err(|_| Error::Invalid)?;
        if unix_seconds().abs_diff(message.timestamp) > CLOCK_WINDOW {
            return Err(Error::Expired);
        }
        if message.nonce.len() != 32 || !message.nonce.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::Invalid);
        }
        self.nonces
            .retain(|_, received| received.elapsed() <= Duration::from_secs(61));
        if self.nonces.contains_key(&message.nonce) {
            return Err(Error::Replay);
        }
        if self.nonces.len() >= 1024 {
            return Err(Error::Invalid);
        }
        self.nonces.insert(message.nonce, Instant::now());
        if pair {
            if message.url.is_some() || message.username.is_some() {
                return Err(Error::Invalid);
            }
            let key = message.client_token.ok_or(Error::Invalid)?;
            if key.len() != 43
                || !key
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            {
                return Err(Error::Invalid);
            }
            self.store.save(&key)?;
            self.token = Some(key);
            self.pending = None;
        } else if path == "/v1/clear" {
            if message.client_token.is_some() {
                return Err(Error::Invalid);
            }
            self.active = None;
        } else {
            if message.client_token.is_some() {
                return Err(Error::Invalid);
            }
            // Clear first so rejected/new contexts cannot leave a previous tab usable.
            self.active = None;
            let url = normalize_url(message.url.as_deref().ok_or(Error::Invalid)?)?;
            if message
                .username
                .as_ref()
                .is_some_and(|s| s.len() > 320 || s.chars().any(char::is_control))
            {
                return Err(Error::Invalid);
            }
            if let Some(window) = window {
                self.active = Some(Active {
                    context: Context {
                        url,
                        username: message.username,
                    },
                    window,
                    received: Instant::now(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
