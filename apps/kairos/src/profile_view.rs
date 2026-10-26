//! Profile controls, opt-in explanations, and honest runtime/reference labels.

use eframe::egui::{self, RichText};
use kairos_core::{GeneratorConfig, ProfileId, WordSource};

use crate::{help, model::Assessment, source, strength, theme, KairosApp};

impl KairosApp {
    /// Cache public configuration assessments on settings changes, never per frame.
    pub(super) fn refresh_profile_bits(&mut self) {
        if self.blocked.error.is_some() {
            self.profile_bits.fill(None);
            return;
        }
        let german = source::FilteredSource::new(self.source.as_ref(), &self.blocked.words);
        let expanded =
            source::FilteredSource::new(self.expanded_source.as_ref(), &self.blocked.words);
        self.source_counts = [german.len(), expanded.len()];
        self.profile_bits = std::array::from_fn(|id| {
            let profile = ProfileId::new(id as u8).expect("built-in");
            let selected: &dyn WordSource = if source::uses_german(profile, self.german_only) {
                &german
            } else {
                &expanded
            };
            kairos_core::entropy_bits_for(
                selected,
                &GeneratorConfig {
                    profile,
                    excluded_chars: self.config.excluded_chars.clone(),
                },
            )
            .ok()
        });
    }

