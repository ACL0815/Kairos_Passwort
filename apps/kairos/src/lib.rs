//! Kairos native application, consuming only the stable core contract.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples
//! Modelled after: Password Tech https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/main

#[cfg(all(feature = "demo", not(debug_assertions)))]
compile_error!("The fake-data demo feature must never be used in a release build.");

mod clipboard;
mod model;
mod theme;

#[cfg(all(feature = "demo", debug_assertions))]
mod demo;

use clipboard::ProtectedClipboard;
use eframe::egui::{self, Color32, Key, Modifiers, RichText};
#[cfg(not(all(feature = "demo", debug_assertions)))]
use kairos_core::generate;
use kairos_core::{GeneratorConfig, ProfileId, WordSource};
use model::{apply_exclusions, password_layout, Assessment, PasswordState, SEPARATORS};

pub struct KairosApp {
    source: Box<dyn WordSource>,
    config: GeneratorConfig,
    excluded: [bool; 20],
    password: PasswordState,
    clipboard: ProtectedClipboard,
    notice: &'static str,
    compact: bool,
}

impl KairosApp {
    pub fn new(ctx: &egui::Context, source: Box<dyn WordSource>, compact: bool) -> Self {
        theme::install(ctx);
        let mut app = Self {
            source,
            config: GeneratorConfig::default(),
            excluded: [false; 20],
            password: PasswordState::default(),
            clipboard: ProtectedClipboard::new(),
            notice: "",
            compact,
        };
        #[cfg(all(feature = "demo", debug_assertions))]
        {
            app.config.profile = ProfileId::new(8).expect("demo profile");
        }
        app.regenerate(ctx);
        app
    }

    /// Entry point for the later global-hotkey integration.
    pub fn show_popup(&mut self, ctx: &egui::Context) {
        self.compact = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(720.0, 560.0)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
    }

    fn regenerate(&mut self, ctx: &egui::Context) {
        #[cfg(all(feature = "demo", debug_assertions))]
        let result = demo::generate(self.source.as_ref(), &self.config);
        #[cfg(not(all(feature = "demo", debug_assertions)))]
        let result = generate(self.source.as_ref(), &self.config);
        self.password.set_result(result);
        self.notice = "";
        ctx.data_mut(|data| {
            data.remove::<egui::text_edit::TextEditState>(egui::Id::unique("password"))
        });
    }

    fn copy(&mut self, ctx: &egui::Context, return_to_previous: bool) {
        match self.clipboard.copy(&self.password.secret) {
            Ok(()) => {
                self.notice = "Kopiert. Zwischenablage wird nach 30 Sekunden geleert, sofern unverändert.";
                if return_to_previous {
                    ctx.memory_mut(|memory| memory.surrender_focus(egui::Id::unique("password")));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
            }
            Err(()) => self.notice = "Sicheres Kopieren fehlgeschlagen. Bitte erneut versuchen. Das Fenster bleibt geöffnet.",
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
        let mut output = egui::TextEdit::multiline(&mut *self.password.secret)
            .id(id)
            .password(true)
            .layouter(&mut layouter)
            .font(egui::FontId::monospace(size))
            .desired_width(f32::INFINITY)
            .desired_rows(if self.compact { 2 } else { 3 })
            .hint_text("Hier erscheint dein Wortpasswort")
            .char_limit(4096)
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
            egui::ComboBox::from_id_salt("profile")
                .selected_text(format!(
                    "{} · {}",
                    self.config.profile.get(),
                    profile_name(self.config.profile.get())
                ))
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
                        egui::vec2(720.0, 560.0)
                    } else {
                        egui::vec2(860.0, 820.0)
                    }));
            }
        });
        if previous != self.config.profile {
            self.regenerate(ui.ctx());
        }
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
            self.regenerate(ui.ctx());
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let paste = ctx.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::V));
        if paste && ctx.memory(|memory| memory.has_focus(egui::Id::unique("password"))) {
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
                ui.label(RichText::new("KAIROS").color(theme::CYAN).size(15.0).strong());
                ui.heading("Dein nächstes Passwort.");
                if !self.compact { ui.label(RichText::new("Leicht abzuschreiben. Sicher per Zufall.").color(theme::MUTED)); }
                #[cfg(all(feature = "demo", debug_assertions))]
                ui.colored_label(Color32::from_rgb(251, 191, 36), "DEMO · Öffentliche Beispieldaten, kein sicheres Passwort");
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
                ui.horizontal_wrapped(|ui| {
                    if ui.add(egui::Button::new(RichText::new("Neu generieren").strong()).fill(Color32::from_rgb(45, 79, 132))).on_hover_text("Strg+R").clicked() { self.regenerate(&ctx); }
                    let available = !self.password.secret.is_empty();
                    if ui.add_enabled(available, egui::Button::new("Kopieren")).on_hover_text("Strg+C kopiert das ganze Passwort").clicked() { self.copy(&ctx, false); }
                    if ui.add_enabled(available, egui::Button::new("Kopieren und zurückspringen")).clicked() { self.copy(&ctx, true); }
                });
                if !self.notice.is_empty() { ui.label(RichText::new(self.notice).color(theme::CYAN).size(14.0)); }
                if !self.compact { ui.label(RichText::new("Kein Passwortverlauf · Kein Cloud-Sync · Zwischenablage 30 s").color(theme::MUTED).size(12.0)); }
            });
        });
    }
}

impl eframe::App for KairosApp {
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
    use kairos_core::{GeneratedPassword, Language};
    use zeroize::Zeroizing;

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
    fn scaffold_error_can_render_full_and_compact_without_entropy_stub() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, Box::new(EmptySource), false);
        assert!(app.password.secret.is_empty());
        assert!(app.password.message.contains("noch nicht verfügbar"));
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
}
