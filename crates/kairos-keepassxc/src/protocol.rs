//! NaCl wire format and response authentication.
//!
//! Modelled after: KeePassXC BrowserMessageBuilder.cpp (2.7.12) and
//! <https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-protocol.md>.

use crate::{Error, MAX_MESSAGE_BYTES};
use base64::{engine::general_purpose::STANDARD, Engine};
use crypto_box::{aead::AeadInPlace, PublicKey, SalsaBox, SecretKey};
use curve25519_dalek::MontgomeryPoint;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, Zeroizing};

#[derive(Serialize)]
pub(crate) struct Request<'a> {
    pub action: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<&'a str>,
    #[serde(rename = "idKey", skip_serializing_if = "Option::is_none")]
    pub id_key: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(rename = "submitUrl", skip_serializing_if = "Option::is_none")]
    pub submit_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<&'a str>,
    #[serde(rename = "groupUuid", skip_serializing_if = "Option::is_none")]
    pub group_uuid: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn new(action: &'a str) -> Self {
        Self {
            action,
            key: None,
            id_key: None,
            id: None,
            url: None,
            submit_url: None,
            login: None,
            password: None,
            group: None,
            group_uuid: None,
        }
    }
}

// Server error text is intentionally not retained. The string-or-number code is
// the only unauthenticated error information exposed to callers.
#[derive(Deserialize, Default)]
pub(crate) struct Envelope {
    pub action: Option<String>,
    pub nonce: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "publicKey")]
    pub public_key: Option<String>,
    pub success: Option<String>,
    pub hash: Option<String>,
    pub id: Option<Zeroizing<String>>,
    #[serde(rename = "errorCode")]
    error_code: Option<ErrorCode>,
    pub error: Option<Zeroizing<String>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ErrorCode {
    Text(String),
    Number(u32),
}

impl Envelope {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > MAX_MESSAGE_BYTES {
            return Err(Error::MessageTooLarge);
        }
        serde_json::from_slice(bytes).map_err(|_| Error::InvalidMessage)
    }

    fn check_action(&self, expected: &str) -> Result<(), Error> {
        if self.action.as_deref() != Some(expected) {
            return Err(Error::UnexpectedAction);
        }
        Ok(())
    }

    fn check_error(&self) -> Result<(), Error> {
        if let Some(code) = &self.error_code {
            let number = match code {
                ErrorCode::Text(value) => value.parse().map_err(|_| Error::InvalidMessage)?,
                ErrorCode::Number(value) => *value,
            };
            return Err(Error::Rejected(number));
        }
        Ok(())
    }

    fn check_success(&self) -> Result<(), Error> {
        self.check_error()?;
        if self.success.as_deref() != Some("true") {
            return Err(Error::InvalidMessage);
        }
        Ok(())
    }
}

pub(crate) struct Session {
    secret: Box<Zeroizing<[u8; 32]>>,
    public: String,
    client_id: String,
    cipher: Option<SalsaBox>,
}

impl Session {
    pub fn generate() -> Result<Self, Error> {
        let secret = random_key()?;
        let public = public_from_secret(&secret);
        Ok(Self {
            secret,
            public: STANDARD.encode(public),
            client_id: STANDARD.encode(random_nonce()?),
            cipher: None,
        })
    }

    pub fn identification_key() -> Result<Zeroizing<[u8; 32]>, Error> {
        let secret = random_key()?;
        // Only the public half of a separate key pair is the permanent bearer
        // credential in this protocol. Its private half is immediately discarded.
        Ok(Zeroizing::new(public_from_secret(&secret)))
    }

    pub fn public_key(&self) -> String {
        self.public.clone()
    }

