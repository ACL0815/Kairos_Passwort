//! Kairos native application, consuming only the stable core contract.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples
//! Modelled after: Password Tech https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/main

#[cfg(all(feature = "demo", not(debug_assertions)))]
compile_error!("The fake-data demo feature must never be used in a release build.");

mod blocked;
mod capture;
mod clipboard;
mod desktop;
mod help;
mod keepass;
mod lifecycle;
mod model;
mod profile_view;
mod rotation;
mod security;
mod source;
mod strength;
mod theme;
#[cfg(windows)]
mod tray;

pub use source::german_source;

pub fn icon_data() -> egui::IconData {
    egui::IconData {
        rgba: include_bytes!("../assets/kairos-64.rgba").to_vec(),
        width: 64,
        height: 64,
    }
}

#[cfg(all(feature = "demo", debug_assertions))]
mod demo;

use clipboard::{ClipboardAccess, ProtectedClipboard};
use desktop::{boxed_target, input_error, Destination};
use eframe::egui::{self, Color32, Key, Modifiers, RichText};
use kairos_core::generate;
#[cfg(test)]
use kairos_core::ProfileId;
use kairos_core::{GeneratorConfig, WordSource};
use kairos_platform::{Action, FocusTarget, GlobalHotkeys, HotkeyBinding, PlatformError};
use model::{
    apply_exclusions, password_layout, Assessment, PasswordState, MAX_SECRET_CHARS, SEPARATORS,
};

pub struct KairosApp {
    source: Box<dyn WordSource>,
    expanded_source: Box<dyn WordSource>,
    source_counts: [usize; 2],
    german_only: bool,
    blocked: blocked::BlockedWords,
    show_blocked: bool,
    blocked_input: String,
    blocked_notice: &'static str,
    hardening: Option<kairos_hardening::HardeningReport>,
    show_security: bool,
    config: GeneratorConfig,
    excluded: [bool; 20],
    password: PasswordState,
    clipboard: Box<dyn ClipboardAccess>,
    notice: &'static str,
    compact: bool,
    show_licenses: bool,
    target: Destination,
    hotkeys: Option<GlobalHotkeys>,
    hotkey_error: Option<&'static str>,
    rotate_profiles: bool,
    rotation_selection: rotation::Selection,
    rotation_settings: rotation::SettingsFile,
    rotation_bonus: Option<f64>,
    profile_bits: [Option<f64>; 13],
    show_profiles: bool,
    help_profile: u8,
    copy_on_insert: bool,
    capture_protection: Option<kairos_platform::CaptureProtection>,
    keepass: keepass::Integration,
    lifecycle: lifecycle::Lifecycle,
}

impl KairosApp {
    /// `source` is used for profile 0 and the German-only override.
    /// Profiles 1–12 otherwise use the embedded combined DE/EN/LA source.
    pub fn new(ctx: &egui::Context, source: Box<dyn WordSource>, compact: bool) -> Self {
        Self::with_clipboard(ctx, source, compact, Box::<ProtectedClipboard>::default())
    }

    fn with_clipboard(
        ctx: &egui::Context,
        source: Box<dyn WordSource>,
        compact: bool,
        clipboard: Box<dyn ClipboardAccess>,
    ) -> Self {
        theme::install(ctx);
        let expanded_source = source::combined_source();
        let source_counts = [source.len(), expanded_source.len()];
        let mut app = Self {
            source,
            expanded_source,
            source_counts,
            german_only: false,
            blocked: blocked::BlockedWords::default(),
            show_blocked: false,
            blocked_input: String::new(),
            blocked_notice: "",
            hardening: None,
            show_security: false,
            config: GeneratorConfig::default(),
            excluded: [false; 20],
            password: PasswordState::default(),
            clipboard,
            notice: "",
            compact,
            show_licenses: false,
            target: Destination::Unavailable,
            hotkeys: None,
            hotkey_error: None,
            rotate_profiles: false,
            rotation_selection: rotation::Selection::default(),
            rotation_settings: rotation::SettingsFile::default(),
            rotation_bonus: None,
            profile_bits: [None; 13],
            show_profiles: false,
            help_profile: 0,
            copy_on_insert: true,
            capture_protection: None,
            keepass: keepass::Integration::default(),
            lifecycle: lifecycle::Lifecycle::default(),
        };
        #[cfg(all(feature = "demo", debug_assertions))]
        {
            demo::configure(&mut app);
        }
        app.refresh_profile_bits();
        app.regenerate(ctx);
        app
    }

    /// Keeps the GUI-thread hook alive for all subsequent native viewports and dialogs.
    pub fn connect_capture_protection(&mut self, protection: kairos_platform::CaptureProtection) {
        self.capture_protection = Some(protection);
    }

    /// Attaches the report produced before eframe and reloads explicit exclusions.
    pub fn connect_startup(
        &mut self,
        ctx: &egui::Context,
        report: kairos_hardening::HardeningReport,
    ) {
        self.hardening = Some(report);
        self.load_rotation_settings(rotation::SettingsFile::configured());
        self.blocked = blocked::BlockedWords::configured();
        self.blocked_settings_changed(ctx);
    }

    fn load_rotation_settings(&mut self, settings: rotation::SettingsFile) {
        self.rotation_settings = settings;
        (self.rotate_profiles, self.rotation_selection) = self.rotation_settings.load();
    }

    /// Connects OS services after the destination was captured before run_native.
    ///
    /// Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/egui.rs>.
    pub fn connect_desktop(
        &mut self,
        ctx: &egui::Context,
        target: Result<FocusTarget, PlatformError>,
    ) {
        self.target =
            boxed_target(target).map_or(Destination::Unavailable, Destination::ReturnOnly);
        if !self.target.can_restore() {
            self.notice = "Kein externes Zielfeld gemerkt. Generieren und Kopieren sind verfügbar; für Einfügen F9 im gewünschten Feld auslösen.";
        }
        self.register_hotkeys(ctx);
        self.start_bridge();
    }

    fn register_hotkeys(&mut self, ctx: &egui::Context) {
        if cfg!(feature = "demo") {
            return;
        }
        let wakeup = ctx.clone();
        match GlobalHotkeys::with_wakeup(&HotkeyBinding::defaults(), move || wakeup.request_repaint()) {
            Ok(hotkeys) => {
                self.hotkeys = Some(hotkeys);
                self.hotkey_error = None;
            }
            Err(_) => self.hotkey_error = Some("Globale Hotkeys konnten nicht registriert werden. Möglicherweise belegt eine andere Anwendung Strg+Umschalt+F9/F10/F11. Dort freigeben und erneut versuchen."),
        }
    }

    fn poll_hotkeys(&mut self, ctx: &egui::Context) {
        let result = self.hotkeys.as_ref().map(GlobalHotkeys::try_recv);
        match result {
            Some(Ok(Some(event))) => {
                self.handle_action(ctx, event.action, boxed_target(event.target));
                ctx.request_repaint();
            }
            Some(Err(_)) => {
                self.hotkeys = None;
                self.hotkey_error =
                    Some("Der Hotkey-Dienst ist ausgefallen. Bitte erneut registrieren.");
            }
            _ => {}
        }
    }

