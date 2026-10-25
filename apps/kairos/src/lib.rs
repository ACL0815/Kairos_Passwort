//! Kairos native application, consuming only the stable core contract.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples
//! Modelled after: Password Tech https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/main

#[cfg(all(feature = "demo", not(debug_assertions)))]
compile_error!("The fake-data demo feature must never be used in a release build.");

mod capture;
mod clipboard;
mod desktop;
mod model;
mod source;
mod theme;

pub use source::german_source;

#[cfg(all(feature = "demo", debug_assertions))]
mod demo;

use clipboard::{ClipboardAccess, ProtectedClipboard};
use desktop::{boxed_target, input_error, Destination};
use eframe::egui::{self, Color32, Key, Modifiers, RichText};
#[cfg(not(all(feature = "demo", debug_assertions)))]
use kairos_core::generate;
use kairos_core::{GeneratorConfig, ProfileId, WordSource};
use kairos_platform::{Action, FocusTarget, GlobalHotkeys, HotkeyBinding, PlatformError};
use model::{
    apply_exclusions, password_layout, Assessment, PasswordState, MAX_SECRET_CHARS, SEPARATORS,
};

pub struct KairosApp {
    source: Box<dyn WordSource>,
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
    rotation_bonus: Option<f64>,
    copy_on_insert: bool,
    capture_protection: Option<kairos_platform::CaptureProtection>,
}

impl KairosApp {
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
        let mut app = Self {
            source,
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
            rotation_bonus: None,
            copy_on_insert: true,
            capture_protection: None,
        };
        #[cfg(all(feature = "demo", debug_assertions))]
        {
            app.config.profile = ProfileId::new(8).expect("demo profile");
        }
        app.regenerate(ctx);
        app
    }

    /// Keeps the GUI-thread hook alive for all subsequent native viewports and dialogs.
    pub fn connect_capture_protection(&mut self, protection: kairos_platform::CaptureProtection) {
        self.capture_protection = Some(protection);
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
    }

    fn register_hotkeys(&mut self, ctx: &egui::Context) {
        if cfg!(feature = "demo") {
            self.hotkey_error = Some("In der öffentlichen Demo sind globale Hotkeys deaktiviert.");
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
                // A background action must not retain a previous editor destination.
                self.target = Destination::Unavailable;
                self.regenerate(ctx);
                if self.password.secret.is_empty() {
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
                self.show_popup(ctx);
                self.notice = "KeePassXC-Anbindung folgt (PR-6)";
            }
        }
    }

    /// Shows the same editor without replacing the destination captured by the caller.
    pub fn show_popup(&mut self, ctx: &egui::Context) {
        self.compact = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(720.0, 640.0)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::unique("password")));
    }

    fn regenerate(&mut self, ctx: &egui::Context) {
        self.rotation_bonus = None;
        #[cfg(all(feature = "demo", debug_assertions))]
        let result = demo::generate(self.source.as_ref(), &self.config);
        #[cfg(not(all(feature = "demo", debug_assertions)))]
        let result = if self.rotate_profiles {
            let rotation = kairos_core::ProfileRotation::new(
                (0..=12)
                    .map(|id| ProfileId::new(id).expect("built-in profile").into())
                    .collect(),
            )
            .expect("thirteen built-in profiles");
            kairos_core::generate_rotating(
                self.source.as_ref(),
                &rotation,
                &self.config.excluded_chars,
                kairos_core::ProfileKnowledge::Unknown,
            )
            .map(|result| {
                self.rotation_bonus = Some(result.rotation_bonus_bits);
                result.password
            })
        } else {
            generate(self.source.as_ref(), &self.config)
        };
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

    fn editor(&mut self, ui: &mut egui::Ui) {
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
            .desired_rows(if self.compact { 2 } else { 3 })
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
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        let previous = self.config.profile;
        ui.horizontal_wrapped(|ui| {
            ui.label("Profil");
            ui.add_enabled_ui(!self.rotate_profiles, |ui| {
                egui::ComboBox::from_id_salt("profile")
                    .selected_text(if self.rotate_profiles {
                        "Zufall aus Profil 0–12".into()
                    } else {
                        format!(
                            "{} · {}",
                            self.config.profile.get(),
                            profile_name(self.config.profile.get())
                        )
                    })
                    .width(if self.compact { 240.0 } else { 300.0 })
                    .show_ui(ui, |ui| {
                        for value in 0..=12 {
                            let profile = ProfileId::new(value).expect("built-in profile");
                            ui.selectable_value(
                                &mut self.config.profile,
                                profile,
                                format!("{value} · {}", profile_name(value)),
                            );
                        }
                    });
            });
            if ui
                .button(if self.compact {
                    "Vollansicht"
                } else {
                    "Kompakt"
                })
                .clicked()
            {
                self.compact = !self.compact;
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(if self.compact {
                        egui::vec2(720.0, 640.0)
                    } else {
                        egui::vec2(860.0, 820.0)
                    }));
            }
        });
        if previous != self.config.profile {
            self.settings_changed(ui.ctx());
        }
        if ui
            .checkbox(
                &mut self.rotate_profiles,
                "Profil bei jeder Generierung wechseln",
            )
            .changed()
        {
            self.settings_changed(ui.ctx());
        }
        if self.rotate_profiles {
            ui.label("Zufall aus Profil 0–12; Wiederholungen sind möglich. Stärke richtet sich nach dem schwächsten Profil.");
            if let Some(bonus) = self
                .rotation_bonus
                .filter(|_| matches!(self.password.assessment, Assessment::Generated(_)))
            {
                ui.label(format!("Profilbonus: +{bonus:.1} Bit. Bei 0 Bit: Profile überlappen oder sind unterschiedlich stark."));
            }
        }
        ui.label(format!("Wortquelle: {} Einträge", self.source.len()));
    }

    fn exclusions(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;
        egui::CollapsingHeader::new("Sonderzeichen ausschließen")
            .default_open(false)
            .show(ui, |ui| {
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
        egui::CentralPanel::default().frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(if self.compact { 18 } else { 28 })).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new("KAIROS").color(theme::CYAN).size(15.0).strong());
                ui.heading("Dein nächstes Passwort.");
                if !self.compact { ui.label(RichText::new("Leicht abzuschreiben. Sicher per Zufall.").color(theme::MUTED)); }
                #[cfg(all(feature = "demo", debug_assertions))]
                ui.colored_label(Color32::from_rgb(251, 191, 36), "DEMO · Öffentliche Beispieldaten, kein sicheres Passwort");
                if let Some(error) = self.hotkey_error {
                    ui.colored_label(model::segment_color(kairos_core::SegmentKind::Separator), error);
                    if ui.button("Hotkeys erneut registrieren").clicked() { self.register_hotkeys(&ctx); }
                }
                ui.add_space(10.0);
                theme::panel().show(ui, |ui| {
                    self.controls(ui);
                    ui.add_space(8.0);
                    self.editor(ui);
                    ui.horizontal_wrapped(|ui| {
                        for (label, kind) in [("Wort", kairos_core::SegmentKind::Word), ("123", kairos_core::SegmentKind::Digit), ("#+", kairos_core::SegmentKind::Separator), ("A", kairos_core::SegmentKind::Uppercase)] {
                            ui.label(RichText::new(label).color(model::segment_color(kind)).monospace().size(13.0));
                        }
                        ui.label(RichText::new("· direkt bearbeitbar · ohne Undo-Verlauf").color(theme::MUTED).size(12.0));
                    });
                    ui.label(RichText::new(self.password.message).color(theme::MUTED).size(14.0));
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
                });
                if !self.notice.is_empty() { ui.label(RichText::new(self.notice).color(theme::CYAN).size(14.0)); }
                let bits = match self.password.assessment { Assessment::Generated(bits) => Some(bits), _ => None };
                let assessment = match self.password.assessment {
                    Assessment::Generated(bits) => format!("{bits:.1} Bit berechnete Entropie"),
                    Assessment::Edited => "bearbeitet – nicht mehr bewertbar".into(),
                    Assessment::Unavailable => "Stärke noch nicht verfügbar".into(),
                };
                if !self.compact {
                    theme::panel().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            theme::strength_ring(ui, bits);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&assessment).size(20.0).strong());
                                ui.label(RichText::new("Aus dem Generator und seiner tatsächlichen Wortauswahl.").color(theme::MUTED));
                                ui.add(egui::ProgressBar::new(bits.map_or(0.0, |bits| (bits / 160.0).clamp(0.0, 1.0) as f32)).fill(theme::CYAN).desired_width(280.0));
                            });
                        });
                    });
                } else { ui.label(assessment); }
                self.exclusions(ui);
                ui.label(RichText::new("Strg+Umschalt+F9: Editor · F10: Tippen + Kopieren · F11: KeePassXC-Hinweis").color(theme::MUTED).size(12.0));
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
    }

    fn settings_changed(&mut self, ctx: &egui::Context) {
        if self.password.assessment == Assessment::Edited {
            self.notice = "Deine Nacharbeit bleibt erhalten. Neue Einstellungen gelten beim nächsten Neu generieren.";
        } else {
            self.regenerate(ctx);
        }
    }
}