    pub fn key_exchange(&self, nonce: &[u8; 24]) -> Result<Zeroizing<Vec<u8>>, Error> {
        #[derive(Serialize)]
        struct Exchange<'a> {
            action: &'a str,
            #[serde(rename = "publicKey")]
            public_key: &'a str,
            nonce: String,
            #[serde(rename = "clientID")]
            client_id: &'a str,
        }
        encode_json(&Exchange {
            action: "change-public-keys",
            public_key: &self.public,
            nonce: STANDARD.encode(nonce),
            client_id: &self.client_id,
        })
    }

    pub fn accept_host(&mut self, bytes: &[u8], nonce: &[u8; 24]) -> Result<(), Error> {
        let reply = Envelope::parse(bytes)?;
        reply.check_action("change-public-keys")?;
        reply.check_success()?;
        check_nonce(reply.nonce.as_deref(), nonce)?;
        let public = decode_fixed::<32>(reply.public_key.as_deref().ok_or(Error::InvalidMessage)?)?;
        // Reject ALL non-contributory peer keys, including alternate encodings.
        // This check does not authenticate the local server executable.
        let shared = Zeroizing::new(
            MontgomeryPoint(public)
                .mul_clamped(**self.secret)
                .to_bytes(),
        );
        if shared.iter().all(|b| *b == 0) {
            return Err(Error::Crypto);
        }
        let secret = WipedSecretKey::new(self.secret.as_slice())?;
        self.cipher = Some(SalsaBox::new(&PublicKey::from(public), secret.get()));
        self.secret.zeroize();
        Ok(())
    }

    pub fn encrypt_request(
        &self,
        request: &Request<'_>,
        nonce: &[u8; 24],
    ) -> Result<Zeroizing<Vec<u8>>, Error> {
        let mut plaintext = encode_json(request)?;
        let ciphertext = encrypt(
            self.cipher.as_ref().ok_or(Error::Crypto)?,
            nonce,
            &mut plaintext,
        )?;
        #[derive(Serialize)]
        struct Wire<'a> {
            action: &'a str,
            message: String,
            nonce: String,
            #[serde(rename = "clientID")]
            client_id: &'a str,
        }
        encode_json(&Wire {
            action: request.action,
            message: STANDARD.encode(ciphertext),
            nonce: STANDARD.encode(nonce),
            client_id: &self.client_id,
        })
    }

    pub fn decrypt_response(
        &self,
        bytes: &[u8],
        action: &str,
        sent_nonce: &[u8; 24],
    ) -> Result<Envelope, Error> {
        let envelope = Envelope::parse(bytes)?;
        envelope.check_action(action)?;
        envelope.check_error()?;
        let nonce = check_nonce(envelope.nonce.as_deref(), sent_nonce)?;
        let encoded = envelope.message.as_deref().ok_or(Error::InvalidMessage)?;
        let plaintext = decrypt(self.cipher.as_ref().ok_or(Error::Crypto)?, &nonce, encoded)?;
        let reply = Envelope::parse(&plaintext[16..])?;
        reply.check_success()?;
        if decode_fixed::<24>(reply.nonce.as_deref().ok_or(Error::InvalidNonce)?)? != nonce {
            return Err(Error::InvalidNonce);
        }
        // KeePassXC uses "success" here, while older protocol examples use "".
        if action == "set-login"
            && !matches!(
                reply.error.as_deref().map(|s| s.as_str()),
                None | Some("") | Some("success")
            )
        {
            return Err(Error::InvalidMessage);
        }
        Ok(reply)
    }
}

fn random_key() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut bytes = Box::new(Zeroizing::new([0; 32]));
    getrandom::getrandom(bytes.as_mut_slice()).map_err(|_| Error::RandomSourceUnavailable)?;
    Ok(bytes)
}

fn public_from_secret(bytes: &[u8; 32]) -> [u8; 32] {
    MontgomeryPoint::mul_base_clamped(*bytes).to_bytes()
}

// crypto_box 0.9.1's SecretKey::drop scrubs its scalar but leaves its raw `bytes`
// field. Keep the complete object in a stable zeroizing allocation. MaybeUninit
// permits zeroing the whole representation after running its destructor, without
// depending on private fields or layout. Ordinary constructor temporaries and
// compiler/OS copies cannot be comprehensively erased by this crate.
struct WipedSecretKey(Box<Zeroizing<std::mem::MaybeUninit<SecretKey>>>);

impl WipedSecretKey {
    fn new(bytes: &[u8]) -> Result<Self, Error> {
        let mut slot = Box::new(Zeroizing::new(std::mem::MaybeUninit::uninit()));
        slot.write(SecretKey::from_slice(bytes).map_err(|_| Error::Crypto)?);
        Ok(Self(slot))
    }
    fn get(&self) -> &SecretKey {
        // SAFETY: new initializes exactly once; this reference cannot outlive self.
        unsafe { self.0.assume_init_ref() }
    }
}