    fn handle_action(
        &mut self,
        ctx: &egui::Context,
        action: Action,
        target: Result<Box<dyn desktop::InputTarget>, PlatformError>,
    ) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title("Kairos Passwort".into()));
        if self
            .keepass
            .flow
            .as_ref()
            .is_some_and(|flow| flow.blocks_hotkeys())
        {
            self.notice =
                "KeePassXC-Auftrag zuerst im offenen Dialog abschließen oder nach dem Ergebnis schließen.";
            if action == Action::GenerateTypeCopy {
                self.set_background_notice(Some(
                    "F10: KeePassXC-Auftrag zuerst abschließen. Kairos öffnen.",
                ));
            }
            return;
        }
        self.keepass.flow = None;
        match action {
            Action::OpenEditor => {
                self.target = target.map_or(Destination::Unavailable, Destination::Editor);
                self.regenerate(ctx);
                if !self.target.can_insert() {
                    self.notice = "Kein externes Zielfeld gemerkt. Generieren und Kopieren sind verfügbar; für Einfügen F9 im gewünschten Feld auslösen.";
                }
                self.show_popup(ctx);
            }
            Action::GenerateTypeCopy => {
                self.set_background_notice(None);
                // A background action must not retain a previous editor destination.
                self.target = Destination::Unavailable;
                self.regenerate(ctx);
                if self.password.secret.is_empty() {
                    self.set_background_notice(Some(
                        "F10: Generierung fehlgeschlagen. Kairos öffnen.",
                    ));
                    self.notice = self.password.message;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                        "Kairos Passwort · Generierung fehlgeschlagen".into(),
                    ));
                    ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                        egui::UserAttentionType::Informational,
                    ));
                    return;
                }
                match target.and_then(|target| target.type_text(&self.password.secret, false)) {
                    Ok(()) => {
                        if !self.copy(ctx, false) {
                            self.set_background_notice(Some(
                                "F10: Eingabe erfolgt, Kopieren fehlgeschlagen. Kairos öffnen.",
                            ));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                                "Kairos Passwort · Kopieren fehlgeschlagen, Eingabe erfolgt".into(),
                            ));
                            ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                                egui::UserAttentionType::Informational,
                            ));
                            self.notice = "Eingabe an Windows übergeben, aber Kopieren fehlgeschlagen. Zielfeld prüfen; Hotkey nicht zum Kopieren wiederholen.";
                        }
                    }
                    Err(error) => {
                        self.set_background_notice(Some(
                            "F10: Eingabe abgebrochen. Kairos öffnen.",
                        ));
                        self.notice = input_error(error);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                            "Kairos Passwort · Eingabe abgebrochen".into(),
                        ));
                        ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                            egui::UserAttentionType::Informational,
                        ));
                    }
                }
            }
            Action::GenerateTypeKeePassXC => {
                self.target = Destination::Unavailable;
                let window = keepass::foreground_browser();
                self.regenerate(ctx);
                self.keepass.flow = Some(keepass::Flow::new(
                    target.ok(),
                    self.keepass
                        .bridge
                        .as_ref()
                        .map(|bridge| bridge as &dyn keepass::ContextSource),
                    window,
                ));
                self.show_popup(ctx);
                self.notice = "KeePassXC: URL und Zielfeld ausdrücklich bestätigen.";
            }
        }
    }

    /// Shows the same editor without replacing the destination captured by the caller.
    pub fn show_popup(&mut self, ctx: &egui::Context) {
        self.compact = true;
        self.show_window(ctx);
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(720.0, 640.0)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
    }

    fn regenerate(&mut self, ctx: &egui::Context) {
        self.rotation_bonus = None;
        if self.blocked.error.is_some() {
            self.password.clear();
            self.password.message =
                "Sperrliste nicht verfügbar. Datei im Sperrlisten-Dialog prüfen und neu laden.";
            self.notice = "";
            ctx.data_mut(|data| {
                data.remove::<egui::text_edit::TextEditState>(egui::Id::unique("password"))
            });
            return;
        }
        let german = source::FilteredSource::new(self.source.as_ref(), &self.blocked.words);
        let expanded =
            source::FilteredSource::new(self.expanded_source.as_ref(), &self.blocked.words);
        self.source_counts = [german.len(), expanded.len()];
        let selected: &dyn WordSource =
            if source::uses_german(self.config.profile, self.german_only) {
                &german
            } else {
                &expanded
            };
        let result = if self.rotate_profiles {
            rotation::generate(
                &self.rotation_selection,
                &german,
                &expanded,
                self.german_only,
                &self.config.excluded_chars,
            )
            .map(|result| {
                self.rotation_bonus = Some(result.rotation_bonus_bits);
                result.password
            })
        } else {
            generate(selected, &self.config)
        };
        #[cfg(all(feature = "demo", debug_assertions))]
        let result = result.map(|result| {
            let profile = if self.rotate_profiles {
                self.rotation_selection.ids()[0]
            } else {
                self.config.profile.get()
            };
            demo::public_example(result, profile)
        });
        self.password.set_result(result);
        self.notice = "";
        ctx.data_mut(|data| {
            data.remove::<egui::text_edit::TextEditState>(egui::Id::unique("password"))
        });
    }

    fn copy(&mut self, ctx: &egui::Context, return_to_previous: bool) -> bool {
        match self.clipboard.copy(&self.password.secret) {
            Ok(()) => {
                self.notice =
                    "Kopiert. Zwischenablage wird nach 30 Sekunden geleert, sofern unverändert.";
                if return_to_previous {
                    match self.target.restore() {
                        Ok(()) => {
                            ctx.memory_mut(|memory| memory.surrender_focus(egui::Id::unique("password")));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                        Err(_) => self.notice = "Kopiert, aber Fokus-Rücksprung fehlgeschlagen. Bitte Zielfenster selbst aktivieren.",
                    }
                }
                true
            }
            Err(()) => {
                self.notice = "Sicheres Kopieren fehlgeschlagen. Bitte erneut versuchen.";
                false
            }
        }
    }

    fn insert(&mut self, ctx: &egui::Context) {
        let Some(target) = self.target.take_editor() else {
            self.notice = "Kein mit F9 gewähltes Einfügeziel. Passwort bleibt erhalten; Kopieren nutzen oder F9 im Ziel auslösen (erzeugt neu).";
            return;
        };
        match target.type_text(&self.password.secret, true) {
            Ok(()) => {
                self.notice = "Eingabe an Windows übergeben. Bitte im Zielfeld prüfen.";
                if self.copy_on_insert && !self.copy(ctx, false) {
                    self.notice = "Eingabe an Windows übergeben, aber Kopieren fehlgeschlagen. Zielfeld prüfen; nur Kopieren erneut versuchen.";
                    return;
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            Err(error) => self.notice = input_error(error),
        }
    }

    fn editor(&mut self, ui: &mut egui::Ui) -> bool {
        let id = egui::Id::unique("password");
        let size = if self.compact { 23.0 } else { 27.0 };
        let segments = &self.password.segments;
        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, width: f32| {
            let mut job = password_layout(text.as_str(), segments, size);
            job.wrap.max_width = width;
            ui.fonts_mut(|fonts| fonts.layout_job(job))
        };
        let mut output = egui::TextEdit::singleline(&mut *self.password.secret)
            .id(id)
            .password(true)
            .clip_text(false)
            .layouter(&mut layouter)
            .font(egui::FontId::monospace(size))
            .desired_width(f32::INFINITY)
            .desired_rows(1)
            .hint_text("Hier erscheint dein Wortpasswort")
            .char_limit(MAX_SECRET_CHARS)
            .show(ui);
        // egui feeds String snapshots even for password fields. Clear every frame;
        // its internal temporary allocations cannot be reliably zeroized by us.
        output.state.clear_undoer();
        output.state.store(ui.ctx(), id);
        if output.response.changed() {
            self.password.edited();
        }
        output.response.changed()
    }

    fn exclusions(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;
        egui::CollapsingHeader::new("Sonderzeichen ausschließen")
            .default_open(false)
            .show(ui, |ui| {
                help::info(ui, "Sonderzeichen ausschließen", help::EXCLUSIONS);
                ui.label(
                    RichText::new(
                        "Markierte Zeichen werden bei der nächsten Generierung ausgelassen.",
                    )
                    .color(theme::MUTED)
                    .size(14.0),
                );
                ui.horizontal_wrapped(|ui| {
                    for (index, character) in SEPARATORS.iter().enumerate() {
                        changed |= ui
                            .toggle_value(
                                &mut self.excluded[index],
                                RichText::new(character.to_string()).monospace().size(18.0),
                            )
                            .changed();
                    }
                });
                if ui.small_button("Alle wieder erlauben").clicked() {
                    self.excluded.fill(false);
                    changed = true;
                }
            });
        if changed {
            apply_exclusions(&mut self.config, &self.excluded);
            self.settings_changed(ui.ctx());
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.painter()
            .rect_filled(ui.max_rect(), 0, theme::BACKGROUND);
        capture::banner(
            ui,
            self.capture_protection.as_ref().map_or(
                kairos_platform::CaptureStatus::Pending,
                kairos_platform::CaptureProtection::status,
            ),
            self.capture_protection.as_ref().map_or(
                kairos_platform::CapturePolicy::Protect,
                kairos_platform::CaptureProtection::policy,
            ),
        );
        if let Some(error) = self.hotkey_error {
            ui.colored_label(
                model::segment_color(kairos_core::SegmentKind::Separator),
                error,
            );
            if ui.button("Hotkeys erneut registrieren").clicked() {
                self.register_hotkeys(ui.ctx());
            }
        }
        theme::panel()
            .corner_radius(10)
            .inner_margin(egui::Margin::symmetric(14, 8))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    let status = security::summary(self.hardening.as_ref());
                    if self
                        .hardening
                        .as_ref()
                        .is_some_and(|report| report.all_applied())
                    {
                        ui.label(status);
                    } else {
                        ui.colored_label(Color32::from_rgb(251, 191, 36), status);
                    }
                    if ui.small_button("Sicherheitsreport").clicked() {
                        self.show_security = true;
                    }
                    help::info(ui, "Härtungsreport", help::HARDENING);
                    ui.label(
                        RichText::new("Bildschirmschutz")
                            .size(14.0)
                            .color(theme::MUTED),
                    );
                    help::info(ui, "Bildschirmaufnahmeschutz", help::CAPTURE);
                });
            });
        egui::Window::new("Sicherheitsreport")
            .open(&mut self.show_security)
            .vscroll(true)
            .show(ui.ctx(), |ui| security::view(ui, self.hardening.as_ref()));
        self.keepass_poll();
        self.lifecycle_hint(ui.ctx());
        if self.keepass.flow.is_some() {
            egui::ScrollArea::vertical().show(ui, |ui| self.keepass_view(ui));
            return;
        }
        let ctx = ui.ctx().clone();
        let paste = ctx.memory(|memory| memory.has_focus(egui::Id::unique("password")))
            && ctx.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::V));
        if paste {
            match self.clipboard.paste() {
                Ok(text) => ctx.input_mut(|input| {
                    input
                        .events
                        .retain(|event| !matches!(event, egui::Event::Paste(_)));
                    input.events.push(egui::Event::Paste(text.to_string()));
                }),
                Err(()) => self.notice = "Die Zwischenablage enthält keinen verfügbaren Text.",
            }
        }
        ctx.input_mut(|input| {
            for event in &mut input.events {
                if let egui::Event::Paste(text) | egui::Event::Text(text) = event {
                    text.retain(|character| !character.is_control());
                }
            }
        });
        let regenerate = ctx.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::R));
        let copy = ctx.input_mut(|input| {
            let copy = input
                .events
                .iter()
                .any(|event| matches!(event, egui::Event::Copy));
            input
                .events
                .retain(|event| !matches!(event, egui::Event::Copy | egui::Event::Cut));
            copy || input.consume_key(Modifiers::CTRL, Key::C)
        });
        if regenerate {
            self.regenerate(&ctx);
        }
        if copy && !self.password.secret.is_empty() {
            self.copy(&ctx, false);
        }
        egui::CentralPanel::default().frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(if self.compact { 18 } else { 28 })).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.compact {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("KAIROS").color(theme::CYAN).size(15.0).strong());
                        ui.label(RichText::new("Dein nächstes Passwort.").size(22.0).strong());
                    });
                } else {
                    ui.label(RichText::new("KAIROS").color(theme::CYAN).size(15.0).strong());
                    ui.heading("Dein nächstes Passwort.");
                }
                if !self.compact { ui.label(RichText::new("Leicht abzuschreiben. Sicher per Zufall.").color(theme::MUTED)); }
                #[cfg(all(feature = "demo", debug_assertions))]
                ui.colored_label(Color32::from_rgb(251, 191, 36), "DEMO · Öffentliche Beispieldaten, kein sicheres Passwort");
                ui.add_space(10.0);
                theme::panel().inner_margin(if self.compact { 14 } else { 22 }).show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 7.0;
                    self.controls(ui);
                    ui.add_space(8.0);
                    self.editor(ui);
                    if self.compact { self.strength_card(ui); }
                    ui.horizontal_wrapped(|ui| {
                        for (label, kind) in [("Wort", kairos_core::SegmentKind::Word), ("123", kairos_core::SegmentKind::Digit), ("#+", kairos_core::SegmentKind::Separator), ("A", kairos_core::SegmentKind::Uppercase)] {
                            ui.label(RichText::new(label).color(model::segment_color(kind)).monospace().size(13.0));
                        }
                        ui.label(RichText::new("· direkt bearbeitbar · ohne Undo-Verlauf").color(theme::MUTED).size(12.0));
                        help::info(ui, "Editor und Generierung", help::EDITOR);
                    });
                    if !self.compact || !matches!(self.password.assessment, Assessment::Generated(_)) {
                        ui.label(RichText::new(self.password.message).color(theme::MUTED).size(14.0));
                    }
                });
                ui.add_space(8.0);
                if self.target.can_insert() {
                    ui.label(RichText::new("Einfügen nutzt das vor dem Öffnen gemerkte Fenster. Windows kann Browser-Feldwechsel im selben Fenster nicht erkennen. Zielfeld vorher prüfen; keine automatische Wiederholung.").color(theme::MUTED).size(12.0));
                    ui.checkbox(&mut self.copy_on_insert, "Beim Einfügen auch kopieren (30 s)");
                } else {
                    ui.label(RichText::new("Einfügen mit F9 im gewünschten Zielfeld freigeben. Das Startfenster dient nur dem Fokus-Rücksprung.").color(theme::MUTED).size(12.0));
                }
                ui.horizontal_wrapped(|ui| {
                    if ui.add(egui::Button::new(RichText::new("Neu generieren").strong()).fill(Color32::from_rgb(45, 79, 132))).on_hover_text("Strg+R").clicked() { self.regenerate(&ctx); }
                    let available = !self.password.secret.is_empty();
                    if ui.add_enabled(available, egui::Button::new("Kopieren")).on_hover_text("Strg+C kopiert das ganze Passwort").clicked() { self.copy(&ctx, false); }
                    if ui.add_enabled(available && self.target.can_restore(), egui::Button::new("Kopieren und zurückspringen")).on_disabled_hover_text("Für einen Rücksprung muss zuerst ein externes Fenster gemerkt sein.").clicked() { self.copy(&ctx, true); }
                    if ui.add_enabled(available && self.target.can_insert(), egui::Button::new("Einfügen")).on_disabled_hover_text("F9 im gewünschten Zielfeld auslösen. Beim App-Start wird kein Einfügeziel freigegeben.").clicked() { self.insert(&ctx); }
                    help::info(ui, "Kopieren und Zurückspringen", help::CLIPBOARD);
                });
                if !self.notice.is_empty() { ui.label(RichText::new(self.notice).color(theme::CYAN).size(14.0)); }
                if !self.compact { self.strength_card(ui); }
                self.exclusions(ui);
                ui.horizontal_wrapped(|ui| {
                    if ui.button(format!("Sperrliste verwalten ({})", self.blocked.words.len())).clicked() { self.show_blocked = true; }
                    help::info(ui, "Sperrliste", help::BLOCKLIST);
                    if self.blocked.error.is_some() { ui.colored_label(Color32::from_rgb(251, 191, 36), "Sperrliste prüfen"); }
                });
                self.bridge_controls(ui);
                self.lifecycle_view(ui);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Strg+Umschalt+F9: Editor · F10: Tippen + Kopieren · F11: KeePassXC-Eintrag").color(theme::MUTED).size(12.0));
                    help::info(ui, "Hotkeys F9/F10/F11", help::HOTKEYS);
                });
                if !self.compact {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Kein Passwortverlauf · Kein Cloud-Sync · Zwischenablage 30 s").color(theme::MUTED).size(12.0));
                        if ui.small_button(RichText::new("Lizenzen und Wortherkunft").size(12.0)).clicked() { self.show_licenses = true; }
                    });
                }
            });
        });
        egui::Window::new("Lizenzen und Wortherkunft")
            .open(&mut self.show_licenses)
            .vscroll(true)
            .show(&ctx, |ui| {
                for (title, license) in [
                    (
                        "Englische und lateinische Wortdaten · Herkunft",
                        include_str!(
                            "../../../crates/kairos-wordlists/data/languages/PROVENANCE.md"
                        ),
                    ),
                    (
                        "EN/LA-Datenlizenzen und Hinweise",
                        include_str!("../../../crates/kairos-wordlists/data/languages/NOTICE.txt"),
                    ),
                    (
                        "Deutsche Wortliste · Herkunft und Änderungen",
                        include_str!("../../../crates/kairos-wordlists/data/PROVENANCE.md"),
                    ),
                    (
                        "Wortdaten dys2p · CC0-1.0 (gewählte Lizenz)",
                        include_str!(
                            "../../../crates/kairos-wordlists/data/upstream/dys2p/LICENSE"
                        ),
                    ),
                    (
                        "Wortdaten enz/german-wordlist · CC0-1.0",
                        include_str!("../../../crates/kairos-wordlists/data/upstream/enz/COPYING"),
                    ),
                    (
                        "JetBrains Mono NL · SIL OFL 1.1",
                        include_str!("../assets/fonts/OFL.txt"),
                    ),
                    (
                        "Ubuntu Light · Ubuntu Font Licence 1.0",
                        include_str!("../assets/fonts/egui-defaults/UFL.txt"),
                    ),
                    (
                        "Hack · MIT / Bitstream Vera",
                        include_str!("../assets/fonts/egui-defaults/Hack-Regular.txt"),
                    ),
                    (
                        "egui-Icons · MIT",
                        include_str!(
                            "../assets/fonts/egui-defaults/emoji-icon-font-mit-license.txt"
                        ),
                    ),
                ] {
                    egui::CollapsingHeader::new(title).show(ui, |ui| {
                        ui.label(license);
                    });
                }
            });
        self.blocked_dialog(&ctx);
        self.profiles_dialog(&ctx);
        #[cfg(all(feature = "demo", debug_assertions))]
        demo::capture(&ctx);
    }

    fn blocked_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_blocked;
        egui::Window::new("Eigene Sperrliste").open(&mut open).vscroll(true).show(ctx, |ui| {
            ui.label("Gespeichert werden ausschließlich ausdrücklich gesperrte Wörter, keine Passwörter. Die Textdatei ist unverschlüsselt und kann persönliche Vorlieben verraten.");
            ui.label("Wörter bereits verwendeter Passwörter nicht sperren: Die Datei verrät dann Passwortbestandteile.");
            if let Some(path) = &self.blocked.path { ui.label(path.display().to_string()); }
            if let Some(error) = self.blocked.error { ui.colored_label(Color32::from_rgb(251, 191, 36), error); }
            if ui.button("Datei neu laden").clicked() {
                self.reload_blocked(ctx);
            }
            ui.label("Wort des aktuellen, unbearbeiteten Passworts sperren:");
            let mut change = None;
            ui.add_enabled_ui(self.blocked.error.is_none(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    for word in self.password.block_candidates() {
                        if ui.button(word.as_str()).on_hover_text("Wort sperren").clicked() { change = Some((word.to_string(), true)); }
                    }
                });
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.blocked_input).char_limit(9).hint_text("Ein Wort (a–z, 3–9)"));
                    if ui.button("Wort sperren").clicked() { change = Some((self.blocked_input.clone(), true)); }
                });
                ui.separator();
                for word in &self.blocked.words {
                    ui.horizontal(|ui| {
                        ui.label(word);
                        if ui.small_button("Entfernen").clicked() { change = Some((word.clone(), false)); }
                    });
                }
            });
            if let Some((word, blocked)) = change {
                self.change_blocked(ctx, &word, blocked);
            }
            ui.label(self.blocked_notice);
        });
        self.show_blocked = open;
    }

    fn reload_blocked(&mut self, ctx: &egui::Context) {
        self.blocked = self.blocked.path.clone().map_or_else(
            blocked::BlockedWords::configured,
            blocked::BlockedWords::load,
        );
        self.blocked_settings_changed(ctx);
    }

    fn change_blocked(&mut self, ctx: &egui::Context, word: &str, blocked: bool) {
        match self.blocked.change(word, blocked) {
            Ok(()) => {
                self.blocked_input.clear();
                self.blocked_notice = if self.password.assessment == Assessment::Edited {
                    "Sperrliste gespeichert. Nacharbeit bleibt erhalten; Ausschlüsse gelten beim nächsten Generieren."
                } else {
                    "Sperrliste gespeichert. Wortauswahl aktualisiert."
                };
                self.blocked_settings_changed(ctx);
            }
            Err(_) => self.blocked_notice = "Speichern fehlgeschlagen oder ungültiges Wort. Bisherige Sperrliste bleibt aktiv; 3–9 Buchstaben a–z und Dateizugriff prüfen.",
        }
    }

    fn blocked_settings_changed(&mut self, ctx: &egui::Context) {
        self.source_counts = [
            source::FilteredSource::new(self.source.as_ref(), &self.blocked.words).len(),
            source::FilteredSource::new(self.expanded_source.as_ref(), &self.blocked.words).len(),
        ];
        if self.blocked.error.is_some() {
            self.refresh_profile_bits();
            self.regenerate(ctx);
        } else {
            self.settings_changed(ctx);
        }
    }

    fn settings_changed(&mut self, ctx: &egui::Context) {
        self.refresh_profile_bits();
        if self.password.assessment == Assessment::Edited {
            self.notice = "Deine Nacharbeit bleibt erhalten. Neue Einstellungen gelten beim nächsten Neu generieren.";
        } else {
            self.regenerate(ctx);
        }
    }
}

