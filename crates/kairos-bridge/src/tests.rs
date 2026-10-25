use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
struct Memory {
    fail: bool,
    saved: Arc<AtomicBool>,
}
impl SecretStore for Memory {
    fn load(&mut self) -> Result<Option<Zeroizing<String>>, Error> {
        Ok(None)
    }
    fn save(&mut self, _: &str) -> Result<(), Error> {
        if self.fail {
            Err(Error::Storage)
        } else {
            self.saved.store(true, Ordering::SeqCst);
            Ok(())
        }
    }
}
fn fixture() -> State {
    State::new(Box::new(Memory {
        fail: false,
        saved: Arc::new(AtomicBool::new(false)),
    }))
    .unwrap()
}
fn message(n: u8) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({"timestamp":unix_seconds(), "nonce":format!("{n:032x}"), "url":"https://example.invalid/signup?token=secret#fragment"})).unwrap()
}
pub(crate) fn signature(secret: &str, path: &str, host: &str, body: &[u8]) -> String {
    let Ok(message) = serde_json::from_slice::<Message>(body) else {
        return "0".repeat(64);
    };
    let mut hmac = mac(secret).unwrap();
    hmac.update(canonical(path, host, body, &message).as_bytes());
    format!("{:x}", hmac.finalize().into_bytes())
}
impl State {
    fn signed(
        &mut self,
        path: &str,
        secret: &str,
        body: &[u8],
        window: Option<BrowserWindow>,
    ) -> Result<(), Error> {
        self.handle(
            path,
            HOST,
            &signature(secret, path, HOST, body),
            body,
            window,
        )
    }
}
fn pair(state: &mut State) -> Zeroizing<String> {
    let code = state.begin_pairing().unwrap();
    let client_key = derive_client_key(&code).unwrap();
    let raw = format!(
        r#"{{"timestamp":{},"nonce":"{}"}}"#,
        unix_seconds(),
        "f".repeat(32)
    );
    state
        .signed("/v2/pair", &code, raw.as_bytes(), None)
        .unwrap();
    assert_ne!(code.as_str(), client_key.as_str());
    assert_eq!(
        state.signed("/v2/context", &code, &message(100), window()),
        Err(Error::Unauthorized)
    );
    client_key
}
fn window() -> Option<BrowserWindow> {
    Some(BrowserWindow {
        hwnd: 10,
        process: 20,
    })
}
#[test]
fn pairing_is_explicit_one_time_and_persisted_before_use() {
    let mut state = fixture();
    assert_eq!(state.status(), PairingStatus::Unpaired);
    assert_eq!(
        state.signed(
            "/v2/context",
            &URL_SAFE_NO_PAD.encode([0u8; 32]),
            &message(1),
            window()
        ),
        Err(Error::Unauthorized)
    );
    let key = pair(&mut state);
    assert_eq!(state.status(), PairingStatus::Paired);
    assert_eq!(state.begin_pairing().err(), Some(Error::AlreadyPaired));
    assert_eq!(
        state.signed("/v2/pair", &key, &message(2), None),
        Err(Error::AlreadyPaired)
    );
}
#[test]
fn invalid_hmac_replay_timestamp_and_unknown_fields_are_rejected() {
    let mut state = fixture();
    let key = pair(&mut state);
    let raw = message(1);
    assert_eq!(
        state.signed(
            "/v2/context",
            &URL_SAFE_NO_PAD.encode([1u8; 32]),
            &raw,
            window()
        ),
        Err(Error::Unauthorized)
    );
    state.signed("/v2/context", &key, &raw, window()).unwrap();
    assert_eq!(
        state.signed("/v2/context", &key, &raw, window()),
        Err(Error::Replay)
    );
    let old = String::from_utf8(message(2))
        .unwrap()
        .replace(&unix_seconds().to_string(), "1");
    assert_eq!(
        state.signed("/v2/context", &key, old.as_bytes(), window()),
        Err(Error::Expired)
    );
    assert_eq!(
        state.signed("/v2/context", &key, br#"{"password":"never"}"#, window()),
        Err(Error::Invalid)
    );
}
#[test]
fn contexts_need_fresh_matching_native_browser_and_clear_on_invalid_data() {
    let mut state = fixture();
    let key = pair(&mut state);
    state
        .signed("/v2/context", &key, &message(1), window())
        .unwrap();
    assert_eq!(
        state.context(window()).unwrap().url,
        "https://example.invalid/signup"
    );
    assert!(state.context(None).is_none());
    assert!(state
        .context(Some(BrowserWindow {
            hwnd: 11,
            process: 20
        }))
        .is_none());
    state.active.as_mut().unwrap().received = Instant::now() - Duration::from_secs(6);
    assert!(state.context(window()).is_none());
    let bad = String::from_utf8(message(2)).unwrap().replace(
        "https://example.invalid/signup?token=secret#fragment",
        "file:///secret",
    );
    state.last_context = None;
    assert_eq!(
        state.signed("/v2/context", &key, bad.as_bytes(), window()),
        Err(Error::Invalid)
    );
    assert!(state.context(window()).is_none());
}

#[test]
fn context_flood_cannot_fill_nonce_cache_and_probe_recovers_uncertain_pairing() {
    let mut state = fixture();
    let key = pair(&mut state);
    state
        .signed("/v2/context", &key, &message(1), window())
        .unwrap();
    for n in 2..=200 {
        assert_eq!(
            state.signed("/v2/context", &key, &message(n), window()),
            Err(Error::RateLimited)
        );
    }
    assert_eq!(state.nonces.len(), 2);
    let probe = format!(
        r#"{{"timestamp":{},"nonce":"{}"}}"#,
        unix_seconds(),
        "c".repeat(32)
    );
    state
        .signed("/v2/probe", &key, probe.as_bytes(), None)
        .unwrap();
    assert_eq!(state.status(), PairingStatus::Paired);
    assert!(state.context(window()).is_some());
}

#[test]
fn explicit_username_survives_same_url_heartbeats_without_renewing_consent() {
    let mut state = fixture();
    let key = pair(&mut state);
    let username = String::from_utf8(message(1)).unwrap().replacen(
        '{',
        "{\"username\":\"user@example.invalid\",",
        1,
    );
    state
        .signed("/v2/context", &key, username.as_bytes(), window())
        .unwrap();
    let deadline = state.active.as_ref().unwrap().username_until;
    state.last_context = None;
    state
        .signed("/v2/context", &key, &message(2), window())
        .unwrap();
    assert_eq!(
        state.context(window()).unwrap().username.as_deref(),
        Some("user@example.invalid")
    );
    assert_eq!(state.active.as_ref().unwrap().username_until, deadline);
    state.active.as_mut().unwrap().username_until = Some(Instant::now() - Duration::from_secs(1));
    state.last_context = None;
    state
        .signed("/v2/context", &key, &message(3), window())
        .unwrap();
    assert!(state.context(window()).unwrap().username.is_none());
}
#[test]
fn pairing_cancel_expiry_attempt_limit_and_storage_failure_fail_closed() {
    let mut state = fixture();
    let key = state.begin_pairing().unwrap();
    assert_eq!(state.status(), PairingStatus::Pending);
    state.pending.as_mut().unwrap().until = Instant::now() - Duration::from_secs(1);
    assert_eq!(
        state.signed("/v2/pair", &key, &message(10), None),
        Err(Error::Expired)
    );
    state.begin_pairing().unwrap();
    for _ in 0..5 {
        assert_eq!(
            state.signed(
                "/v2/pair",
                &URL_SAFE_NO_PAD.encode([0u8; 32]),
                &message(10),
                None
            ),
            Err(Error::Unauthorized)
        );
    }
    assert_eq!(
        state.signed(
            "/v2/pair",
            &URL_SAFE_NO_PAD.encode([0u8; 32]),
            &message(10),
            None
        ),
        Err(Error::Expired)
    );
    state.store = Box::new(Memory {
        fail: true,
        saved: Arc::new(AtomicBool::new(false)),
    });
    let key = state.begin_pairing().unwrap();
    let raw = format!(
        r#"{{"timestamp":{},"nonce":"{}"}}"#,
        unix_seconds(),
        "a".repeat(32)
    );
    assert_eq!(
        state.signed("/v2/pair", &key, raw.as_bytes(), None),
        Err(Error::Storage)
    );
    assert!(state.secret.is_none());
    state.pending = None;
    assert_eq!(state.status(), PairingStatus::Unpaired);
}
#[test]
fn url_normalization_keeps_path_and_rejects_credentials() {
    assert_eq!(
        normalize_url("https://example.invalid/a%20b?secret#x"),
        Ok("https://example.invalid/a%20b".into())
    );
    for url in [
        "https://user:pw@example.invalid/",
        "javascript:alert(1)",
        "https://example.invalid/\n",
        "https://exa%mple.invalid/",
        "https://example.invalid\\evil/",
        "example.invalid",
    ] {
        assert_eq!(normalize_url(url), Err(Error::Invalid));
    }
}

#[test]
fn shared_webcrypto_vector_matches_rust_canonical_hmac_and_key_derivation() {
    let vector: serde_json::Value =
        serde_json::from_str(include_str!("../../../browser/hmac-vector.json")).unwrap();
    let secret = vector["secret"].as_str().unwrap();
    let body = vector["body"].as_str().unwrap().as_bytes();
    let message: Message = serde_json::from_slice(body).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(body)), vector["body_sha256"]);
    assert_eq!(
        canonical("/v2/context", HOST, body, &message),
        vector["canonical"]
    );
    assert_eq!(signature(secret, "/v2/context", HOST, body), vector["hmac"]);
    assert_eq!(*derive_client_key(secret).unwrap(), vector["client_key"]);
    verify_hmac(
        secret,
        vector["hmac"].as_str().unwrap(),
        vector["canonical"].as_str().unwrap(),
    )
    .unwrap();
}

