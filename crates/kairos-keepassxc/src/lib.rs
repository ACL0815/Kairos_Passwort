//! Independent, synchronous KeePassXC browser-protocol client.
//!
//! Modelled after: KeePassXC BrowserAction/BrowserMessageBuilder 2.7.12
//! <https://github.com/keepassxreboot/keepassxc/tree/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser>.
//! Modelled after: keepassxc-browser's protocol and git-credential-keepassxc.
//! Source evaluations and transport details: `docs/sources/keepassxc.md`.
//! No implementation code was copied from these projects.
//!
//! Call from a blocking application worker. No hotkey, focus, clipboard, browser
//! discovery, password generation, or automatic retry is performed here.

mod protocol;
mod storage;
#[cfg(windows)]
mod windows;

pub use storage::{Association, AssociationStore, DpapiStore};
#[cfg(windows)]
pub use windows::NamedPipeTransport;

use base64::{engine::general_purpose::STANDARD, Engine};
use protocol::{Envelope, Request, Session};
use std::fmt;
use zeroize::Zeroizing;

/// Maximum size of one JSON object, including its encrypted/base64 envelope.
pub const MAX_MESSAGE_BYTES: usize = 1024 * 1024;
const MAX_FIELD_BYTES: usize = 16 * 1024;

/// Errors deliberately contain no server text, paths, URLs, keys, or passwords.
/// A transport error during `set_login` can mean that the request already ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    UnsupportedPlatform,
    InvalidInput,
    RandomSourceUnavailable,
    Transport,
    Timeout,
    InvalidContext,
    MessageTooLarge,
    InvalidMessage,
    UnexpectedAction,
    InvalidNonce,
    Crypto,
    /// A numeric, unauthenticated protocol error; never treat it as proof of identity.
    Rejected(u32),
    MissingAssociation,
    AlreadyAssociated,
    DatabaseChanged,
    Storage,
    KeyProtection,
    /// A previous protocol/transport failure invalidated this connection.
    SessionClosed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedPlatform => "KeePassXC integration is unsupported on this platform",
            Self::InvalidInput => "Invalid KeePassXC request input",
            Self::RandomSourceUnavailable => "System randomness is unavailable",
            Self::Transport => "KeePassXC transport failed; request outcome may be unknown",
            Self::Timeout => "KeePassXC timed out; request outcome may be unknown",
            Self::InvalidContext => {
                "Use KeePassXC transport from a blocking worker outside a Tokio runtime"
            }
            Self::MessageTooLarge => "KeePassXC message exceeds the size limit",
            Self::InvalidMessage => "Invalid KeePassXC message",
            Self::UnexpectedAction => "Unexpected KeePassXC response action",
            Self::InvalidNonce => "Invalid KeePassXC response nonce",
            Self::Crypto => "KeePassXC message authentication failed",
            Self::Rejected(_) => "KeePassXC rejected the request",
            Self::MissingAssociation => "No KeePassXC association is stored",
            Self::AlreadyAssociated => "An association is already stored; use test_associate",
            Self::DatabaseChanged => "KeePassXC database does not match the stored association",
            Self::Storage => "KeePassXC association storage failed",
            Self::KeyProtection => "KeePassXC association protection failed",
            Self::SessionClosed => "KeePassXC connection must be recreated",
        })
    }
}

impl std::error::Error for Error {}

/// One sequential request/reply exchange. Implementations must enforce a timeout
/// and `MAX_MESSAGE_BYTES`, assemble fragmented JSON, and skip only the documented
/// `database-locked`/`database-unlocked` broadcasts. Never log request or reply data.
/// An error must not cause the request to be resent.
pub trait Transport {
    fn exchange(&mut self, request: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error>;
}

/// An existing group, identified by both its name and 32-digit hexadecimal UUID.
/// KeePassXC falls back to its default group if the UUID no longer exists.
#[derive(Clone, Copy)]
pub struct Group<'a> {
    pub name: &'a str,
    pub uuid: &'a str,
}

/// KeePassXC accepted the request. Its protocol does not return the new entry UUID
/// or prove that database selection, group selection, or disk persistence succeeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetLoginOutcome {
    Accepted,
}

/// A session has fresh client/host keys and clientID; identification is persisted
/// separately by the supplied store. Intentionally neither `Debug` nor `Clone`.
pub struct Client<T, S> {
    transport: T,
    store: S,
    session: Session,
    closed: bool,
}

impl<T: Transport, S: AssociationStore> Client<T, S> {
    /// Exchanges ephemeral public keys; does not associate, unlock, or modify a DB.
    pub fn connect(mut transport: T, store: S) -> Result<Self, Error> {
        let mut session = Session::generate()?;
        let nonce = protocol::random_nonce()?;
        let request = session.key_exchange(&nonce)?;
        let reply = transport.exchange(&request)?;
        session.accept_host(&reply, &nonce)?;
        Ok(Self {
            transport,
            store,
            session,
            closed: false,
        })
    }