impl Drop for WipedSecretKey {
    fn drop(&mut self) {
        // SAFETY: every constructed owner contains one live SecretKey. Its
        // destructor runs once, then Zeroizing<MaybeUninit<_>> scrubs all bytes.
        unsafe {
            self.0.assume_init_drop();
        }
    }
}

pub(crate) fn random_nonce() -> Result<[u8; 24], Error> {
    let mut bytes = [0; 24];
    getrandom::getrandom(&mut bytes).map_err(|_| Error::RandomSourceUnavailable)?;
    Ok(bytes)
}

pub(crate) fn increment_nonce(mut nonce: [u8; 24]) -> [u8; 24] {
    for byte in &mut nonce {
        let (next, overflow) = byte.overflowing_add(1);
        *byte = next;
        if !overflow {
            break;
        }
    }
    nonce
}

fn check_nonce(value: Option<&str>, sent: &[u8; 24]) -> Result<[u8; 24], Error> {
    let actual =
        decode_fixed::<24>(value.ok_or(Error::InvalidNonce)?).map_err(|_| Error::InvalidNonce)?;
    if actual != increment_nonce(*sent) {
        return Err(Error::InvalidNonce);
    }
    Ok(actual)
}

pub(crate) fn decode_fixed<const N: usize>(value: &str) -> Result<[u8; N], Error> {
    let mut decoded = [0; N];
    let count = STANDARD
        .decode_slice(value, &mut decoded)
        .map_err(|_| Error::InvalidMessage)?;
    if count != N {
        return Err(Error::InvalidMessage);
    }
    Ok(decoded)
}

pub(crate) fn encode_json(value: &impl Serialize) -> Result<Zeroizing<Vec<u8>>, Error> {
    // Fixed allocation: serializing secrets cannot leave an old allocation after
    // Vec growth. Cursor fails instead of reallocating on oversized input.
    let mut bytes = Zeroizing::new(vec![0; MAX_MESSAGE_BYTES]);
    let used = {
        let mut output = std::io::Cursor::new(bytes.as_mut_slice());
        serde_json::to_writer(&mut output, value).map_err(|_| Error::MessageTooLarge)?;
        output.position() as usize
    };
    bytes.truncate(used);
    Ok(bytes)
}

pub(crate) fn encrypt(
    cipher: &SalsaBox,
    nonce: &[u8; 24],
    plaintext: &mut [u8],
) -> Result<Vec<u8>, Error> {
    let tag = cipher
        .encrypt_in_place_detached(nonce.into(), b"", plaintext)
        .map_err(|_| Error::Crypto)?;
    let mut result = Vec::with_capacity(16 + plaintext.len());
    result.extend_from_slice(&tag);
    result.extend_from_slice(plaintext);
    Ok(result)
}

pub(crate) fn decrypt(
    cipher: &SalsaBox,
    nonce: &[u8; 24],
    encoded: &str,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let mut bytes = Zeroizing::new(
        STANDARD
            .decode(encoded)
            .map_err(|_| Error::InvalidMessage)?,
    );
    if bytes.len() < 16 {
        return Err(Error::InvalidMessage);
    }
    let tag: [u8; 16] = bytes[..16].try_into().map_err(|_| Error::InvalidMessage)?;
    cipher
        .decrypt_in_place_detached(nonce.into(), b"", &mut bytes[16..], (&tag).into())
        .map_err(|_| Error::Crypto)?;
    // Retain the tag prefix and never truncate a plaintext-bearing tail.
    Ok(bytes)
}

/// Finds one raw JSON object, preserving coalesced objects for the next read.
/// This is structural framing only; full typed validation follows in the client.
#[cfg(any(test, windows))]
pub(crate) fn frame_length(bytes: &[u8]) -> Result<Option<usize>, Error> {
    if bytes.len() > MAX_MESSAGE_BYTES {
        return Err(Error::MessageTooLarge);
    }
    let mut stream =
        serde_json::Deserializer::from_slice(bytes).into_iter::<serde::de::IgnoredAny>();
    match stream.next() {
        None => Ok(None),
        Some(Ok(_)) => {
            if bytes.iter().find(|b| !b.is_ascii_whitespace()) != Some(&b'{') {
                return Err(Error::InvalidMessage);
            }
            Ok(Some(stream.byte_offset()))
        }
        Some(Err(error)) if error.is_eof() => Ok(None),
        Some(Err(_)) => Err(Error::InvalidMessage),
    }
}
