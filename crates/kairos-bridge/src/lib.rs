//! Context-only loopback service; never receives or returns generated passwords.
//! Modelled after: Syncthing's Host/CSRF boundary and KeePassXC association.
//! See docs/sources/browser.md. Independently implemented, no upstream code copied.

mod http;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use kairos_keepassxc::{Association, AssociationStore, DpapiStore};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

pub use http::Bridge;
pub const PORT: u16 = 19471;
pub const FRESH_FOR: Duration = Duration::from_secs(5);
const CLOCK_WINDOW: u64 = 30;
const HOST: &str = "127.0.0.1:19471";
const VERSION: &str = "kairos-bridge-v2";
const KEY_DERIVATION: &[u8] = b"kairos-bridge-v2-client-key";
type HmacSha256 = Hmac<Sha256>;

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
    RateLimited,
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
    fn save(&mut self, secret: &str) -> Result<(), Error>;
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
                if a.database_hash() != VERSION || a.id() != "browser-context" {
                    return Err(Error::Storage);
                }
                Ok(Zeroizing::new(URL_SAFE_NO_PAD.encode(a.id_key())))
            })
            .transpose()
    }
    fn save(&mut self, secret: &str) -> Result<(), Error> {
        let mut raw = Zeroizing::new([0u8; 32]);
        if URL_SAFE_NO_PAD
            .decode_slice(secret, raw.as_mut())
            .map_err(|_| Error::Invalid)?
            != 32
        {
            return Err(Error::Invalid);
        }
        let association = Association::new(
            VERSION.into(),
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
    secret: Zeroizing<String>,
    until: Instant,
    attempts: u8,
}
struct Active {
    context: Context,
    window: BrowserWindow,
    received: Instant,
    username_until: Option<Instant>,
}

pub(crate) struct State {
    store: Box<dyn SecretStore>,
    secret: Option<Zeroizing<String>>,
    pending: Option<Pending>,
    nonces: HashMap<String, Instant>,
    active: Option<Active>,
    last_context: Option<Instant>,
}

pub(crate) type Shared = Arc<Mutex<State>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Message {
    timestamp: u64,
    nonce: String,
    url: Option<String>,
    username: Option<String>,
}

fn decode_secret(secret: &str) -> Result<Zeroizing<[u8; 32]>, Error> {
    let mut raw = Zeroizing::new([0u8; 32]);
    if secret.len() != 43 || URL_SAFE_NO_PAD.decode_slice(secret, raw.as_mut()).ok() != Some(32) {
        return Err(Error::Invalid);
    }
    Ok(raw)
}

fn mac(secret: &str) -> Result<HmacSha256, Error> {
    let raw = decode_secret(secret)?;
    HmacSha256::new_from_slice(raw.as_ref()).map_err(|_| Error::Invalid)
}

fn derive_client_key(code: &str) -> Result<Zeroizing<String>, Error> {
    let mut hmac = mac(code)?;
    hmac.update(KEY_DERIVATION);
    let raw = Zeroizing::new(<[u8; 32]>::from(hmac.finalize().into_bytes()));
    Ok(Zeroizing::new(URL_SAFE_NO_PAD.encode(raw.as_ref())))
}

fn canonical(path: &str, host: &str, body: &[u8], message: &Message) -> String {
    format!(
        "{VERSION}\nPOST\n{path}\n{host}\n{}\n{}\n{:x}",
        message.timestamp,
        message.nonce,
        Sha256::digest(body)
    )
}

fn verify_hmac(secret: &str, signature: &str, canonical: &str) -> Result<(), Error> {
    if signature.len() != 64
        || !signature
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(Error::Unauthorized);
    }
    let mut tag = [0u8; 32];
    for (byte, hex) in tag.iter_mut().zip(signature.as_bytes().as_chunks::<2>().0) {
        let digit = |b: u8| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
        *byte = (digit(hex[0]) << 4) | digit(hex[1]);
    }
    let mut hmac = mac(secret)?;
    hmac.update(canonical.as_bytes());
    hmac.verify_slice(&tag).map_err(|_| Error::Unauthorized)
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
        let secret = store.load()?;
        if secret
            .as_ref()
            .is_some_and(|key| decode_secret(key).is_err())
        {
            return Err(Error::Storage);
        }
        Ok(Self {
            store,
            secret,
            pending: None,
            nonces: HashMap::new(),
            active: None,
            last_context: None,
        })
    }
    fn status(&self) -> PairingStatus {
        if self.secret.is_some() {
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
        if self.secret.is_some() {
            return Err(Error::AlreadyPaired);
        }
        let mut key = Zeroizing::new([0u8; 32]);
        getrandom::getrandom(key.as_mut()).map_err(|_| Error::Random)?;
        let secret = Zeroizing::new(URL_SAFE_NO_PAD.encode(key.as_slice()));
        self.pending = Some(Pending {
            secret: Zeroizing::new(secret.to_string()),
            until: Instant::now() + Duration::from_secs(120),
            attempts: 0,
        });
        Ok(secret)
    }
    fn context(&self, window: Option<BrowserWindow>) -> Option<Context> {
        let active = self.active.as_ref()?;
        (window == Some(active.window) && active.received.elapsed() <= FRESH_FOR)
            .then(|| active.context.clone())
    }
    fn handle(
        &mut self,
        path: &str,
        host: &str,
        signature: &str,
        body: &[u8],
        window: Option<BrowserWindow>,
    ) -> Result<(), Error> {
        let pair = path == "/v2/pair";
        if host != HOST || !matches!(path, "/v2/pair" | "/v2/context" | "/v2/clear" | "/v2/probe") {
            return Err(Error::Invalid);
        }
        let secret = if pair {
            if self.secret.is_some() {
                return Err(Error::AlreadyPaired);
            }
            let pending = self.pending.as_mut().ok_or(Error::Unauthorized)?;
            if Instant::now() >= pending.until || pending.attempts >= 5 {
                self.pending = None;
                return Err(Error::Expired);
            }
            pending.attempts += 1;
            &pending.secret
        } else {
            self.secret.as_ref().ok_or(Error::Unauthorized)?
        };
        let message: Message = serde_json::from_slice(body).map_err(|_| Error::Invalid)?;
        verify_hmac(secret, signature, &canonical(path, host, body, &message))?;
        if unix_seconds().abs_diff(message.timestamp) > CLOCK_WINDOW {
            return Err(Error::Expired);
        }
        if message.nonce.len() != 32
            || !message
                .nonce
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        self.nonces
            .retain(|_, received| received.elapsed() <= Duration::from_secs(61));
        if self.nonces.contains_key(&message.nonce) {
            return Err(Error::Replay);
        }
        if matches!(path, "/v2/context" | "/v2/clear") {
            if self
                .last_context
                .is_some_and(|last| last.elapsed() < Duration::from_millis(250))
            {
                return Err(Error::RateLimited);
            }
            self.last_context = Some(Instant::now());
        }
        if self.nonces.len() >= 1024 {
            return Err(Error::Invalid);
        }
        self.nonces.insert(message.nonce, Instant::now());
        if pair {
            if message.url.is_some() || message.username.is_some() {
                return Err(Error::Invalid);
            }
            let key = derive_client_key(&self.pending.as_ref().ok_or(Error::Unauthorized)?.secret)?;
            self.store.save(&key)?;
            self.secret = Some(key);
            self.pending = None;
        } else if matches!(path, "/v2/clear" | "/v2/probe") {
            if message.url.is_some() || message.username.is_some() {
                return Err(Error::Invalid);
            }
            if path == "/v2/clear" {
                self.active = None;
            }
        } else {
            // Clear first so rejected/new contexts cannot leave a previous tab usable.
            let previous = self.active.take();
            let url = normalize_url(message.url.as_deref().ok_or(Error::Invalid)?)?;
            if message
                .username
                .as_ref()
                .is_some_and(|s| s.len() > 320 || s.chars().any(char::is_control))
            {
                return Err(Error::Invalid);
            }
            if let Some(window) = window {
                let mut username = message.username;
                let mut username_until = username
                    .as_ref()
                    .map(|_| Instant::now() + Duration::from_secs(10));
                if username.is_none() {
                    if let Some(previous) = previous.filter(|a| {
                        a.window == window
                            && a.context.url == url
                            && a.username_until.is_some_and(|until| Instant::now() < until)
                    }) {
                        username = previous.context.username;
                        username_until = previous.username_until;
                    }
                }
                self.active = Some(Active {
                    context: Context { url, username },
                    window,
                    received: Instant::now(),
                    username_until,
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
