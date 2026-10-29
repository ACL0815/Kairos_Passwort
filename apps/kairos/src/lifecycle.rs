//! Window-independent commands. All callbacks queue first, then wake App::logic.
use crate::{help, KairosApp};
use eframe::egui;
#[cfg(windows)]
use kairos_platform::integration::{self, RunState, Shortcut, StartupApproval, RUN_VALUE};
use std::{path::PathBuf, sync::mpsc};

#[derive(Clone, Copy, Debug)]
pub(crate) enum AppEvent {
    Show,
    Popup,
    CopyNew,
    ToggleStartup,
    Quit,
    Refresh,
}

#[derive(Clone)]
pub(crate) struct EventSender {
    tx: mpsc::Sender<AppEvent>,
    ctx: egui::Context,
}
impl EventSender {
    pub(crate) fn send(&self, event: AppEvent) {
        if self.tx.send(event).is_ok() {
            self.ctx.request_repaint();
        }
    }
}
#[derive(Default)]
pub(crate) struct Lifecycle {
    receiver: Option<mpsc::Receiver<AppEvent>>,
    pub(crate) hidden: bool,
    quitting: bool,
    pub(crate) exiting: bool,
    pub(crate) tray_available: bool,
    show_hint: bool,
    pub(crate) error: Option<String>,
    integration_error: Option<String>,
    registry_error: bool,
    background_notice: Option<&'static str>,
    exe: Option<PathBuf>,
    shortcut: Option<PathBuf>,
    #[cfg(windows)]
    run: Option<RunState>,
    shortcut_exists: bool,
    #[cfg(windows)]
    tray: Option<crate::tray::Tray>,
    #[cfg(windows)]
    instance: Option<kairos_platform::instance::SingleInstance>,
    #[cfg(windows)]
    visibility: Option<kairos_platform::visibility::WindowVisibility>,
}

impl KairosApp {
    #[cfg(windows)]
    pub fn connect_lifecycle(
        &mut self,
        creation: &eframe::CreationContext<'_>,
        mut instance: kairos_platform::instance::SingleInstance,
        start_hidden: bool,
    ) {
        let ctx = &creation.egui_ctx;
        let (tx, receiver) = mpsc::channel();
        let sender = EventSender {
            tx,
            ctx: ctx.clone(),
        };
        self.lifecycle.receiver = Some(receiver);
        let ipc = sender.clone();
        if instance
            .listen(move |request| {
                ipc.send(match request {
                    kairos_platform::instance::InstanceRequest::Show => AppEvent::Show,
                    kairos_platform::instance::InstanceRequest::Popup => AppEvent::Popup,
                });
            })
            .is_err()
        {
            self.lifecycle.error =
                Some("Einzelinstanz-Signale nicht verfügbar. Kairos neu starten.".into());
        }
        self.lifecycle.instance = Some(instance);
        match crate::tray::Tray::new(sender) {
            Ok(tray) => {
                self.lifecycle.tray = Some(tray);
                self.lifecycle.tray_available = true;
            }
            Err(_) => {
                self.lifecycle.error = Some(
                    "Tray nicht verfügbar. Das Fenster bleibt geöffnet; Schließen beendet Kairos."
                        .into(),
                );
            }
        }
        self.lifecycle.exe = std::env::current_exe().ok();
        self.refresh_integration();
        match kairos_platform::visibility::WindowVisibility::attach(
            creation,
            start_hidden && self.lifecycle.tray_available,
        ) {
            Ok(guard) => self.lifecycle.visibility = Some(guard),
            Err(_) => {
                self.lifecycle.error =
                    Some("Verborgener Start nicht verfügbar. Kairos bleibt sichtbar.".into())
            }
        }
        if start_hidden && self.lifecycle.tray_available && self.lifecycle.visibility.is_some() {
            self.hide_window(ctx);
        } else if start_hidden {
            self.show_window(ctx);
        }
    }

