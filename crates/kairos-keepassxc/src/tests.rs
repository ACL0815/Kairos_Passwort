use super::*;
use crate::protocol::{decode_fixed, frame_length, increment_nonce};
use crypto_box::{aead::Aead, PublicKey, SalsaBox, SecretKey};
use serde_json::{json, Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Clone)]
struct MemoryStore {
    bytes: Rc<RefCell<Option<Zeroizing<Vec<u8>>>>>,
    fail: bool,
    fail_write: bool,
}

impl AssociationStore for MemoryStore {
    fn load(&mut self) -> Result<Option<Association>, Error> {
        if self.fail {
            return Err(Error::Storage);
        }
        self.bytes
            .borrow()
            .as_deref()
            .map(|b| Association::from_bytes(b))
            .transpose()
    }
    fn save(&mut self, association: &Association) -> Result<(), Error> {
        if self.fail || self.fail_write {
            return Err(Error::Storage);
        }
        let mut bytes = self.bytes.borrow_mut();
        if bytes.is_some() {
            return Err(Error::AlreadyAssociated);
        }
        *bytes = Some(association.to_bytes()?);
        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
enum Fault {
    #[default]
    None,
    NonceReplay,
    InnerNonce,
    Tamper,
    WrongAction,
    DatabaseChanged,
    Reject,
    TimeoutOnSet,
    MissingHash,
}

struct Peer {
    cipher: Option<SalsaBox>,
    requests: Rc<RefCell<Vec<Value>>>,
    fault: Fault,
    client_id: Option<String>,
}

impl Peer {
    fn new(fault: Fault) -> Self {
        Self {
            cipher: None,
            requests: Rc::default(),
            fault,
            client_id: None,
        }
    }
}

impl Transport for Peer {
    fn exchange(&mut self, request: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
        let outer: Value = serde_json::from_slice(request).unwrap();
        let nonce = decode_fixed::<24>(outer["nonce"].as_str().unwrap()).unwrap();
        let response_nonce = increment_nonce(nonce);
        let action = outer["action"].as_str().unwrap();
        if action == "change-public-keys" {
            let host_secret = SecretKey::from([7; 32]);
            let public = decode_fixed::<32>(outer["publicKey"].as_str().unwrap()).unwrap();
            self.cipher = Some(SalsaBox::new(&PublicKey::from(public), &host_secret));
            self.client_id = outer["clientID"].as_str().map(str::to_owned);
            assert_eq!(outer.as_object().unwrap().len(), 4);
            assert_eq!(
                decode_fixed::<24>(self.client_id.as_deref().unwrap())
                    .unwrap()
                    .len(),
                24
            );
            return protocol::encode_json(
                &json!({"action":action,"success":"true", "version":"2.7.12",
                "nonce":STANDARD.encode(response_nonce), "publicKey":STANDARD.encode(host_secret.public_key().as_bytes())}),
            );
        }
        assert_eq!(outer["clientID"].as_str(), self.client_id.as_deref());
        assert_eq!(outer.as_object().unwrap().len(), 4);
        let cipher = self.cipher.as_ref().unwrap();
        // Use the dependency's allocating NaCl API on the peer side, independently
        // of our detached-buffer implementation. Test passwords are fixed dummies.
        let encrypted = STANDARD.decode(outer["message"].as_str().unwrap()).unwrap();
        let plain = cipher
            .decrypt((&nonce).into(), encrypted.as_slice())
            .unwrap();
        let inner: Value = serde_json::from_slice(&plain).unwrap();
        assert_eq!(inner["action"], action);
        self.requests.borrow_mut().push(inner);
        if matches!(self.fault, Fault::TimeoutOnSet) && action == "set-login" {
            return Err(Error::Timeout);
        }
        if matches!(self.fault, Fault::Reject) {
            return protocol::encode_json(
                &json!({"action":action,"errorCode":"5", "error":"untrusted-secret-text"}),
            );
        }
        let hash = if matches!(self.fault, Fault::DatabaseChanged) {
            "other-db"
        } else {
            "test-db-hash"
        };
        let mut data = json!({"success":"true", "nonce":STANDARD.encode(response_nonce), "version":"2.7.12", "hash":hash});
        if matches!(self.fault, Fault::MissingHash) {
            data.as_object_mut().unwrap().remove("hash");
        }
        if matches!(self.fault, Fault::InnerNonce) {
            data["nonce"] = json!(STANDARD.encode(nonce));
        }
        if matches!(action, "associate" | "test-associate") {
            data["id"] = json!("kairos-test");
        }
        if action == "set-login" {
            data["error"] = json!("success");
        }
        let mut message = cipher
            .encrypt(
                (&response_nonce).into(),
                serde_json::to_vec(&data).unwrap().as_slice(),
            )
            .unwrap();
        if matches!(self.fault, Fault::Tamper) {
            message[0] ^= 1;
        }
        protocol::encode_json(
            &json!({"action":if matches!(self.fault, Fault::WrongAction) {"wrong"} else {action},
            "nonce":STANDARD.encode(if matches!(self.fault, Fault::NonceReplay) {nonce} else {response_nonce}),
            "message":STANDARD.encode(message)}),
        )
    }
}

fn paired(fault: Fault) -> Client<Peer, MemoryStore> {
    let mut client = Client::connect(Peer::new(Fault::None), MemoryStore::default()).unwrap();
    client.associate().unwrap();
    client.transport.fault = fault;
    client
}

#[test]
fn association_persists_and_new_session_authenticates_without_pairing() {
    let mut first = paired(Fault::None);
    let store = first.store.clone();
    assert_eq!(first.associate(), Err(Error::AlreadyAssociated));
    let mut second = Client::connect(Peer::new(Fault::None), store).unwrap();
    assert_eq!(second.test_associate().unwrap(), "test-db-hash");
    assert_eq!(second.get_databasehash().unwrap(), "test-db-hash");
    assert_eq!(
        second.transport.requests.borrow()[0]["action"],
        "test-associate"
    );
    assert_ne!(first.session.public_key(), second.session.public_key());
}

#[test]
fn messages_match_protocol_fixture_and_never_request_update_or_favicon() {
    let fixture: Value =
        serde_json::from_str(include_str!("../tests/fixtures/protocol.json")).unwrap();
    let mut client = paired(Fault::None);
    client.test_associate().unwrap();
    client.get_databasehash().unwrap();
    assert_eq!(
        client
            .set_login(
                "https://example.invalid/signup",
                "dummy-user",
                "dummy-Äpfel\"\\password",
                Some(Group {
                    name: "Test group",
                    uuid: "0123456789abcdef0123456789abcdef"
                })
            )
            .unwrap(),
        SetLoginOutcome::Accepted
    );
    let requests = client.transport.requests.borrow();
    let mut associate = requests[0].clone();
    assert_eq!(
        decode_fixed::<32>(associate["key"].as_str().unwrap())
            .unwrap()
            .len(),
        32
    );
    let identification = associate["idKey"].as_str().unwrap().to_owned();
    assert_eq!(decode_fixed::<32>(&identification).unwrap().len(), 32);
    associate["key"] = json!("<client public key>");
    associate["idKey"] = json!("<identification public key>");
    assert_eq!(associate, fixture["associate"]);
    let mut test = requests[1].clone();
    assert_eq!(test["key"], identification);
    test["key"] = json!("<identification public key>");
    assert_eq!(test, fixture["test-associate"]);
    assert_eq!(requests[2], fixture["get-databasehash"]);
    assert_eq!(requests[3]["action"], "test-associate");
    assert_eq!(requests[4], fixture["set-login"]);
    assert!(requests[4].get("uuid").is_none());
    assert!(requests[4].get("downloadFavicon").is_none());
}

#[test]
fn default_group_is_omitted_and_empty_username_is_allowed() {
    let mut client = paired(Fault::None);
    client
        .set_login("http://localhost:8080/", "", "dummy", None)
        .unwrap();
    let requests = client.transport.requests.borrow();
    let request = requests.last().unwrap();
    assert!(request.get("group").is_none());
    assert!(request.get("groupUuid").is_none());
    assert_eq!(request["login"], "");
}

#[test]
fn nonce_increment_is_little_endian_and_wraps() {
    let mut value = [0; 24];
    value[0] = 255;
    value[1] = 255;
    let next = increment_nonce(value);
    assert_eq!(&next[..4], &[0, 0, 1, 0]);
    assert_eq!(increment_nonce([255; 24]), [0; 24]);
}

#[test]
fn replay_inner_nonce_tamper_and_wrong_action_close_session() {
    for (fault, expected) in [
        (Fault::NonceReplay, Error::InvalidNonce),
        (Fault::InnerNonce, Error::InvalidNonce),
        (Fault::Tamper, Error::Crypto),
        (Fault::WrongAction, Error::UnexpectedAction),
    ] {
        let mut client = paired(fault);
        assert_eq!(client.get_databasehash(), Err(expected));
        let count = client.transport.requests.borrow().len();
        assert_eq!(client.get_databasehash(), Err(Error::SessionClosed));
        assert_eq!(client.transport.requests.borrow().len(), count);
    }
}

#[test]
fn database_mismatch_prevents_sending_password() {
    let mut client = paired(Fault::DatabaseChanged);
    assert_eq!(
        client.set_login("https://example.invalid/", "dummy", "dummy", None),
        Err(Error::DatabaseChanged)
    );
    assert!(!client
        .transport
        .requests
        .borrow()
        .iter()
        .any(|r| r["action"] == "set-login"));
}

#[test]
fn timeout_on_set_is_never_retried() {
    let mut client = paired(Fault::TimeoutOnSet);
    assert_eq!(
        client.set_login("https://example.invalid/", "dummy", "dummy", None),
        Err(Error::Timeout)
    );
    assert_eq!(
        client.set_login("https://example.invalid/", "dummy", "dummy", None),
        Err(Error::SessionClosed)
    );
    assert_eq!(
        client
            .transport
            .requests
            .borrow()
            .iter()
            .filter(|r| r["action"] == "set-login")
            .count(),
        1
    );
}

#[test]
fn missing_or_corrupt_store_never_causes_automatic_pairing() {
    let mut client = Client::connect(Peer::new(Fault::None), MemoryStore::default()).unwrap();
    assert_eq!(client.test_associate(), Err(Error::MissingAssociation));
    *client.store.bytes.borrow_mut() = Some(Zeroizing::new(b"corrupt".to_vec()));
    assert_eq!(client.associate(), Err(Error::Storage));
    assert!(client.transport.requests.borrow().is_empty());
    client.store.fail = true;
    assert_eq!(client.test_associate(), Err(Error::Storage));
}

#[test]
fn server_error_text_is_redacted() {
    let mut client = paired(Fault::Reject);
    let error = client.test_associate().unwrap_err();
    assert_eq!(error, Error::Rejected(5));
    assert!(!format!("{error} {error:?}").contains("untrusted-secret-text"));
}

#[test]
fn failed_publication_and_missing_response_hash_close_session() {
    let store = MemoryStore {
        fail_write: true,
        ..MemoryStore::default()
    };
    let mut client = Client::connect(Peer::new(Fault::None), store).unwrap();
    assert_eq!(client.associate(), Err(Error::Storage));
    assert_eq!(client.associate(), Err(Error::SessionClosed));
    assert_eq!(client.transport.requests.borrow().len(), 1);
    let mut client = paired(Fault::MissingHash);
    assert_eq!(client.get_databasehash(), Err(Error::InvalidMessage));
    assert_eq!(client.get_databasehash(), Err(Error::SessionClosed));
}

#[test]
fn url_and_group_validation_happen_before_transport() {
    for url in [
        "",
        "example.com",
        "https:///",
        "file:///secret",
        "https://user:secret@example.com/",
        "https://example.com/\nsecret",
        "https://example.com\\@evil.com",
        "https://example.com:99999/",
        "https://[bad]/",
    ] {
        assert_eq!(validate_url(url), Err(Error::InvalidInput), "{url:?}");
    }
    for url in [
        "https://example.com/path?x=1#form",
        "http://localhost:8080/",
        "https://[::1]/",
        "https://xn--bcher-kva.example/",
    ] {
        assert_eq!(validate_url(url), Ok(()));
    }
    let mut client = paired(Fault::None);
    assert_eq!(
        client.set_login("https://example.invalid", "", "", None),
        Err(Error::InvalidInput)
    );
    assert_eq!(
        client.set_login(
            "https://example.invalid",
            "",
            "dummy",
            Some(Group {
                name: "Test",
                uuid: "bad"
            })
        ),
        Err(Error::InvalidInput)
    );
    assert_eq!(client.transport.requests.borrow().len(), 1);
}

#[test]
fn raw_json_framing_handles_fragmentation_coalescing_and_braces_in_strings() {
    let first = br#"{"action":"test","value":"}\\\"{"}"#;
    for end in 0..first.len() {
        assert_eq!(frame_length(&first[..end]), Ok(None));
    }
    assert_eq!(frame_length(first), Ok(Some(first.len())));
    let joined = [first.as_slice(), br#"{"action":"next"}"#].concat();
    assert_eq!(frame_length(&joined), Ok(Some(first.len())));
    assert_eq!(frame_length(b"[]"), Err(Error::InvalidMessage));
    assert_eq!(frame_length(b"{bad}"), Err(Error::InvalidMessage));
    assert_eq!(
        frame_length(&vec![b' '; MAX_MESSAGE_BYTES + 1]),
        Err(Error::MessageTooLarge)
    );
}

#[test]
fn association_serialization_validates_version_length_and_key() {
    let mut client = paired(Fault::None);
    let record = client.store.load().unwrap().unwrap();
    let bytes = record.to_bytes().unwrap();
    let roundtrip = Association::from_bytes(&bytes).unwrap();
    assert_eq!(record.id_key(), roundtrip.id_key());
    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["version"] = json!(2);
    assert!(matches!(
        Association::from_bytes(&serde_json::to_vec(&value).unwrap()),
        Err(Error::Storage)
    ));
    assert!(matches!(
        Association::new(
            "hash".into(),
            Zeroizing::new("id".into()),
            Zeroizing::new([0; 32])
        ),
        Err(Error::Storage)
    ));
}

fn unhex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

#[test]
fn nacl_fixture_matches_libsodium_mac_prefix_and_ciphertext() {
    let vector: Value =
        serde_json::from_str(include_str!("../tests/fixtures/nacl-box.json")).unwrap();
    let secret = SecretKey::from_slice(&unhex(vector["secret"].as_str().unwrap())).unwrap();
    let public = PublicKey::from_slice(&unhex(vector["public"].as_str().unwrap())).unwrap();
    let nonce: [u8; 24] = unhex(vector["nonce"].as_str().unwrap()).try_into().unwrap();
    let mut plaintext = Zeroizing::new(unhex(vector["plaintext"].as_str().unwrap()));
    let original = plaintext.clone();
    let cipher = SalsaBox::new(&public, &secret);
    let encrypted = protocol::encrypt(&cipher, &nonce, &mut plaintext).unwrap();
    assert_eq!(encrypted, unhex(vector["ciphertext"].as_str().unwrap()));
    let decrypted = protocol::decrypt(&cipher, &nonce, &STANDARD.encode(&encrypted)).unwrap();
    assert_eq!(&decrypted[16..], original.as_slice());
}

#[test]
fn handshake_requires_nonce_and_valid_key() {
    let nonce = [0; 24];
    let mut session = Session::generate().unwrap();
    let no_nonce =
        json!({"action":"change-public-keys","success":"true","publicKey":STANDARD.encode([7;32])});
    assert_eq!(
        session.accept_host(&serde_json::to_vec(&no_nonce).unwrap(), &nonce),
        Err(Error::InvalidNonce)
    );
    let zero_key = json!({"action":"change-public-keys","success":"true","publicKey":STANDARD.encode([0;32]),"nonce":STANDARD.encode(increment_nonce(nonce))});
    assert_eq!(
        session.accept_host(&serde_json::to_vec(&zero_key).unwrap(), &nonce),
        Err(Error::Crypto)
    );
}

#[test]
fn handshake_rejects_low_order_and_noncanonical_peer_keys() {
    let nonce = [3; 24];
    let mut one = [0; 32];
    one[0] = 1;
    let mut modulus = [255; 32];
    modulus[0] = 237;
    modulus[31] = 127;
    let mut modulus_plus_one = modulus;
    modulus_plus_one[0] += 1;
    for key in [[0; 32], one, modulus, modulus_plus_one] {
        let mut session = Session::generate().unwrap();
        let reply = json!({"action":"change-public-keys", "success":"true",
            "publicKey":STANDARD.encode(key), "nonce":STANDARD.encode(increment_nonce(nonce))});
        assert_eq!(
            session.accept_host(&serde_json::to_vec(&reply).unwrap(), &nonce),
            Err(Error::Crypto)
        );
    }
}

#[cfg(windows)]
#[test]
fn concurrent_dpapi_publish_has_exactly_one_winner() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("association.dpapi");
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let path = path.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let association = Association::new(
                    "dummy-db".into(),
                    Zeroizing::new("dummy-id".into()),
                    Zeroizing::new([42; 32]),
                )
                .unwrap();
                barrier.wait();
                DpapiStore::new(path).save(&association)
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| **r == Err(Error::AlreadyAssociated))
            .count(),
        3
    );
    assert_eq!(
        DpapiStore::new(path).load().unwrap().unwrap().id_key(),
        &[42; 32]
    );
}

#[cfg(windows)]
#[test]
fn dpapi_file_roundtrip_is_insert_only_and_rejects_corruption() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("association.dpapi");
    let mut store = DpapiStore::new(&path);
    assert!(store.load().unwrap().is_none());
    let mut client = paired(Fault::None);
    let association = client.store.load().unwrap().unwrap();
    store.save(&association).unwrap();
    let protected = std::fs::read(&path).unwrap();
    assert!(!protected.windows(32).any(|w| w == association.id_key()));
    assert_eq!(
        store.load().unwrap().unwrap().id_key(),
        association.id_key()
    );
    assert_eq!(store.save(&association), Err(Error::AlreadyAssociated));
    assert_eq!(std::fs::read(&path).unwrap(), protected);
    std::fs::write(&path, b"corrupt").unwrap();
    assert!(matches!(store.load(), Err(Error::KeyProtection)));
    assert_eq!(store.save(&association), Err(Error::AlreadyAssociated));
    assert_eq!(std::fs::read(&path).unwrap(), b"corrupt");
}