impl eframe::App for KairosApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.background_logic(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "demo"))]
    #[test]
    fn app_profile_source_override_and_persisted_filter_change_real_generation() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, german_source(), false);
        assert!(app.password.assessment == Assessment::Generated(4.0 * 6907_f64.log2()));
        app.config.profile = ProfileId::new(1).unwrap();
        app.regenerate(&ctx);
        assert!(app.password.assessment == Assessment::Generated(4.0 * 20697_f64.log2()));
        app.german_only = true;
        app.regenerate(&ctx);
        assert!(app.password.assessment == Assessment::Generated(4.0 * 6907_f64.log2()));
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("blocked.txt");
        let blocked = (0..app.source.len() - 1)
            .map(|index| app.source.word(index))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&path, blocked).unwrap();
        app.blocked = blocked::BlockedWords::load(path);
        app.regenerate(&ctx);
        let retained = app.source.word(app.source.len() - 1);
        assert!(app.password.secret.split('-').all(|word| word == retained));
        assert!(app.password.assessment == Assessment::Generated(0.0));
        app.rotate_profiles = true;
        app.regenerate(&ctx);
        assert!(app.password.assessment == Assessment::Generated(0.0));
        assert_eq!(app.rotation_bonus, Some(0.0));
        let _ = frame(&ctx, &mut app, vec![]);
        app.blocked.error = Some("PUBLIC BLOCKLIST FAILURE");
        app.regenerate(&ctx);
        assert!(app.password.secret.is_empty());
        assert!(matches!(app.password.assessment, Assessment::Unavailable));
        assert!(egui::TextEdit::load_state(&ctx, egui::Id::unique("password")).is_none());
    }

    #[test]
    fn blocking_offers_dictionary_words_without_digits_only_before_editing() {
        let mut state = PasswordState::default();
        state.set_result(Ok(kairos_core::GeneratedPassword {
            secret: zeroize::Zeroizing::new("Aho7rn++wIEse".into()),
            segments: vec![],
            entropy_bits: 12.0,
        }));
        assert_eq!(
            state
                .block_candidates()
                .iter()
                .map(|word| word.as_str())
                .collect::<Vec<_>>(),
            ["ahorn", "wiese"]
        );
        state.edited();
        assert!(state.block_candidates().is_empty());
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn blocked_word_actions_persist_and_regenerate_unedited_results() {
        let ctx = egui::Context::default();
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("blocked.txt");
        let mut app = KairosApp::new(&ctx, german_source(), false);
        app.blocked = blocked::BlockedWords::load(path.clone());
        let word = app.password.block_candidates()[0].to_string();
        app.change_blocked(&ctx, &word, true);
        assert_eq!(std::fs::read_to_string(path).unwrap(), format!("{word}\n"));
        assert!(app.password.secret.split('-').all(|part| part != word));
        assert_eq!(app.source_counts, [6906, 20696]);
        assert!(app.password.assessment == Assessment::Generated(4.0 * 6906_f64.log2()));
        app.show_blocked = true;
        let _ = frame(&ctx, &mut app, vec![]);
        let output = frame(&ctx, &mut app, vec![]);
        assert!(output
            .shapes
            .iter()
            .any(|shape| has_text(&shape.shape, "Wörter bereits verwendeter Passwörter")));
    }

    #[test]
    fn blocked_settings_preserve_edits_but_invalid_reload_clears_them() {
        let ctx = egui::Context::default();
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("blocked.txt");
        let mut app = KairosApp::new(&ctx, german_source(), false);
        app.blocked = blocked::BlockedWords::load(path.clone());
        app.password.secret.clear();
        app.password.secret.push_str("PUBLIC MANUAL FIXTURE");
        app.password.edited();
        app.change_blocked(&ctx, "ahorn", true);
        assert_eq!(&**app.password.secret, "PUBLIC MANUAL FIXTURE");
        assert!(app.password.assessment == Assessment::Edited);
        assert_eq!(app.source_counts, [6906, 20696]);
        app.change_blocked(&ctx, "ahorn", false);
        assert_eq!(&**app.password.secret, "PUBLIC MANUAL FIXTURE");
        assert_eq!(app.source_counts, [6907, 20697]);
        std::fs::write(&path, "wiese\n").unwrap();
        app.reload_blocked(&ctx);
        assert_eq!(&**app.password.secret, "PUBLIC MANUAL FIXTURE");
        assert!(app.password.assessment == Assessment::Edited);
        assert!(app.notice.contains("bleibt erhalten"));
        std::fs::write(path, "invalid-word\n").unwrap();
        app.reload_blocked(&ctx);
        assert!(app.password.secret.is_empty());
        assert!(app.password.assessment == Assessment::Unavailable);
        assert!(app.blocked.error.is_some());
    }

    #[test]
    fn hardening_failure_summary_is_rendered_without_opening_report() {
        let ctx = egui::Context::default();
        let (mut app, _) = fixture_app(&ctx);
        app.hardening = Some(kairos_hardening::HardeningReport {
            measures: vec![(
                kairos_hardening::Measure::ImageLoads,
                kairos_hardening::Outcome::Failed(kairos_hardening::OsError::Win32(5)),
            )],
        });
        let output = frame(&ctx, &mut app, vec![]);
        assert!(output
            .shapes
            .iter()
            .any(|shape| has_text(&shape.shape, "Prozesshärtung unvollständig")));
    }
    use crate::desktop::InputTarget;
    use kairos_core::{GeneratedPassword, Language};
    use std::{cell::RefCell, rc::Rc};
    use zeroize::Zeroizing;

    #[derive(Default)]
    struct FakeClipboardState {
        copied: Vec<String>,
        paste: String,
        paste_calls: usize,
        fail_copy: bool,
    }

    struct FakeClipboard(Rc<RefCell<FakeClipboardState>>);

    #[derive(Default)]
    struct FakeTargetState {
        restores: usize,
        typed: Vec<(String, bool)>,
        error: Option<PlatformError>,
    }

    struct FakeTarget(Rc<RefCell<FakeTargetState>>);

    impl InputTarget for FakeTarget {
        fn restore(&self) -> Result<(), PlatformError> {
            let mut state = self.0.borrow_mut();
            state.restores += 1;
            state.error.map_or(Ok(()), Err)
        }
        fn type_text(&self, text: &str, from_editor: bool) -> Result<(), PlatformError> {
            let mut state = self.0.borrow_mut();
            if let Some(error) = state.error {
                return Err(error);
            }
            state.typed.push((text.into(), from_editor));
            Ok(())
        }
    }

    impl ClipboardAccess for FakeClipboard {
        fn copy(&self, text: &str) -> Result<(), ()> {
            let mut state = self.0.borrow_mut();
            if state.fail_copy {
                return Err(());
            }
            state.copied.push(text.into());
            Ok(())
        }
        fn paste(&self) -> Result<Zeroizing<String>, ()> {
            let mut state = self.0.borrow_mut();
            state.paste_calls += 1;
            Ok(Zeroizing::new(state.paste.clone()))
        }
    }

    fn fixture_app(ctx: &egui::Context) -> (KairosApp, Rc<RefCell<FakeClipboardState>>) {
        let clipboard = Rc::new(RefCell::new(FakeClipboardState::default()));
        let mut app = KairosApp::with_clipboard(
            ctx,
            Box::new(EmptySource),
            false,
            Box::new(FakeClipboard(clipboard.clone())),
        );
        app.password.set_result(Ok(GeneratedPassword {
            secret: Zeroizing::new("Öl-7Wiese".into()),
            segments: vec![],
            entropy_bits: 70.0,
        }));
        (app, clipboard)
    }

    fn key(key: Key, modifiers: Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            modifiers,
            physical_key: None,
            pressed: true,
            repeat: false,
        }
    }

    struct EmptySource;
    impl WordSource for EmptySource {
        fn len(&self) -> usize {
            0
        }
        fn word(&self, _index: usize) -> &str {
            unreachable!()
        }
        fn language(&self, _index: usize) -> Language {
            unreachable!()
        }
    }

    fn frame(
        ctx: &egui::Context,
        app: &mut KairosApp,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(860.0, 820.0),
                )),
                events,
                ..Default::default()
            },
            |ui| app.draw(ui),
        );
        output.textures_delta.clear();
        output
    }

    fn has_text(shape: &egui::epaint::Shape, expected: &str) -> bool {
        match shape {
            egui::epaint::Shape::Text(text) => text.galley.text().contains(expected),
            egui::epaint::Shape::Vec(shapes) => {
                shapes.iter().any(|shape| has_text(shape, expected))
            }
            _ => false,
        }
    }

    #[test]
    fn real_editor_invalidates_entropy_and_drops_undo_every_frame() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, Box::new(EmptySource), false);
        app.password.set_result(Ok(GeneratedPassword {
            secret: Zeroizing::new("Öl-7Wiese".into()),
            segments: vec![],
            entropy_bits: 70.0,
        }));
        let _ = frame(&ctx, &mut app, vec![]);
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let output = frame(&ctx, &mut app, vec![egui::Event::Text("x".into())]);
        assert!(matches!(app.password.assessment, Assessment::Edited));
        let state =
            egui::TextEdit::load_state(&ctx, egui::Id::unique("password")).expect("editor state");
        assert!(!state.undoer().is_in_flux());
        let before_undo = Zeroizing::new(app.password.secret.to_string());
        let _ = frame(
            &ctx,
            &mut app,
            vec![egui::Event::Key {
                key: Key::Z,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::CTRL,
            }],
        );
        assert_eq!(*app.password.secret, *before_undo);
        assert!(!output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(command, egui::OutputCommand::CopyText(_))));
    }

    #[test]
    fn popup_restores_a_minimized_window_and_remains_editable() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, Box::new(EmptySource), false);
        app.show_popup(&ctx);
        assert!(app.compact);
        let output = frame(&ctx, &mut app, vec![]);
        let viewport = &output.viewport_output[&egui::ViewportId::ROOT];
        assert!(viewport
            .commands
            .iter()
            .any(|command| matches!(command, egui::ViewportCommand::Minimized(false))));
        assert!(egui::TextEdit::load_state(&ctx, egui::Id::unique("password")).is_some());
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn empty_source_error_can_render_full_and_compact() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, Box::new(EmptySource), false);
        assert!(app.password.secret.is_empty());
        assert!(app.password.message.contains("Noch keine Wörter verfügbar"));
        let _ = frame(&ctx, &mut app, vec![]);
        app.compact = true;
        let _ = frame(&ctx, &mut app, vec![]);
    }

    #[test]
    fn every_valid_profile_is_named() {
        for value in 0..=12 {
            assert!(
                !help::PROFILES[ProfileId::new(value).unwrap().get() as usize]
                    .name
                    .is_empty()
            );
        }
    }

    #[test]
    fn copy_shortcut_uses_the_entire_secret_without_egui_clipboard_export() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        let output = frame(&ctx, &mut app, vec![key(Key::C, Modifiers::CTRL)]);
        assert_eq!(clipboard.borrow().copied, ["Öl-7Wiese"]);
        assert!(!output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(command, egui::OutputCommand::CopyText(_))));
    }

    #[test]
    fn copy_and_return_requires_both_copy_and_real_restore_success() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        let target = Rc::new(RefCell::new(FakeTargetState::default()));
        app.target = Destination::Editor(Box::new(FakeTarget(target.clone())));
        clipboard.borrow_mut().fail_copy = true;
        app.copy(&ctx, true);
        let output = frame(&ctx, &mut app, vec![]);
        assert!(!output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|command| matches!(command, egui::ViewportCommand::Minimized(true))));
        assert!(app.notice.contains("fehlgeschlagen"));
        assert_eq!(target.borrow().restores, 0);
        clipboard.borrow_mut().fail_copy = false;
        app.copy(&ctx, true);
        let output = frame(&ctx, &mut app, vec![]);
        assert!(output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|command| matches!(command, egui::ViewportCommand::Minimized(true))));
        assert_eq!(clipboard.borrow().copied, ["Öl-7Wiese"]);
        assert_eq!(target.borrow().restores, 1);
        target.borrow_mut().error = Some(PlatformError::TargetChanged);
        app.copy(&ctx, true);
        let output = frame(&ctx, &mut app, vec![]);
        assert!(!output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|command| matches!(command, egui::ViewportCommand::Minimized(true))));
        assert!(app.notice.contains("Fokus-Rücksprung fehlgeschlagen"));
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn default_app_generates_from_the_real_embedded_german_source() {
        let ctx = egui::Context::default();
        let app = KairosApp::new(&ctx, german_source(), false);
        let words = kairos_wordlists::german();
        assert_eq!(app.source.len(), words.len());
        assert_eq!(app.password.secret.split('-').count(), 4);
        for word in app.password.secret.split('-') {
            assert!((0..words.len()).any(|index| words.word(index) == word));
        }
        assert!(
            matches!(app.password.assessment, Assessment::Generated(bits) if (bits - 4.0 * (words.len() as f64).log2()).abs() < 1e-9)
        );
    }

    #[test]
    fn f9_and_f11_open_the_editor_even_without_a_target() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        for action in [Action::OpenEditor, Action::GenerateTypeKeePassXC] {
            app.compact = false;
            app.handle_action(&ctx, action, Err(PlatformError::TargetUnavailable));
            assert!(app.compact);
            assert!(!app.target.can_restore());
            let output = frame(&ctx, &mut app, vec![]);
            assert!(output.viewport_output[&egui::ViewportId::ROOT]
                .commands
                .iter()
                .any(|command| matches!(command, egui::ViewportCommand::Focus)));
        }
        assert_eq!(
            app.notice,
            "KeePassXC: URL und Zielfeld ausdrücklich bestätigen."
        );
        assert!(clipboard.borrow().copied.is_empty());
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn f10_generates_types_and_copies_without_requesting_focus() {
        let ctx = egui::Context::default();
        let clipboard = Rc::new(RefCell::new(FakeClipboardState::default()));
        let target = Rc::new(RefCell::new(FakeTargetState::default()));
        let mut app = KairosApp::with_clipboard(
            &ctx,
            german_source(),
            false,
            Box::new(FakeClipboard(clipboard.clone())),
        );
        app.handle_action(
            &ctx,
            Action::GenerateTypeCopy,
            Ok(Box::new(FakeTarget(target.clone()))),
        );
        assert!(target.borrow().typed == [(app.password.secret.to_string(), false)]);
        assert!(clipboard.borrow().copied == [app.password.secret.to_string()]);
        assert!(!app.target.can_restore());
        let output = frame(&ctx, &mut app, vec![]);
        assert!(!output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .iter()
            .any(|command| matches!(
                command,
                egui::ViewportCommand::Focus
                    | egui::ViewportCommand::Minimized(false)
                    | egui::ViewportCommand::Visible(true)
            )));
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn f10_refuses_changed_target_without_copy_or_retry() {
        let ctx = egui::Context::default();
        let clipboard = Rc::new(RefCell::new(FakeClipboardState::default()));
        let target = Rc::new(RefCell::new(FakeTargetState {
            error: Some(PlatformError::TargetChanged),
            ..Default::default()
        }));
        let mut app = KairosApp::with_clipboard(
            &ctx,
            german_source(),
            false,
            Box::new(FakeClipboard(clipboard.clone())),
        );
        app.handle_action(
            &ctx,
            Action::GenerateTypeCopy,
            Ok(Box::new(FakeTarget(target.clone()))),
        );
        assert!(target.borrow().typed.is_empty());
        assert!(clipboard.borrow().copied.is_empty());
        assert!(app.notice.contains("gewechselt"));
        assert!(!app.target.can_restore());
    }

    #[test]
    fn editor_insert_uses_and_consumes_the_original_target() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        let target = Rc::new(RefCell::new(FakeTargetState::default()));
        app.target = Destination::Editor(Box::new(FakeTarget(target.clone())));
        app.password.secret.clear();
        app.password.secret.push_str("PUBLIC-Ä@#+'!?");
        app.password.edited();
        app.insert(&ctx);
        app.insert(&ctx);
        assert_eq!(target.borrow().typed, [("PUBLIC-Ä@#+'!?".into(), true)]);
        assert_eq!(clipboard.borrow().copied, ["PUBLIC-Ä@#+'!?"]);
        assert!(!app.target.can_restore());
    }

    #[test]
    fn startup_destination_can_only_return_focus_even_in_compact_mode() {
        let ctx = egui::Context::default();
        for compact in [false, true] {
            let (mut app, clipboard) = fixture_app(&ctx);
            let target = Rc::new(RefCell::new(FakeTargetState::default()));
            app.compact = compact;
            app.target = Destination::ReturnOnly(Box::new(FakeTarget(target.clone())));
            assert!(!app.target.can_insert());
            app.insert(&ctx);
            assert!(target.borrow().typed.is_empty());
            assert!(clipboard.borrow().copied.is_empty());
            assert!(app.target.can_restore());
            app.copy(&ctx, true);
            assert_eq!(target.borrow().restores, 1);
            assert_eq!(clipboard.borrow().copied, ["Öl-7Wiese"]);
            assert!(target.borrow().typed.is_empty());
        }
    }

    #[test]
    fn f9_explicitly_authorizes_the_destination_across_regeneration() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        let target = Rc::new(RefCell::new(FakeTargetState::default()));
        app.handle_action(
            &ctx,
            Action::OpenEditor,
            Ok(Box::new(FakeTarget(target.clone()))),
        );
        assert!(app.target.can_insert());
        app.regenerate(&ctx);
        assert!(app.target.can_insert());
        app.password.secret.clear();
        app.password.secret.push_str("PUBLIC EDITED FIXTURE");
        app.password.edited();
        app.insert(&ctx);
        assert_eq!(
            target.borrow().typed,
            [("PUBLIC EDITED FIXTURE".into(), true)]
        );
        assert_eq!(clipboard.borrow().copied, ["PUBLIC EDITED FIXTURE"]);
        assert!(!app.target.can_restore());
    }

    #[test]
    fn f11_generates_a_new_transaction_without_f9_input_permission() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        app.password.edited();
        app.handle_action(
            &ctx,
            Action::GenerateTypeKeePassXC,
            Err(PlatformError::TargetUnavailable),
        );
        // Empty fixture source makes the requested fresh generation fail closed.
        assert!(app.password.secret.is_empty());
        assert!(app.password.assessment == Assessment::Unavailable);
        assert!(app.keepass.flow.is_some());
        assert!(!app.target.can_insert());
        assert!(clipboard.borrow().copied.is_empty());
        assert_eq!(
            app.notice,
            "KeePassXC: URL und Zielfeld ausdrücklich bestätigen."
        );
    }

    #[test]
    fn f11_password_copy_uses_protected_clipboard_without_egui_export() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        app.keepass.flow = Some(keepass::Flow::new(None, None, None));
        let _ = frame(&ctx, &mut app, vec![]);
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let output = frame(&ctx, &mut app, vec![key(Key::C, Modifiers::CTRL)]);
        assert_eq!(clipboard.borrow().copied, ["Öl-7Wiese"]);
        assert!(!output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(command, egui::OutputCommand::CopyText(_))));
    }

    #[test]
    fn f11_generator_and_protected_copy_errors_are_visible() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        app.handle_action(
            &ctx,
            Action::GenerateTypeKeePassXC,
            Err(PlatformError::TargetUnavailable),
        );
        let output = frame(&ctx, &mut app, vec![]);
        assert!(output
            .shapes
            .iter()
            .any(|s| has_text(&s.shape, "Noch keine Wörter verfügbar")));
        app.password.secret.push_str("PUBLIC TEST");
        clipboard.borrow_mut().fail_copy = true;
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let output = frame(&ctx, &mut app, vec![key(Key::C, Modifiers::CTRL)]);
        assert!(output
            .shapes
            .iter()
            .any(|s| has_text(&s.shape, "Sicheres Kopieren fehlgeschlagen")));
    }

    #[test]
    fn registration_error_and_retry_control_remain_visible_after_regeneration() {
        fn has_text(shape: &egui::epaint::Shape, expected: &str) -> bool {
            match shape {
                egui::epaint::Shape::Text(text) => text.galley.text().contains(expected),
                egui::epaint::Shape::Vec(shapes) => {
                    shapes.iter().any(|shape| has_text(shape, expected))
                }
                _ => false,
            }
        }
        let ctx = egui::Context::default();
        let (mut app, _) = fixture_app(&ctx);
        app.hotkey_error = Some("PUBLIC HOTKEY REGISTRATION FAILURE");
        let _ = frame(&ctx, &mut app, vec![]);
        app.regenerate(&ctx);
        let output = frame(&ctx, &mut app, vec![]);
        for expected in [
            "PUBLIC HOTKEY REGISTRATION FAILURE",
            "Hotkeys erneut registrieren",
        ] {
            assert!(output
                .shapes
                .iter()
                .any(|shape| has_text(&shape.shape, expected)));
        }
        assert!(output
            .shapes
            .iter()
            .filter(|shape| has_text(&shape.shape, "PUBLIC HOTKEY REGISTRATION FAILURE"))
            .all(|shape| shape.shape.visual_bounding_rect().top() < 250.0));
    }

    #[test]
    fn editor_failure_does_not_copy_and_copy_opt_out_is_respected() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        let target = Rc::new(RefCell::new(FakeTargetState {
            error: Some(PlatformError::InputIncomplete),
            ..Default::default()
        }));
        app.target = Destination::Editor(Box::new(FakeTarget(target.clone())));
        app.insert(&ctx);
        assert!(clipboard.borrow().copied.is_empty());
        assert!(app.notice.contains("Teil der Eingabe"));
        assert!(!app.target.can_restore());
        target.borrow_mut().error = None;
        app.target = Destination::Editor(Box::new(FakeTarget(target.clone())));
        app.copy_on_insert = false;
        app.insert(&ctx);
        assert_eq!(target.borrow().typed.len(), 1);
        assert!(clipboard.borrow().copied.is_empty());
    }

    #[cfg(not(feature = "demo"))]
    #[test]
    fn rotation_uses_core_bound_and_never_promises_four_bonus_bits() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, german_source(), false);
        app.rotate_profiles = true;
        app.regenerate(&ctx);
        assert_eq!(app.rotation_bonus, Some(0.0));
        let minimum =
            kairos_core::entropy_bits_for(app.source.as_ref(), &GeneratorConfig::default())
                .unwrap();
        assert!(app.password.assessment == Assessment::Generated(minimum));
        app.config.excluded_chars.push('-');
        app.regenerate(&ctx);
        assert!(app.password.secret.is_empty());
        assert_eq!(app.rotation_bonus, None);
    }

    #[test]
    fn paste_requires_editor_focus_and_removes_control_characters() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        clipboard.borrow_mut().paste = "a\r\nb\t\0".into();
        let _ = frame(&ctx, &mut app, vec![key(Key::V, Modifiers::CTRL)]);
        assert_eq!(clipboard.borrow().paste_calls, 0);
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let _ = frame(&ctx, &mut app, vec![key(Key::V, Modifiers::CTRL)]);
        assert_eq!(clipboard.borrow().paste_calls, 1);
        assert!(app.password.secret.contains("ab"));
        assert!(!app.password.secret.chars().any(char::is_control));
        assert!(matches!(app.password.assessment, Assessment::Edited));
    }

    #[test]
    fn enter_does_not_insert_a_newline_and_utf8_paste_cannot_reallocate() {
        let ctx = egui::Context::default();
        let (mut app, _) = fixture_app(&ctx);
        let _ = frame(&ctx, &mut app, vec![]);
        let pointer = app.password.secret.as_ptr();
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let _ = frame(&ctx, &mut app, vec![key(Key::Enter, Modifiers::NONE)]);
        assert_eq!(&**app.password.secret, "Öl-7Wiese");
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
        let _ = frame(
            &ctx,
            &mut app,
            vec![egui::Event::Paste("🦀".repeat(MAX_SECRET_CHARS + 10))],
        );
        assert_eq!(app.password.secret.chars().count(), MAX_SECRET_CHARS);
        assert_eq!(app.password.secret.as_ptr(), pointer);
    }

    #[test]
    fn settings_regenerate_clean_results_but_preserve_manual_edits() {
        let ctx = egui::Context::default();
        let (mut app, _) = fixture_app(&ctx);
        let _ = frame(&ctx, &mut app, vec![]);
        app.config.profile = ProfileId::new(12).unwrap();
        app.settings_changed(&ctx);
        assert!(egui::TextEdit::load_state(&ctx, egui::Id::unique("password")).is_none());
        assert_ne!(&**app.password.secret, "Öl-7Wiese");
        let _ = frame(&ctx, &mut app, vec![]);
        app.excluded[0] = true;
        apply_exclusions(&mut app.config, &app.excluded);
        app.settings_changed(&ctx);
        assert!(egui::TextEdit::load_state(&ctx, egui::Id::unique("password")).is_none());
        app.password.secret.clear();
        app.password.secret.push_str("PUBLIC MANUAL FIXTURE");
        app.password.edited();
        app.config.profile = ProfileId::new(1).unwrap();
        app.settings_changed(&ctx);
        assert_eq!(&**app.password.secret, "PUBLIC MANUAL FIXTURE");
        assert!(app.notice.contains("bleibt erhalten"));
    }
}