#[test]
fn hmac_binds_body_path_host_timestamp_nonce_and_rejects_missing_or_wrong_tags() {
    let mut state = fixture();
    let key = pair(&mut state);
    let body = message(1);
    let tag = signature(&key, "/v2/context", HOST, &body);
    let text = String::from_utf8(body.clone()).unwrap();
    let altered_body = text.replace("signup", "signin");
    let timestamp = serde_json::from_slice::<Message>(&body).unwrap().timestamp;
    let altered_timestamp = text.replace(&timestamp.to_string(), &(timestamp + 1).to_string());
    let altered_nonce = text.replace(&format!("{:032x}", 1), &format!("{:032x}", 2));
    for (path, host, signature, bytes) in [
        ("/v2/context", HOST, "", body.as_slice()),
        (
            "/v2/context",
            HOST,
            "0000000000000000000000000000000000000000000000000000000000000000",
            body.as_slice(),
        ),
        ("/v2/context", HOST, tag.as_str(), altered_body.as_bytes()),
        (
            "/v2/context",
            HOST,
            tag.as_str(),
            altered_timestamp.as_bytes(),
        ),
        ("/v2/context", HOST, tag.as_str(), altered_nonce.as_bytes()),
        ("/v2/clear", HOST, tag.as_str(), body.as_slice()),
        (
            "/v2/context",
            "localhost:19471",
            tag.as_str(),
            body.as_slice(),
        ),
    ] {
        assert!(state
            .handle(path, host, signature, bytes, window())
            .is_err());
        assert!(state.context(window()).is_none());
    }
    state
        .handle("/v2/context", HOST, &tag, &body, window())
        .unwrap();
    assert_eq!(
        state.handle("/v2/context", HOST, &tag, &body, window()),
        Err(Error::Replay)
    );
}
#[cfg(windows)]
#[test]
fn dpapi_roundtrip_uses_only_an_isolated_file_and_never_overwrites() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("test-bridge.dpapi");
    let mut store = ProtectedStore(DpapiStore::new(&path));
    assert!(store.load().unwrap().is_none());
    let key = URL_SAFE_NO_PAD.encode([42u8; 32]);
    store.save(&key).unwrap();
    assert_eq!(&**store.load().unwrap().unwrap(), key);
    assert!(!std::fs::read(&path)
        .unwrap()
        .windows(43)
        .any(|s| s == key.as_bytes()));
    assert_eq!(store.save(&key), Err(Error::Storage));
    std::fs::write(&path, b"corrupt").unwrap();
    assert_eq!(store.load().err(), Some(Error::Storage));
}