    /// Explicit pairing. KeePassXC asks the user to name/approve this connection.
    /// The store is insert-only: an existing or corrupt record is never replaced.
    /// A storage failure after approval leaves a remote association to revoke in
    /// KeePassXC; this method does not attempt a second pairing automatically.
    pub fn associate(&mut self) -> Result<(), Error> {
        self.ensure_open()?;
        if self.store.load()?.is_some() {
            return Err(Error::AlreadyAssociated);
        }
        let id_key = Session::identification_key()?;
        let encoded = Zeroizing::new(STANDARD.encode(id_key.as_slice()));
        let public_key = self.session.public_key();
        let mut request = Request::new("associate");
        request.key = Some(&public_key);
        request.id_key = Some(&encoded);
        let mut reply = self.request(&request)?;
        let result = (|| {
            let association = Association::new(
                reply.hash.take().ok_or(Error::InvalidMessage)?,
                reply.id.take().ok_or(Error::InvalidMessage)?,
                id_key,
            )?;
            self.store.save(&association)
        })();
        if result.is_err() {
            // Approval may already have created a remote identity. A failed
            // publication must not allow another pairing on this connection.
            self.closed = true;
        }
        result
    }

    /// Verifies the stored bearer key against the currently active database.
    /// Returns the database hash, never the association ID or identification key.
    pub fn test_associate(&mut self) -> Result<String, Error> {
        self.ensure_open()?;
        let association = self.store.load()?.ok_or(Error::MissingAssociation)?;
        self.authenticate(&association)?;
        Ok(association.database_hash().to_owned())
    }

    /// Gets the active database hash without requesting an unlock or pairing.
    pub fn get_databasehash(&mut self) -> Result<String, Error> {
        let reply = self.request(&Request::new("get-databasehash"))?;
        match reply.hash.filter(|s| !s.is_empty()) {
            Some(hash) => Ok(hash),
            None => {
                self.closed = true;
                Err(Error::InvalidMessage)
            }
        }
    }

    /// Requests a NEW entry; deliberately never sends an entry `uuid`.
    /// URL must be an explicit absolute HTTP(S) URI, with no embedded credentials.
    /// International domains/paths must use punycode/percent encoding.
    /// `None` uses KeePassXC's default group; `Some` supplies an existing group.
    ///
    /// Revalidates the association immediately before sending. A later database
    /// selection/race is governed by KeePassXC; see `SetLoginOutcome`. Never retry
    /// automatically after an error: a new entry may already have been created.
    pub fn set_login(
        &mut self,
        url: &str,
        username: &str,
        password: &str,
        group: Option<Group<'_>>,
    ) -> Result<SetLoginOutcome, Error> {
        validate_url(url)?;
        if username.len() > MAX_FIELD_BYTES
            || password.is_empty()
            || password.len() > MAX_FIELD_BYTES
        {
            return Err(Error::InvalidInput);
        }
        if let Some(group) = group {
            if group.name.is_empty()
                || group.name.len() > MAX_FIELD_BYTES
                || group.uuid.len() != 32
                || !group.uuid.bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Err(Error::InvalidInput);
            }
        }
        self.ensure_open()?;
        let association = self.store.load()?.ok_or(Error::MissingAssociation)?;
        self.authenticate(&association)?;
        let mut request = Request::new("set-login");
        request.id = Some(association.id());
        request.url = Some(url);
        request.submit_url = Some(url);
        request.login = Some(username);
        request.password = Some(password);
        request.group = group.map(|g| g.name);
        request.group_uuid = group.map(|g| g.uuid);
        let reply = self.request(&request)?;
        if reply.hash.as_deref() != Some(association.database_hash()) {
            self.closed = true;
            return Err(Error::DatabaseChanged);
        }
        Ok(SetLoginOutcome::Accepted)
    }

    fn authenticate(&mut self, association: &Association) -> Result<(), Error> {
        let key = Zeroizing::new(STANDARD.encode(association.id_key()));
        let mut request = Request::new("test-associate");
        request.id = Some(association.id());
        request.key = Some(&key);
        let reply = self.request(&request)?;
        if reply.hash.as_deref() != Some(association.database_hash())
            || reply.id.as_deref().map(|s| s.as_str()) != Some(association.id())
        {
            self.closed = true;
            return Err(Error::DatabaseChanged);
        }
        Ok(())
    }

    fn ensure_open(&self) -> Result<(), Error> {
        if self.closed {
            Err(Error::SessionClosed)
        } else {
            Ok(())
        }
    }

    fn request(&mut self, request: &Request<'_>) -> Result<Envelope, Error> {
        self.ensure_open()?;
        // A failed exchange closes the logical session. A late reply after a
        // timeout must never be consumed by a subsequent operation.
        let result = (|| {
            let nonce = protocol::random_nonce()?;
            let bytes = self.session.encrypt_request(request, &nonce)?;
            let response = self.transport.exchange(&bytes)?;
            self.session
                .decrypt_response(&response, request.action, &nonce)
        })();
        if result.is_err() {
            self.closed = true;
        }
        result
    }
}

/// Conservative validation, without guessing schemes, removing path/query data,
/// or discovering a browser tab. The original URL is transmitted unchanged.
/// Callers must avoid URLs containing tokens in the path/query themselves.
pub fn validate_url(input: &str) -> Result<(), Error> {
    if input.is_empty()
        || input.len() > MAX_FIELD_BYTES
        || input.chars().any(|c| c.is_control() || c.is_whitespace())
        || input.contains('\\')
        || !(input.starts_with("https://") || input.starts_with("http://"))
    {
        return Err(Error::InvalidInput);
    }
    let parsed = fluent_uri::Uri::parse(input).map_err(|_| Error::InvalidInput)?;
    let authority = parsed.authority().ok_or(Error::InvalidInput)?;
    if authority.host().is_empty()
        || authority.host().contains('%')
        || authority.userinfo().is_some()
        || authority.port_to_u16().is_err()
    {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
