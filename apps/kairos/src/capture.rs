//! Persistent capture warnings, outside the scrolling password editor.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp>.

use eframe::egui::{self, Color32, RichText};
use kairos_platform::{CapturePolicy, CaptureStatus};

pub(crate) fn banner(ui: &mut egui::Ui, status: CaptureStatus, policy: CapturePolicy) {
    let (headline, detail) = match status {
        CaptureStatus::Allowed => (
            "Bildschirmaufnahme erlaubt",
            "Mit --allow-screencapture gestartet. Passwörter können in Freigaben und Aufnahmen sichtbar sein.",
        ),
        CaptureStatus::Unavailable(_) if policy == CapturePolicy::Allow => (
            "Bildschirmaufnahme erlaubt",
            "Mit --allow-screencapture gestartet. Windows konnte die Freigabe nicht bestätigen; Aufnahmen können weiterhin eingeschränkt sein.",
        ),
        CaptureStatus::Unavailable(_) | CaptureStatus::Pending => (
            "Schutz nicht verfügbar",
            "Der Schutz konnte nicht für alle Fenster bestätigt werden. Passwörter können in Freigaben und Aufnahmen sichtbar sein.",
        ),
        CaptureStatus::MonitorOnly => (
            "Eingeschränkter Aufnahmeschutz",
            "Windows meldet nur Inhaltsschutz (WDA_MONITOR); die Fensterfläche kann in Aufnahmen sichtbar bleiben.",
        ),
        CaptureStatus::Excluded => return,
    };
    egui::Frame::new()
        .fill(Color32::from_rgb(84, 43, 15))
        .inner_margin(10)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                RichText::new(headline)
                    .strong()
                    .color(Color32::from_rgb(255, 209, 128)),
            );
            ui.label(RichText::new(detail).color(Color32::WHITE));
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use kairos_platform::CaptureError;

    fn contains_text(shape: &egui::epaint::Shape, expected: &str) -> bool {
        match shape {
            egui::epaint::Shape::Text(text) => text.galley.text().contains(expected),
            egui::epaint::Shape::Vec(shapes) => {
                shapes.iter().any(|shape| contains_text(shape, expected))
            }
            _ => false,
        }
    }

    #[test]
    fn app_keeps_capture_warning_visible_in_editor_and_f11_dialog() {
        for allow in [false, true] {
            for f11 in [false, true] {
                let ctx = egui::Context::default();
                let mut app = crate::KairosApp::new(&ctx, crate::german_source(), true);
                if allow {
                    app.connect_capture_protection(kairos_platform::CaptureProtection::new(
                        CapturePolicy::Allow,
                    ));
                }
                if f11 {
                    app.keepass.flow = Some(crate::keepass::Flow::new(None, None, None));
                }
                let mut output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(550.0, 360.0),
                        )),
                        ..Default::default()
                    },
                    |ui| app.draw(ui),
                );
                output.textures_delta.clear();
                let expected = if allow {
                    "Bildschirmaufnahme erlaubt"
                } else {
                    "Schutz nicht verfügbar"
                };
                let warning = output
                    .shapes
                    .iter()
                    .find(|shape| contains_text(&shape.shape, expected))
                    .expect("the capture warning must survive the F11 branch");
                assert!(warning
                    .clip_rect
                    .contains_rect(warning.shape.visual_bounding_rect()));
                assert!(warning.shape.visual_bounding_rect().bottom() < 120.0);
                let content = if f11 {
                    "KeePassXC · Eintrag bestätigen"
                } else {
                    "Dein nächstes Passwort."
                };
                assert!(output
                    .shapes
                    .iter()
                    .any(|shape| contains_text(&shape.shape, content)));
            }
        }
    }

    #[test]
    fn warnings_are_visible_above_the_editor_including_minimum_popup_height() {
        for (status, expected) in [
            (CaptureStatus::Allowed, "Bildschirmaufnahme erlaubt"),
            (CaptureStatus::Pending, "Schutz nicht verfügbar"),
            (
                CaptureStatus::Unavailable(CaptureError::SetAffinityFailed(5)),
                "Schutz nicht verfügbar",
            ),
            (CaptureStatus::MonitorOnly, "Eingeschränkter Aufnahmeschutz"),
        ] {
            let ctx = egui::Context::default();
            crate::theme::install(&ctx);
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(550.0, 360.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    banner(ui, status, CapturePolicy::Protect);
                    egui::CentralPanel::default().show(ui, |ui| {
                        ui.label("PUBLIC PASSWORD EDITOR");
                    });
                },
            );
            output.textures_delta.clear();
            let warning = output
                .shapes
                .iter()
                .find(|shape| contains_text(&shape.shape, expected))
                .unwrap();
            assert!(warning.shape.visual_bounding_rect().bottom() < 120.0);
            for text in output.shapes.iter().filter(|shape| {
                contains_text(&shape.shape, expected)
                    || contains_text(&shape.shape, "Passwörter können")
                    || contains_text(&shape.shape, "Windows meldet nur")
            }) {
                assert!(
                    text.clip_rect
                        .contains_rect(text.shape.visual_bounding_rect()),
                    "the entire warning must be visible with the real app theme: {status:?}"
                );
            }
            assert!(output
                .shapes
                .iter()
                .any(|shape| contains_text(&shape.shape, "PUBLIC PASSWORD EDITOR")));
        }
    }

    #[test]
    fn confirmed_exclusion_does_not_render_a_warning() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            banner(ui, CaptureStatus::Excluded, CapturePolicy::Protect)
        });
        output.textures_delta.clear();
        assert!(!output
            .shapes
            .iter()
            .any(|shape| contains_text(&shape.shape, "Schutz nicht verfügbar")));
    }

    #[test]
    fn failed_opt_out_is_described_as_unconfirmed_permission() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            banner(
                ui,
                CaptureStatus::Unavailable(CaptureError::SetAffinityFailed(5)),
                CapturePolicy::Allow,
            );
        });
        output.textures_delta.clear();
        assert!(output
            .shapes
            .iter()
            .any(|shape| contains_text(&shape.shape, "Bildschirmaufnahme erlaubt")));
        assert!(output
            .shapes
            .iter()
            .any(|shape| contains_text(&shape.shape, "Freigabe nicht bestätigen")));
        assert!(!output
            .shapes
            .iter()
            .any(|shape| contains_text(&shape.shape, "Schutz nicht verfügbar")));
    }
}