    pub(crate) fn show_window(&mut self, ctx: &egui::Context) {
        self.lifecycle.hidden = false;
        #[cfg(windows)]
        if let Some(guard) = &self.lifecycle.visibility {
            guard.set_hidden(false);
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        // The existing GUI-thread CaptureProtection show hook reapplies and reads
        // back affinity before WM_SHOWWINDOW / SWP_SHOWWINDOW, including re-shows.
    }
    fn hide_window(&mut self, ctx: &egui::Context) {
        self.lifecycle.hidden = true;
        #[cfg(windows)]
        if let Some(guard) = &self.lifecycle.visibility {
            guard.set_hidden(true);
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
    }
    pub(crate) fn background_logic(&mut self, ctx: &egui::Context) {
        self.keepass_poll();
        self.poll_hotkeys(ctx);
        // Bound work per wakeup; a flooded queue cannot starve native messages.
        for _ in 0..32 {
            let Some(event) = self
                .lifecycle
                .receiver
                .as_ref()
                .and_then(|r| r.try_recv().ok())
            else {
                break;
            };
            self.lifecycle_event(ctx, event);
            ctx.request_repaint();
        }
        if self.keepass.flow.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
        }
        if self.lifecycle.quitting {
            if self.keepass.flow.as_ref().is_some_and(|f| f.busy()) {
                self.show_window(ctx);
                self.notice = "Kairos beendet sich nach Abschluss der laufenden KeePassXC-Anfrage.";
            } else {
                self.lifecycle.exiting = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        if self.lifecycle.exiting {
            return;
        }
        self.keepass_guard_close(ctx);
        if !self.lifecycle.hidden && ctx.input(|i| i.viewport().close_requested()) {
            self.close_window(ctx);
        }
    }
    pub(crate) fn lifecycle_event(&mut self, ctx: &egui::Context, event: AppEvent) {
        match event {
            AppEvent::Show => self.show_window(ctx),
            AppEvent::Popup => self.show_popup(ctx),
            AppEvent::CopyNew => {
                if self
                    .keepass
                    .flow
                    .as_ref()
                    .is_some_and(|f| f.blocks_hotkeys())
                {
                    self.show_window(ctx);
                    self.notice =
                        "KeePassXC-Auftrag zuerst abschließen. Kein neues Passwort kopiert.";
                } else {
                    self.regenerate(ctx);
                    if !self.password.secret.is_empty() {
                        self.copy(ctx, false);
                    }
                    if self.password.secret.is_empty() || !self.notice.starts_with("Kopiert.") {
                        self.show_window(ctx);
                    }
                }
            }
            AppEvent::Quit => self.lifecycle.quitting = true,
            AppEvent::ToggleStartup => self.toggle_startup(ctx),
            AppEvent::Refresh => self.refresh_integration(),
        }
    }
    fn close_window(&mut self, ctx: &egui::Context) {
        if !self.rotation_settings.desktop.close_to_tray || !self.lifecycle.tray_available {
            return;
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        if self.rotation_settings.desktop.tray_notice_seen {
            self.hide_window(ctx);
        } else {
            self.lifecycle.show_hint = true;
        }
    }
    fn save_desktop_preferences(&mut self) {
        self.rotation_settings
            .save_desktop(self.rotate_profiles, &self.rotation_selection);
    }
    #[cfg(windows)]
    fn refresh_integration(&mut self) {
        if let Some(exe) = &self.lifecycle.exe {
            match integration::read_run(RUN_VALUE, exe) {
                Ok(state) => {
                    self.lifecycle.run = Some(state);
                    self.lifecycle.registry_error = false;
                }
                Err(_) => {
                    self.lifecycle.run = None;
                    self.lifecycle.registry_error = true;
                }
            }
        }
        self.lifecycle.shortcut = integration::programs_dir()
            .and_then(|p| integration::start_menu_path(&p))
            .ok();
        self.lifecycle.shortcut_exists =
            self.lifecycle.shortcut.as_ref().is_some_and(|p| p.exists());
        if let Some(tray) = &self.lifecycle.tray {
            let checked = self
                .lifecycle
                .run
                .as_ref()
                .is_some_and(|s| s.command.is_some());
            let enabled = self.lifecycle.run.as_ref().is_some_and(|s| {
                !matches!(
                    s.approval,
                    StartupApproval::Disabled | StartupApproval::Unknown
                )
            });
            tray.set_startup(checked, enabled);
        }
    }
    #[cfg(not(windows))]
    fn refresh_integration(&mut self) {}

    #[cfg(windows)]
    fn toggle_startup(&mut self, ctx: &egui::Context) {
        self.refresh_integration();
        let Some(state) = &self.lifecycle.run else {
            self.show_window(ctx);
            return;
        };
        if matches!(
            state.approval,
            StartupApproval::Disabled | StartupApproval::Unknown
        ) {
            self.show_window(ctx);
            self.notice =
                "Autostart-Freigabe im Windows-Task-Manager prüfen. Kairos ändert sie nicht.";
            return;
        }
        // A foreign/old path needs the explicit update action in settings.
        if state.command.is_some() && !state.command_matches {
            self.show_window(ctx);
            self.notice = "Autostart verwendet einen anderen Pfad oder andere Parameter. Unter Windows und Tray prüfen und ausdrücklich Pfad aktualisieren wählen.";
            return;
        }
        let result = if state.command.is_some() {
            integration::remove_run(RUN_VALUE)
        } else {
            self.lifecycle
                .exe
                .as_ref()
                .ok_or_else(|| std::io::Error::other("no exe"))
                .and_then(|p| integration::set_run(RUN_VALUE, p))
        };
        self.integration_result(ctx, result);
    }
    #[cfg(not(windows))]
    fn toggle_startup(&mut self, _ctx: &egui::Context) {}
    #[cfg(windows)]
    fn integration_result(&mut self, ctx: &egui::Context, result: std::io::Result<()>) {
        self.lifecycle.integration_error = result
            .err()
            .map(|e| format!("Windows-Integration fehlgeschlagen: {e}"));
        self.refresh_integration();
        if self.lifecycle.integration_error.is_some() {
            self.show_window(ctx);
        }
    }
    pub(crate) fn lifecycle_view(&mut self, ui: &mut egui::Ui) {
        if let Some(error) = &self.lifecycle.error {
            ui.colored_label(egui::Color32::YELLOW, error);
        }
        if let Some(error) = &self.lifecycle.integration_error {
            ui.colored_label(egui::Color32::YELLOW, error);
        }
        if self.lifecycle.registry_error {
            ui.colored_label(
                egui::Color32::YELLOW,
                "Autostart-Status konnte nicht gelesen werden.",
            );
        }
        if let Some(notice) = self.lifecycle.background_notice {
            ui.colored_label(egui::Color32::YELLOW, notice);
        }
        let response = egui::CollapsingHeader::new("Windows und Tray").show(ui, |ui| {
            if !self.lifecycle.tray_available { ui.label("Tray in diesem Lauf nicht verfügbar."); }
            ui.horizontal_wrapped(|ui| {
                if ui.checkbox(&mut self.rotation_settings.desktop.close_to_tray, "Beim Schließen in den Tray").changed() { self.save_desktop_preferences(); }
                help::info(ui, "Beim Schließen in den Tray", "Das Fenster-X verbirgt Kairos. Hotkeys und geschütztes Kopieren laufen weiter. Öffnen per Tray-Klick, Beenden im Tray-Menü. Ohne Haken beendet das Fenster-X die App. Die Auswahl wird mit den übrigen Einstellungen gespeichert.");
            });
            if let Some(notice) = self.rotation_settings.notice { ui.label(notice); }
            #[cfg(windows)]
            self.integration_controls(ui);
        });
        if response.header_response.clicked() {
            self.refresh_integration();
            ui.ctx().request_repaint();
        }
    }
    pub(crate) fn lifecycle_hint(&mut self, ctx: &egui::Context) {
        if self.lifecycle.show_hint {
            egui::Window::new("Kairos läuft im Tray weiter").collapsible(false).resizable(false).show(ctx, |ui| {
                ui.label("Kairos bleibt rechts unten im Infobereich aktiv (gegebenenfalls unter ^). Klick auf das K öffnet das Fenster. F9/F10/F11 bleiben aktiv. Beenden über das Tray-Menü.");
                if ui.button("Verstanden – in den Tray").clicked() {
                    self.rotation_settings.desktop.tray_notice_seen = true;
                    self.save_desktop_preferences();
                    self.lifecycle.show_hint = false;
                    self.hide_window(ui.ctx());
                }
                if ui.button("Fenster offen lassen").clicked() { self.lifecycle.show_hint = false; }
            });
        }
    }
    pub(crate) fn set_background_notice(&mut self, message: Option<&'static str>) {
        self.lifecycle.background_notice = message;
        #[cfg(windows)]
        if let Some(tray) = &self.lifecycle.tray {
            tray.set_status(message);
        }
    }
    #[cfg(windows)]
    fn integration_controls(&mut self, ui: &mut egui::Ui) {
        if ui.small_button("Status aktualisieren").clicked() {
            self.refresh_integration();
        }
        let ctx = ui.ctx().clone();
        if let Some(state) = &self.lifecycle.run {
            let mut enabled = state.command.is_some();
            let blocked = matches!(
                state.approval,
                StartupApproval::Disabled | StartupApproval::Unknown
            );
            let mismatch = enabled && !state.command_matches;
            let path_mismatch = !state.path_matches;
            let old_command = state.command.clone();
            ui.horizontal_wrapped(|ui| {
                if ui.add_enabled(!blocked && !mismatch, egui::Checkbox::new(&mut enabled, "Mit Windows starten")).changed() { self.toggle_startup(&ctx); }
                help::info(ui, "Mit Windows starten", "Autostart für dieses Windows-Konto, ohne Administratorrechte: HKCU Run → Kairos Passwort. Startet mit --tray unsichtbar. Zustand kommt aus der Registry. Ein anderer EXE-Pfad benötigt ausdrücklich Pfad aktualisieren. Eine Task-Manager-Sperre wird nur angezeigt; dort wieder aktivieren. Kairos verändert StartupApproved niemals.");
            });
            if blocked {
                ui.label("Im Windows-Task-Manager deaktiviert oder Freigabestatus unbekannt. Dort prüfen; Kairos überschreibt die Freigabe nicht.");
            }
            if mismatch {
                ui.label(if path_mismatch {
                    "Autostart zeigt auf einen anderen EXE-Pfad."
                } else {
                    "Autostart verwendet andere Startparameter."
                });
                if let Some(command) = old_command {
                    ui.label(command);
                }
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(!blocked, egui::Button::new("Pfad aktualisieren"))
                        .clicked()
                    {
                        let result = integration::set_run(
                            RUN_VALUE,
                            self.lifecycle.exe.as_ref().expect("read with exe"),
                        );
                        self.integration_result(&ctx, result);
                    }
                    if ui
                        .add_enabled(!blocked, egui::Button::new("Autostart entfernen"))
                        .clicked()
                    {
                        self.integration_result(&ctx, integration::remove_run(RUN_VALUE));
                    }
                });
            }
        }
        let mut shortcut = self.lifecycle.shortcut_exists;
        ui.horizontal_wrapped(|ui| {
            if ui.add_enabled(self.lifecycle.shortcut.is_some() && self.lifecycle.exe.is_some(), egui::Checkbox::new(&mut shortcut, "Startmenü-Eintrag")).changed() {
                let path = self.lifecycle.shortcut.as_ref().expect("enabled with path");
                let result = if shortcut { Shortcut::create(path, self.lifecycle.exe.as_ref().expect("enabled with exe")) } else { Shortcut::remove(path) };
                self.integration_result(&ctx, result);
            }
            help::info(ui, "Startmenü-Eintrag", "Erstellt oder entfernt ausschließlich Kairos Passwort.lnk im persönlichen Windows-Startmenü. Ziel, Arbeitsverzeichnis und Icon verwenden diese EXE. Eine vorhandene fremde Verknüpfung wird nicht überschrieben. Nach dem Verschieben der EXE den eigenen Eintrag entfernen und neu anlegen. --remove-integration entfernt Autostart und den eigenen Startmenü-Eintrag ohne Fenster.");
        });
        if self.lifecycle.shortcut.is_none() {
            ui.label("Persönlicher Startmenü-Pfad nicht verfügbar.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    struct PublicSource;
    impl kairos_core::WordSource for PublicSource {
        fn len(&self) -> usize {
            1
        }
        fn word(&self, _: usize) -> &str {
            "ahorn"
        }
        fn language(&self, _: usize) -> kairos_core::Language {
            kairos_core::Language::German
        }
    }
    struct Clipboard(Rc<RefCell<Vec<String>>>);
    impl crate::clipboard::ClipboardAccess for Clipboard {
        fn copy(&self, text: &str) -> Result<(), ()> {
            self.0.borrow_mut().push(text.to_owned());
            Ok(())
        }
        fn paste(&self) -> Result<zeroize::Zeroizing<String>, ()> {
            Err(())
        }
    }
    fn fixture(ctx: &egui::Context) -> (KairosApp, EventSender, Rc<RefCell<Vec<String>>>) {
        let copies = Rc::new(RefCell::new(vec![]));
        let mut app = KairosApp::with_clipboard(
            ctx,
            Box::new(PublicSource),
            false,
            Box::new(Clipboard(copies.clone())),
        );
        let (tx, receiver) = mpsc::channel();
        app.lifecycle.receiver = Some(receiver);
        app.lifecycle.tray_available = true;
        (
            app,
            EventSender {
                tx,
                ctx: ctx.clone(),
            },
            copies,
        )
    }
    #[test]
    fn f10_failure_has_a_tray_status_without_showing_or_focusing_the_window() {
        let ctx = egui::Context::default();
        let (mut app, _, copied) = fixture(&ctx);
        app.lifecycle.hidden = true;
        let output = ctx.run_logic(&egui::RawInput::default(), |ctx| {
            app.handle_action(
                ctx,
                kairos_platform::Action::GenerateTypeCopy,
                Err(kairos_platform::PlatformError::TargetUnavailable),
            );
        });
        assert!(app.lifecycle.background_notice.is_some());
        assert!(copied.borrow().is_empty() && app.lifecycle.hidden);
        assert!(output
            .viewport_commands
            .values()
            .flatten()
            .all(|cmd| !matches!(
                cmd,
                egui::ViewportCommand::Visible(true) | egui::ViewportCommand::Focus
            )));
    }
    #[cfg(windows)]
    #[test]
    fn successful_integration_does_not_erase_a_tray_or_ipc_failure() {
        let ctx = egui::Context::default();
        let (mut app, _, _) = fixture(&ctx);
        app.lifecycle.error = Some("PUBLIC TRAY FAILURE".into());
        app.lifecycle.integration_error = Some("PUBLIC WRITE FAILURE".into());
        app.integration_result(&ctx, Ok(()));
        assert_eq!(app.lifecycle.error.as_deref(), Some("PUBLIC TRAY FAILURE"));
        assert!(app.lifecycle.integration_error.is_none());
    }
    #[test]
    fn hidden_logic_handles_tray_copy_show_popup_and_quit_without_drawing() {
        let ctx = egui::Context::default();
        let (mut app, sender, copied) = fixture(&ctx);
        let input = egui::RawInput::default();
        app.lifecycle.hidden = true;
        sender.send(AppEvent::CopyNew);
        let _ = ctx.run_logic(&input, |ctx| app.background_logic(ctx));
        assert_eq!(*copied.borrow(), ["ahorn-ahorn-ahorn-ahorn"]);
        assert!(app.lifecycle.hidden, "copy must not steal foreground focus");
        sender.send(AppEvent::Show);
        let _ = ctx.run_logic(&input, |ctx| app.background_logic(ctx));
        assert!(!app.lifecycle.hidden);
        app.lifecycle.hidden = true;
        sender.send(AppEvent::Popup);
        let _ = ctx.run_logic(&input, |ctx| app.background_logic(ctx));
        assert!(!app.lifecycle.hidden && app.compact);
        sender.send(AppEvent::Quit);
        let _ = ctx.run_logic(&input, |ctx| app.background_logic(ctx));
        assert!(
            app.lifecycle.exiting,
            "explicit quit bypasses close-to-tray"
        );
    }
    #[test]
    fn close_to_tray_requires_an_available_tray_and_hint_is_once_per_settings() {
        let ctx = egui::Context::default();
        let (mut app, _, _) = fixture(&ctx);
        app.close_window(&ctx);
        assert!(app.lifecycle.show_hint);
        assert!(!app.lifecycle.hidden);
        app.lifecycle.show_hint = false;
        app.rotation_settings.desktop.tray_notice_seen = true;
        app.close_window(&ctx);
        assert!(app.lifecycle.hidden);
        app.lifecycle.hidden = false;
        app.rotation_settings.desktop.close_to_tray = false;
        app.close_window(&ctx);
        assert!(!app.lifecycle.hidden);
        app.rotation_settings.desktop.close_to_tray = true;
        app.lifecycle.tray_available = false;
        app.close_window(&ctx);
        assert!(!app.lifecycle.hidden);
    }

    #[test]
    fn hotkey_actions_execute_in_hidden_logic_f10_without_showing_f9_f11_show() {
        struct Target(Rc<RefCell<Vec<String>>>);
        impl crate::desktop::InputTarget for Target {
            fn restore(&self) -> Result<(), kairos_platform::PlatformError> {
                Ok(())
            }
            fn type_text(
                &self,
                text: &str,
                from_editor: bool,
            ) -> Result<(), kairos_platform::PlatformError> {
                assert!(!from_editor);
                self.0.borrow_mut().push(text.to_owned());
                Ok(())
            }
        }
        let ctx = egui::Context::default();
        let (mut app, _, copied) = fixture(&ctx);
        let typed = Rc::new(RefCell::new(vec![]));
        app.lifecycle.hidden = true;
        let output = ctx.run_logic(&egui::RawInput::default(), |ctx| {
            app.handle_action(
                ctx,
                kairos_platform::Action::GenerateTypeCopy,
                Ok(Box::new(Target(typed.clone()))),
            );
            app.background_logic(ctx);
        });
        assert_eq!(*typed.borrow(), *copied.borrow());
        assert_eq!(typed.borrow().len(), 1);
        assert!(app.lifecycle.hidden);
        assert!(output
            .viewport_commands
            .values()
            .flatten()
            .all(|cmd| !matches!(
                cmd,
                egui::ViewportCommand::Visible(true)
                    | egui::ViewportCommand::Focus
                    | egui::ViewportCommand::Minimized(false)
            )));
        for action in [
            kairos_platform::Action::OpenEditor,
            kairos_platform::Action::GenerateTypeKeePassXC,
        ] {
            app.lifecycle.hidden = true;
            let _ = ctx.run_logic(&egui::RawInput::default(), |ctx| {
                app.handle_action(
                    ctx,
                    action,
                    Err(kairos_platform::PlatformError::TargetUnavailable),
                );
                app.background_logic(ctx);
            });
            assert!(!app.lifecycle.hidden && app.compact);
        }
        assert_eq!(copied.borrow().len(), 1, "F9/F11 never copy automatically");
    }
}
