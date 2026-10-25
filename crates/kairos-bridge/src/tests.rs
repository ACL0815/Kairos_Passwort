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
fn pair(state: &mut State) -> Zeroizing<String> {
    let key = state.begin_pairing().unwrap();
    let client_key = Zeroizing::new(URL_SAFE_NO_PAD.encode([42u8; 32]));
    let raw = format!(
        r#"{{"timestamp":{},"nonce":"{}","client_token":"{}"}}"#,
        unix_seconds(),
        "f".repeat(32),
        client_key.as_str()
    );
    state
        .handle("/v1/pair", &key, raw.as_bytes(), None)
        .unwrap();
    assert_ne!(key.as_str(), client_key.as_str());
    assert_eq!(
        state.handle("/v1/context", &key, &message(100), window()),
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
        state.handle("/v1/context", "bad", &message(1), window()),
        Err(Error::Unauthorized)
    );
    let key = pair(&mut state);
    assert_eq!(state.status(), PairingStatus::Paired);
    assert_eq!(state.begin_pairing().err(), Some(Error::AlreadyPaired));
    assert_eq!(
        state.handle("/v1/pair", &key, &message(2), None),
        Err(Error::AlreadyPaired)
    );
}
#[test]
fn invalid_token_replay_timestamp_and_unknown_fields_are_rejected() {
    let mut state = fixture();
    let key = pair(&mut state);
    let raw = message(1);
    assert_eq!(
        state.handle("/v1/context", &"x".repeat(43), &raw, window()),
        Err(Error::Unauthorized)
    );
    state.handle("/v1/context", &key, &raw, window()).unwrap();
    assert_eq!(
        state.handle("/v1/context", &key, &raw, window()),
        Err(Error::Replay)
    );
    let old = String::from_utf8(message(2))
        .unwrap()
        .replace(&unix_seconds().to_string(), "1");
    assert_eq!(
        state.handle("/v1/context", &key, old.as_bytes(), window()),
        Err(Error::Expired)
    );
    assert_eq!(
        state.handle("/v1/context", &key, br#"{"password":"never"}"#, window()),
        Err(Error::Invalid)
    );
}
#[test]
fn contexts_need_fresh_matching_native_browser_and_clear_on_invalid_data() {
    let mut state = fixture();
    let key = pair(&mut state);
    state
        .handle("/v1/context", &key, &message(1), window())
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
    assert_eq!(
        state.handle("/v1/context", &key, bad.as_bytes(), window()),
        Err(Error::Invalid)
    );
    assert!(state.context(window()).is_none());
}
#[test]
fn pairing_cancel_expiry_attempt_limit_and_storage_failure_fail_closed() {
    let mut state = fixture();
    let key = state.begin_pairing().unwrap();
    assert_eq!(state.status(), PairingStatus::Pending);
    state.pending.as_mut().unwrap().until = Instant::now() - Duration::from_secs(1);
    assert_eq!(
        state.handle("/v1/pair", &key, b"{}", None),
        Err(Error::Expired)
    );
    state.begin_pairing().unwrap();
    for _ in 0..5 {
        assert_eq!(
            state.handle("/v1/pair", "wrong", b"{}", None),
            Err(Error::Unauthorized)
        );
    }
    assert_eq!(
        state.handle("/v1/pair", "wrong", b"{}", None),
        Err(Error::Expired)
    );
    state.store = Box::new(Memory {
        fail: true,
        saved: Arc::new(AtomicBool::new(false)),
    });
    let key = state.begin_pairing().unwrap();
    let raw = format!(
        r#"{{"timestamp":{},"nonce":"{}","client_token":"{}"}}"#,
        unix_seconds(),
        "a".repeat(32),
        URL_SAFE_NO_PAD.encode([42u8; 32])
    );
    assert_eq!(
        state.handle("/v1/pair", &key, raw.as_bytes(), None),
        Err(Error::Storage)
    );
    assert!(state.token.is_none());
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