    pub(super) fn controls(&mut self, ui: &mut egui::Ui) {
        let previous = self.config.profile;
        ui.horizontal_wrapped(|ui| {
            ui.label("Profil");
            ui.add_enabled_ui(!self.rotate_profiles, |ui| {
                egui::ComboBox::from_id_salt("profile")
                    .selected_text(if self.rotate_profiles {
                        format!(
                            "Zufall aus {} Profilen",
                            self.rotation_selection.ids().len()
                        )
                    } else {
                        let id = self.config.profile.get();
                        format!("{id} · {}", help::PROFILES[id as usize].name)
                    })
                    .width(if self.compact { 240.0 } else { 300.0 })
                    .show_ui(ui, |ui| {
                        for id in 0..=12 {
                            ui.push_id(id, |ui| {
                                ui.horizontal(|ui| {
                                    ui.selectable_value(
                                        &mut self.config.profile,
                                        ProfileId::new(id).expect("built-in"),
                                        format!("{id} · {}", help::PROFILES[id as usize].name),
                                    );
                                    if help::info_button(
                                        ui,
                                        &format!("Profil {id}"),
                                        &help::profile_text(id),
                                    )
                                    .clicked()
                                    {
                                        self.help_profile = id;
                                        self.show_profiles = true;
                                        ui.close();
                                    }
                                });
                            });
                        }
                    });
            });
            if ui.small_button("Profile erklärt").clicked() {
                self.help_profile = self.config.profile.get();
                self.show_profiles = true;
            }
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
        let mut rotation_changed = false;
        ui.horizontal_wrapped(|ui| {
            rotation_changed |= ui
                .checkbox(
                    &mut self.rotate_profiles,
                    "Profil bei jeder Generierung wechseln",
                )
                .changed();
            help::info(ui, "Profilrotation", help::ROTATION);
        });
        if self.rotate_profiles {
            if !self.compact {
                ui.label(
                    RichText::new("Profile wählen · mindestens zwei · Wiederholungen möglich")
                        .size(13.0)
                        .color(theme::MUTED),
                );
            }
            ui.horizontal_wrapped(|ui| {
                for id in 0..=12 {
                    let mut selected = self.rotation_selection.ids().contains(&id);
                    let can_remove = !selected || self.rotation_selection.ids().len() > 2;
                    let response = ui
                        .add_enabled(
                            can_remove,
                            egui::Checkbox::new(&mut selected, id.to_string()),
                        )
                        .on_hover_text(help::profile_text(id))
                        .on_disabled_hover_text(
                            "Mindestens zwei Profile müssen ausgewählt bleiben.",
                        );
                    if response.changed() {
                        // Selection enforces the same minimum as the disabled checkbox.
                        if self.rotation_selection.set(id, selected).is_ok() {
                            rotation_changed = true;
                        }
                    }
                }
            });
        }
        if rotation_changed {
            self.rotation_settings
                .save(self.rotate_profiles, &self.rotation_selection);
            self.settings_changed(ui.ctx());
        }
        if let Some(notice) = self.rotation_settings.notice {
            ui.colored_label(egui::Color32::from_rgb(251, 191, 36), notice);
        }
        if self.compact {
            let title = if self.german_only {
                "Wortquelle · Nur Deutsch"
            } else if self.rotate_profiles {
                "Wortquellen · Profil 0 Deutsch, ab 1 DE+EN+LA"
            } else if self.config.profile.get() == 0 {
                "Wortquelle · Deutsch"
            } else {
                "Wortquelle · DE+EN+LA"
            };
            egui::CollapsingHeader::new(title).show(ui, |ui| self.source_controls(ui));
        } else {
            self.source_controls(ui);
        }
    }

    fn source_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui
                .checkbox(&mut self.german_only, "Nur deutsche Wörter")
                .changed()
            {
                self.settings_changed(ui.ctx());
            }
            help::info(ui, "Wortquelle", help::GERMAN);
        });
        let description = if self.rotate_profiles && !self.german_only {
            format!(
                "Wortquelle: Profil 0 Deutsch ({}); ab 1 DE+EN+LA ({}).",
                self.source_counts[0], self.source_counts[1]
            )
        } else if source::uses_german(self.config.profile, self.german_only) {
            format!(
                "Wortquelle: Deutsch · {} Wörter nach Sperrliste",
                self.source_counts[0]
            )
        } else {
            format!(
                "Wortquelle: DE+EN+LA · {} Wörter nach Sperrliste",
                self.source_counts[1]
            )
        };
        ui.label(RichText::new(description).size(13.0).color(theme::MUTED));
        if self.german_only {
            ui.label(
                RichText::new(
                    "Deutsch-Override: Zusätzliche Wortauswahl-Bits von Profil 1 fehlen.",
                )
                .size(13.0)
                .color(theme::MUTED),
            );
        }
    }

    fn deviation_reason(&self, id: u8) -> String {
        let german = source::uses_german(ProfileId::new(id).expect("built-in"), self.german_only);
        let count = self.source_counts[usize::from(!german)];
        let target = if id == 0 { "7.000" } else { "21.000" };
        let mut reasons = vec![format!("{count} statt {target} Wörter")];
        if id >= 3 {
            reasons.push("reale Wortlängen statt sechs Buchstaben".into());
        }
        if id >= 2 {
            reasons.push("aktuelle Core-Trennerregeln".into());
        }
        if self.german_only && id > 0 {
            reasons.push("Nur Deutsch".into());
        }
        let unfiltered_count = if german {
            self.source.len()
        } else {
            self.expanded_source.len()
        };
        if count < unfiltered_count {
            reasons.push("Sperrliste".into());
        }
        if self.config.excluded_chars.iter().any(|&character| {
            kairos_core::profile_rules(ProfileId::new(id).expect("built-in"))
                .separator_pool
                .contains(character)
        }) {
            reasons.push("Trennerausschlüsse".into());
        }
        reasons.join(" · ")
    }

    pub(super) fn strength_card(&self, ui: &mut egui::Ui) {
        let bits = match self.password.assessment {
            Assessment::Generated(bits) => Some(bits),
            _ => None,
        };
        let concept = if self.rotate_profiles {
            format!(
                "{} Bit · Minimum der Auswahl",
                strength::decimal(self.rotation_selection.concept_minimum())
            )
        } else {
            format!(
                "{} Bit",
                strength::decimal(strength::CONCEPT_BITS[self.config.profile.get() as usize])
            )
        };
        if self.compact {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Theoretisch (Konzept): {concept}"));
                help::info(ui, "Stärke und Vergleich", help::STRENGTH);
            });
            if let Some(bits) = bits {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(format!("Tatsächlich: {} Bit", strength::decimal(bits)))
                            .color(theme::CYAN)
                            .strong(),
                    );
                    if let Some(length) = strength::equivalent_characters(bits) {
                        ui.label(format!(
                            "Vergleich: so stark wie {length} zufällige Zeichen*"
                        ));
                    }
                });
                if let Some(guesses) = strength::g50(bits) {
                    ui.label(format!("G50-Modell: {guesses} Rateversuche*"));
                }
                if self.rotate_profiles {
                    if let Some(bonus) = self.rotation_bonus {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(format!(
                                "Profilbonus: +{} Bit (Core)",
                                strength::decimal(bonus)
                            ));
                            help::info(ui, "Profilbonus", help::BONUS);
                        });
                    }
                }
            } else {
                ui.label(if self.password.assessment == Assessment::Edited {
                    "bearbeitet – nicht mehr bewertbar · kein Profilbonus"
                } else {
                    "Tatsächlich: nicht verfügbar · kein Vergleich"
                });
            }
            return;
        }
        theme::panel().inner_margin(14).show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.horizontal_wrapped(|ui| {
                ui.strong("Profil und Stärke");
                help::info(ui, "Stärke und Vergleich", help::STRENGTH);
                ui.label(format!("Theoretisch (Konzept): {concept}"));
            });
            ui.horizontal(|ui| {
                if !self.compact {
                    theme::strength_ring(ui, bits);
                }
                ui.vertical(|ui| match bits {
                Some(bits) => {
                    ui.label(RichText::new(format!("Tatsächlich: {} Bit · konservative Core-Grenze", strength::decimal(bits))).color(theme::CYAN).strong());
                    comparison(ui, bits);
                    if self.rotate_profiles {
                        ui.label(RichText::new("Bewertung der gesamten Rotation; keine Einzelstärke des gezogenen Profils.").size(12.0).color(theme::MUTED));
                        if let Some(bonus) = self.rotation_bonus {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(format!("Profilbonus: +{} Bit (Core)", strength::decimal(bonus)));
                                help::info(ui, "Profilbonus", help::BONUS);
                            });
                        }
                    } else {
                        ui.label(RichText::new(format!("Abweichung: {}.", self.deviation_reason(self.config.profile.get()))).size(12.0).color(theme::MUTED));
                    }
                }
                None => {
                    ui.label(if self.password.assessment == Assessment::Edited {
                        "bearbeitet – nicht mehr bewertbar · kein Profilbonus"
                    } else {
                        "Tatsächlich: nicht verfügbar · kein Vergleich"
                    });
                }
                });
            });
        });
    }

    pub(super) fn profiles_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_profiles;
        egui::Window::new("Profile erklärt")
            .open(&mut open)
            .fade_in(false)
            .default_width(620.0)
            .default_height(720.0)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label(help::PRINCIPLE);
                ui.label("Alle Regeln sind kumulativ. Konzeptwerte sind feste Referenzen; tatsächliche Werte unten gelten für die aktuellen Einstellungen, nicht für manuelle Nacharbeit.");
                ui.horizontal_wrapped(|ui| {
                    for id in 0..=12 {
                        ui.selectable_value(&mut self.help_profile, id, id.to_string());
                    }
                });
                let id = self.help_profile;
                ui.separator();
                ui.strong(format!("Profil {id} · {}", help::PROFILES[id as usize].name));
                if let Some(bits) = self.profile_bits[id as usize] {
                    ui.label(format!("Tatsächlich (aktuelle Einstellungen): {} Bit", strength::decimal(bits)));
                    if id > 0 {
                        if let Some(previous) = self.profile_bits[id as usize - 1] {
                            ui.label(format!("Tatsächlicher Unterschied zum Vorgänger: {} Bit", strength::decimal(bits - previous)));
                        }
                    }
                    comparison(ui, bits);
                } else {
                    ui.label("Tatsächlich: mit diesen Einstellungen nicht verfügbar. Sperrliste und Trennerausschlüsse prüfen.");
                }
                ui.label(help::profile_body(id));
                ui.label(format!("Abweichungsgründe: {}.", self.deviation_reason(id)));
                egui::CollapsingHeader::new("Alle 13 Profile im Vergleich").show(ui, |ui| {
                    egui::Grid::new("profile_strengths").striped(true).show(ui, |ui| {
                        ui.strong("Profil");
                        ui.strong("Konzept (Bit)");
                        ui.strong("Core (Bit)");
                        ui.end_row();
                        for id in 0..13 {
                            if ui.selectable_label(self.help_profile as usize == id, id.to_string()).clicked() {
                                self.help_profile = id as u8;
                            }
                            ui.label(strength::decimal(strength::CONCEPT_BITS[id]));
                            ui.label(self.profile_bits[id].map_or_else(|| "nicht verfügbar".into(), strength::decimal));
                            ui.end_row();
                        }
                    });
                });
                egui::CollapsingHeader::new("Wie werden die Werte berechnet?").show(ui, |ui| {
                    ui.label(help::STRENGTH);
                    ui.label("Konzeptreferenz: docs/Grundkonzept.md, beide Profiltabellen. Die G50-Zellen im Konzept wurden aus ungerundeten Modellwerten berechnet; hier wird mit dem tatsächlichen Core-Wert gerechnet.");
                    ui.label(help::BONUS);
                });
                egui::CollapsingHeader::new("Bedienung und Schutz erklärt").show(ui, |ui| {
                    for (title, text) in [
                        ("Editor und Generierung", help::EDITOR),
                        ("Sonderzeichen", help::EXCLUSIONS),
                        ("Nur Deutsch", help::GERMAN),
                        ("Sperrliste", help::BLOCKLIST),
                        ("Rotation", help::ROTATION),
                        ("Kopieren und Zurückspringen", help::CLIPBOARD),
                        ("Hotkeys F9/F10/F11", help::HOTKEYS),
                        ("Bildschirmaufnahmeschutz", help::CAPTURE),
                        ("Härtungsreport", help::HARDENING),
                        ("Violentmonkey-Kopplung", help::BRIDGE),
                        ("KeePassXC", help::KEEPASS),
                    ] {
                        egui::CollapsingHeader::new(title).show(ui, |ui| { ui.label(text); });
                    }
                });
            });
        self.show_profiles = open;
    }
}