impl eframe::App for KairosApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_hotkeys(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

fn profile_name(profile: u8) -> &'static str {
    match profile {
        0 => "Vier Wörter · feste Minuszeichen",
        1 => "Vier Wörter · größere Wortauswahl",
        2 => "Fünf mögliche Trenner",
        3 => "Zusätzliche Zufallsziffer",
        4 => "Acht mögliche Trenner",
        5 => "Zwölf mögliche Trenner",
        6 => "16 mögliche Trenner",
        7 => "20 mögliche Trenner",
        8 => "Zufällige Wortanfänge",
        9 => "Fünf Wörter",
        10 => "Sechs Wörter",
        11 => "Zufälliger Großbuchstabe je Wort",
        12 => "Zwei Trenner je Wortpaar",
        _ => unreachable!("validated ProfileId"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            assert!(!profile_name(ProfileId::new(value).unwrap().get()).is_empty());
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
        assert_eq!(app.notice, "KeePassXC-Anbindung folgt (PR-6)");
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
    fn f11_keeps_manual_work_and_does_not_authorize_a_new_destination() {
        let ctx = egui::Context::default();
        let (mut app, clipboard) = fixture_app(&ctx);
        app.password.edited();
        app.handle_action(
            &ctx,
            Action::GenerateTypeKeePassXC,
            Err(PlatformError::TargetUnavailable),
        );
        assert_eq!(&**app.password.secret, "Öl-7Wiese");
        assert!(app.password.assessment == Assessment::Edited);
        assert!(!app.target.can_insert());
        assert!(clipboard.borrow().copied.is_empty());
        assert_eq!(app.notice, "KeePassXC-Anbindung folgt (PR-6)");
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
