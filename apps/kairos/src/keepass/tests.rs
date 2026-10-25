use super::*;
use kairos_platform::PlatformError;
use std::{cell::RefCell, rc::Rc};
struct Target(Rc<RefCell<Vec<String>>>, bool);
impl InputTarget for Target {
    fn restore(&self) -> Result<(), PlatformError> {
        Ok(())
    }
    fn type_text(&self, text: &str, from_editor: bool) -> Result<(), PlatformError> {
        assert!(from_editor);
        if self.1 {
            return Err(PlatformError::TargetChanged);
        }
        self.0.borrow_mut().push(text.into());
        Ok(())
    }
}
struct MockBridge;
impl ContextSource for MockBridge {
    fn latest(&self, window: Option<BrowserWindow>) -> Option<Context> {
        window.map(|_| Context {
            url: "https://example.invalid/signup".into(),
            username: Some("test@example.invalid".into()),
        })
    }
}
struct MockVault {
    test: Result<(), Error>,
    write: Result<(), Error>,
    calls: Vec<&'static str>,
}
impl Vault for MockVault {
    fn test(&mut self) -> Result<(), Error> {
        self.calls.push("test-associate");
        self.test
    }
    fn associate(&mut self) -> Result<(), Error> {
        self.calls.push("associate");
        Ok(())
    }
    fn save(&mut self, _: &Login) -> Result<(), Error> {
        self.calls.push("set-login");
        self.write
    }
}
fn login() -> Login {
    Login {
        url: "https://example.invalid/".into(),
        username: String::new(),
        group: String::new(),
        group_uuid: String::new(),
        password: Zeroizing::new("PUBLIC TEST".into()),
    }
}
fn fixture(fail_input: bool) -> (Flow, Rc<RefCell<Vec<String>>>) {
    let typed = Rc::new(RefCell::new(Vec::new()));
    let mut flow = Flow::new(
        Some(Box::new(Target(typed.clone(), fail_input))),
        Some(&MockBridge),
        Some(BrowserWindow {
            hwnd: 1,
            process: 2,
        }),
    );
    flow.phase = Phase::Saving;
    flow.saved_secret = Some(Zeroizing::new("PUBLIC SAVED".into()));
    (flow, typed)
}
#[test]
fn bridge_context_is_a_proposal_and_missing_identity_yields_no_url() {
    let (flow, _) = fixture(false);
    assert!(flow.url.is_empty());
    assert!(!flow.confirmed);
    assert_eq!(
        flow.candidate.unwrap().username.as_deref(),
        Some("test@example.invalid")
    );
    let flow = Flow::new(None, Some(&MockBridge), None);
    assert!(flow.url.is_empty());
    assert!(flow.candidate.is_none());
}
#[test]
fn kee_passxc_rejection_and_missing_association_never_type_or_auto_pair() {
    for error in [
        Error::MissingAssociation,
        Error::Rejected(1),
        Error::Timeout,
        Error::DatabaseChanged,
    ] {
        let mut vault = MockVault {
            test: Err(error),
            write: Ok(()),
            calls: Vec::new(),
        };
        let (mut flow, typed) = fixture(false);
        flow.complete(save(&mut vault, &login()));
        assert_eq!(vault.calls, ["test-associate"]);
        assert!(flow.type_saved().is_err());
        assert!(typed.borrow().is_empty());
        assert_eq!(flow.allow_pairing, error == Error::MissingAssociation);
    }
}
#[test]
fn accepted_save_requires_fresh_click_uses_frozen_secret_and_consumes_target() {
    let mut vault = MockVault {
        test: Ok(()),
        write: Ok(()),
        calls: Vec::new(),
    };
    let (mut flow, typed) = fixture(false);
    flow.complete(save(&mut vault, &login()));
    assert_eq!(vault.calls, ["test-associate", "set-login"]);
    assert!(typed.borrow().is_empty());
    flow.type_saved().unwrap();
    assert_eq!(*typed.borrow(), ["PUBLIC SAVED"]);
    assert!(flow.type_saved().is_err());
}
#[test]
fn save_timeout_or_changed_input_target_reports_partial_outcome_without_retry() {
    let (mut flow, typed) = fixture(false);
    flow.complete(Err(Error::Timeout));
    assert!(flow.type_saved().is_err());
    assert!(typed.borrow().is_empty());
    assert!(flow.notice.contains("bereits ein Eintrag"));
    let (mut flow, typed) = fixture(true);
    flow.complete(Ok(()));
    assert_eq!(flow.type_saved(), Err(PlatformError::TargetChanged));
    assert!(flow.type_saved().is_err());
    assert!(typed.borrow().is_empty());
}
#[test]
fn explicit_association_does_not_write_or_type() {
    let mut vault = MockVault {
        test: Ok(()),
        write: Ok(()),
        calls: Vec::new(),
    };
    vault.associate().unwrap();
    let (mut flow, typed) = fixture(false);
    flow.phase = Phase::Pairing;
    flow.complete(Ok(()));
    assert!(flow.phase == Phase::Draft);
    assert!(!flow.allow_pairing);
    assert!(flow.type_saved().is_err());
    assert!(typed.borrow().is_empty());
    assert_eq!(vault.calls, ["associate"]);
}

#[test]
fn invalid_autotype_password_is_rejected_before_spawning_a_vault_request() {
    for secret in ["bad\npassword".to_owned(), "x".repeat(1025)] {
        let (mut flow, typed) = fixture(false);
        flow.phase = Phase::Draft;
        flow.saved_secret = None;
        flow.url = "https://example.invalid/".into();
        flow.confirmed = true;
        flow.submit(&secret, false, &egui::Context::default());
        assert!(flow.phase == Phase::Draft);
        assert!(flow.pending.is_none());
        assert!(flow.saved_secret.is_none());
        assert!(typed.borrow().is_empty());
    }
}

#[cfg(windows)]
#[test]
#[ignore = "requires an explicitly paired isolated browser profile and KeePassXC test database; read only"]
fn real_browser_context_and_keepassxc_association_read_only() {
    // Only explicit paths. Never changes a browser profile, creates an association
    // or writes a database. Run manually with Kairos closed (fixed bridge port).
    let bridge_path =
        std::env::var_os("KAIROS_BRIDGE_TEST_ASSOCIATION_FILE").expect("explicit bridge test path");
    let vault_path = std::env::var_os("KAIROS_KEEPASSXC_TEST_ASSOCIATION_FILE")
        .expect("explicit KeePassXC test path");
    let bridge = Bridge::start(
        Box::new(ProtectedStore(kairos_keepassxc::DpapiStore::new(
            bridge_path,
        ))),
        foreground_browser,
    )
    .unwrap();
    assert_eq!(bridge.status().unwrap(), PairingStatus::Paired);
    let until = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        if bridge.latest_for(foreground_browser()).is_some() {
            break;
        }
        assert!(
            std::time::Instant::now() < until,
            "no fresh foreground browser context"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    let transport = kairos_keepassxc::NamedPipeTransport::connect(Duration::from_secs(5)).unwrap();
    let mut client =
        kairos_keepassxc::Client::connect(transport, kairos_keepassxc::DpapiStore::new(vault_path))
            .unwrap();
    client.test_associate().unwrap();
}