fn comparison(ui: &mut egui::Ui, bits: f64) {
    if let (Some(length), Some(guesses)) =
        (strength::equivalent_characters(bits), strength::g50(bits))
    {
        ui.label(format!(
            "Vergleich: so stark wie {length} zufällige Zeichen*"
        ));
        ui.label(format!("G50-Modell: {guesses} Rateversuche*"));
        ui.label(
            RichText::new("* 94 ASCII · vier Zeichenarten · Modellvergleich / Untergrenze.")
                .size(12.0)
                .color(theme::MUTED),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(shape: &egui::epaint::Shape, output: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                output.push_str(shape.galley.text());
                output.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    text(shape, output);
                }
            }
            _ => {}
        }
    }

    fn card(app: &KairosApp, ctx: &egui::Context) -> String {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 1500.0),
                )),
                ..Default::default()
            },
            |ui| app.strength_card(ui),
        );
        let mut output_text = String::new();
        output.textures_delta.clear();
        for shape in output.shapes {
            text(&shape.shape, &mut output_text);
        }
        output_text
    }

    fn controls_frame(
        app: &mut KairosApp,
        ctx: &egui::Context,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 1050.0),
                )),
                events,
                ..Default::default()
            },
            |ui| app.controls(ui),
        );
        output.textures_delta.clear();
        output
    }

    fn caption_rect(output: &egui::FullOutput, caption: &str) -> egui::Rect {
        output
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) if text.galley.text() == caption => {
                    Some(shape.shape.visual_bounding_rect())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("missing control: {caption}"))
    }

    fn click_control(
        app: &mut KairosApp,
        ctx: &egui::Context,
        pos: egui::Pos2,
    ) -> egui::FullOutput {
        for pressed in [true, false] {
            let _ = controls_frame(
                app,
                ctx,
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
        }
        let _ = controls_frame(app, ctx, vec![]);
        controls_frame(app, ctx, vec![])
    }

    #[test]
    fn checkbox_actions_persist_and_the_startup_loader_restores_the_selection() {
        let ctx = egui::Context::default();
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("settings.json");
        let mut app = KairosApp::new(&ctx, crate::german_source(), false);
        app.rotation_settings.path = Some(path.clone());
        app.rotation_selection = crate::rotation::Selection::new(vec![2, 4, 8, 12]).unwrap();
        app.password.edited();
        let output = controls_frame(&mut app, &ctx, vec![]);
        let pos = caption_rect(&output, "Profil bei jeder Generierung wechseln").center();
        let output = click_control(&mut app, &ctx, pos);
        assert!(app.rotate_profiles);
        let pos = caption_rect(&output, "8").center();
        let _ = click_control(&mut app, &ctx, pos);
        assert_eq!(app.rotation_selection.ids(), [2, 4, 12]);
        let mut restarted =
            KairosApp::new(&egui::Context::default(), crate::german_source(), false);
        // Exercise the same loader invoked by connect_startup, without process-
        // global env changes or access to the user's actual preference files.
        restarted.load_rotation_settings(crate::rotation::SettingsFile {
            path: Some(path),
            notice: None,
        });
        assert!(restarted.rotate_profiles);
        assert_eq!(restarted.rotation_selection.ids(), [2, 4, 12]);
    }

    #[test]
    fn info_inside_profile_picker_opens_the_matching_help_without_selecting_it() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), false);
        let output = controls_frame(&mut app, &ctx, vec![]);
        let caption = format!("0 · {}", help::PROFILES[0].name);
        let pos = caption_rect(&output, &caption).center();
        let output = click_control(&mut app, &ctx, pos);
        let row = caption_rect(&output, &format!("2 · {}", help::PROFILES[2].name));
        let icon = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) if text.galley.text() == "i" => {
                    let center = shape.shape.visual_bounding_rect().center();
                    (center.x > row.right() && (center.y - row.center().y).abs() < 10.0)
                        .then_some(center)
                }
                _ => None,
            })
            .expect("info for profile 2");
        let _ = click_control(&mut app, &ctx, icon);
        assert!(app.show_profiles);
        assert_eq!(app.help_profile, 2);
        assert_eq!(app.config.profile.get(), 0);
    }

    #[test]
    fn deviation_reasons_only_include_effective_separator_exclusions() {
        let mut app = KairosApp::new(&egui::Context::default(), crate::german_source(), false);
        app.config.excluded_chars = vec!['!'];
        assert!(!app.deviation_reason(0).contains("Trennerausschlüsse"));
        assert!(!app.deviation_reason(2).contains("Trennerausschlüsse"));
        assert!(app.deviation_reason(4).contains("Trennerausschlüsse"));
        app.blocked.words.insert("zzzzzzzzz".into());
        app.refresh_profile_bits();
        assert!(!app.deviation_reason(0).contains("Sperrliste"));
        app.blocked.words.insert(app.source.word(0).to_owned());
        app.refresh_profile_bits();
        assert!(app.deviation_reason(0).contains("Sperrliste"));
    }

    #[test]
    fn strength_card_distinguishes_concept_real_comparison_and_invalidated_edits() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), true);
        app.config.profile = ProfileId::new(0).unwrap();
        app.password.assessment = Assessment::Generated(8.0);
        let output = card(&app, &ctx);
        assert!(output.contains("Theoretisch (Konzept): 51,1 Bit"));
        assert!(output.contains("Tatsächlich: 8,0 Bit"));
        assert!(output.contains("4 zufällige Zeichen"));
        assert!(output.contains("1,28 × 10²"));
        app.password.edited();
        let output = card(&app, &ctx);
        assert!(output.contains("bearbeitet – nicht mehr bewertbar"));
        assert!(!output.contains("G50-Modell:") && !output.contains("Tatsächlich: 8,0"));
    }

    #[test]
    fn bonus_display_uses_core_metadata_and_disappears_after_editing() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), true);
        app.rotate_profiles = true;
        app.rotation_selection = crate::rotation::Selection::new(vec![2, 4, 8, 12]).unwrap();
        app.password.assessment = Assessment::Generated(64.0);
        app.rotation_bonus = Some(1.5); // A future core result must not be replaced by a fixed zero.
        let output = card(&app, &ctx);
        assert!(output.contains("64,4 Bit · Minimum der Auswahl"));
        assert!(output.contains("Profilbonus: +1,5 Bit (Core)"));
        app.password.edited();
        let output = card(&app, &ctx);
        assert!(!output.contains("Profilbonus:"));
    }

    #[test]
    fn per_profile_help_assessments_follow_filtered_core_with_override_and_exclusions() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), false);
        app.german_only = true;
        app.blocked.words.insert("ahorn".into());
        app.config.excluded_chars = vec!['!'];
        app.refresh_profile_bits();
        let filtered = source::FilteredSource::new(app.source.as_ref(), &app.blocked.words);
        for id in 0..=12 {
            let config = GeneratorConfig {
                profile: ProfileId::new(id).unwrap(),
                excluded_chars: vec!['!'],
            };
            assert_eq!(
                app.profile_bits[id as usize],
                Some(kairos_core::entropy_bits_for(&filtered, &config).unwrap())
            );
        }
        app.config.excluded_chars.push('-');
        app.refresh_profile_bits();
        assert_eq!(app.profile_bits[0], None);
        assert_eq!(app.profile_bits[1], None);
        assert!(app.profile_bits[2].is_some());
    }

    #[test]
    fn profile_help_opens_with_rule_reference_and_real_assessment() {
        let ctx = egui::Context::default();
        let mut app = KairosApp::new(&ctx, crate::german_source(), false);
        app.help_profile = 12;
        app.show_profiles = true;
        let render = |app: &mut KairosApp| {
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 1050.0),
                    )),
                    ..Default::default()
                },
                |ui| app.profiles_dialog(ui.ctx()),
            );
            output.textures_delta.clear();
            let mut result = String::new();
            for shape in output.shapes {
                text(&shape.shape, &mut result);
            }
            result
        };
        let _ = render(&mut app);
        let visible = render(&mut app);
        assert!(visible.contains("Profile erklärt"));
        assert!(visible.contains("153,1"));
        assert!(visible.contains("Tatsächlich (aktuelle Einstellungen)"));
        assert!(visible.contains("Öffentliches Lehrbeispiel"));
        assert!(visible.contains(help::PRINCIPLE));
        assert_eq!(visible.matches(help::PRINCIPLE).count(), 1);
        assert_eq!(
            visible
                .matches("Profil 12 · Zwei Trenner je Wortpaar")
                .count(),
            1
        );
    }
}
