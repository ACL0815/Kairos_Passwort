//! Persistent capture warnings, outside the scrolling password editor.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp>.

use eframe::egui::{self, Color32, RichText};
use kairos_platform::CaptureStatus;

pub(crate) fn banner(ui: &mut egui::Ui, status: CaptureStatus) {
    let (headline, detail) = match status {
        CaptureStatus::Allowed => (
            "Bildschirmaufnahme erlaubt",
            "Mit --allow-screencapture gestartet. Passwörter können in Freigaben und Aufnahmen sichtbar sein.",
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
    egui::Panel::top("capture-warning")
        .frame(
            egui::Frame::new()
                .fill(Color32::from_rgb(84, 43, 15))
                .inner_margin(10),
        )
        .show(ui, |ui| {
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
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(550.0, 360.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    banner(ui, status);
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
            banner(ui, CaptureStatus::Excluded)
        });
        output.textures_delta.clear();
        assert!(!output
            .shapes
            .iter()
            .any(|shape| contains_text(&shape.shape, "Schutz nicht verfügbar")));
    }
}
