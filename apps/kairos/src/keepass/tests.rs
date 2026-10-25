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

fn draw(ctx: &egui::Context, app: &mut KairosApp, events: Vec<egui::Event>) -> egui::FullOutput {
    let mut output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 1200.0),
            )),
            events,
            ..Default::default()
        },
        |ui| app.draw(ui),
    );
    output.textures_delta.clear();
    output
}

#[test]
fn editing_any_confirmed_login_field_requires_new_confirmation() {
    let ctx = egui::Context::default();
    let mut app = KairosApp::new(&ctx, crate::german_source(), false);
    let (mut flow, _) = fixture(false);
    flow.phase = Phase::Draft;
    app.keepass.flow = Some(flow);
    let _ = draw(&ctx, &mut app, vec![]);
    for id in [
        egui::Id::unique("keepass_url"),
        egui::Id::unique("keepass_username"),
        egui::Id::unique("keepass_group"),
        egui::Id::unique("keepass_uuid"),
        egui::Id::unique("password"),
    ] {
        app.keepass.flow.as_mut().unwrap().confirmed = true;
        ctx.memory_mut(|memory| memory.request_focus(id));
        let _ = draw(&ctx, &mut app, vec![egui::Event::Text("x".into())]);
        assert!(!app.keepass.flow.as_ref().unwrap().confirmed);
    }
}

#[test]
fn saving_and_accepted_orders_block_new_hotkeys_and_preserve_password() {
    let ctx = egui::Context::default();
    let mut app = KairosApp::new(&ctx, crate::german_source(), false);
    let original = Zeroizing::new(app.password.secret.to_string());
    for phase in [Phase::Saving, Phase::Accepted] {
        let (mut flow, typed) = fixture(false);
        flow.phase = phase;
        app.keepass.flow = Some(flow);
        app.handle_action(
            &ctx,
            kairos_platform::Action::GenerateTypeCopy,
            Err(PlatformError::TargetUnavailable),
        );
        assert_eq!(app.password.secret.as_str(), original.as_str());
        assert!(app.keepass.flow.as_ref().unwrap().phase == phase);
        assert!(typed.borrow().is_empty());
    }
}

#[test]
fn close_request_is_cancelled_only_while_a_vault_operation_is_running() {
    for phase in [
        Phase::Saving,
        Phase::Pairing,
        Phase::Accepted,
        Phase::Finished,
    ] {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), false);
        let (mut flow, _) = fixture(false);
        flow.phase = phase;
        app.keepass.flow = Some(flow);
        let mut input = egui::RawInput::default();
        input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .events
            .push(egui::ViewportEvent::Close);
        let mut output = ctx.run_ui(input, |ui| app.keepass_guard_close(ui.ctx()));
        output.textures_delta.clear();
        let cancelled = output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|command| matches!(command, egui::ViewportCommand::CancelClose));
        assert_eq!(cancelled, matches!(phase, Phase::Saving | Phase::Pairing));
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
