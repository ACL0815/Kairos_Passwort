//! Native palette and password typography.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/crates/egui_demo_lib/src/demo
//! The palette follows Kairos UI-STYLE.md; no third-party theme code is copied.

use eframe::egui::{self, Color32, FontFamily, FontId, Stroke, Vec2};

pub const BACKGROUND: Color32 = Color32::from_rgb(10, 16, 48);
pub const PANEL: Color32 = Color32::from_rgb(17, 28, 60);
pub const TEXT: Color32 = Color32::from_rgb(234, 242, 255);
pub const MUTED: Color32 = Color32::from_rgb(167, 187, 219);
pub const CYAN: Color32 = Color32::from_rgb(34, 211, 238);

/// Installs the fixed dark palette and a ligature-free password font.
///
/// Modelled after: JetBrains Mono https://github.com/JetBrains/JetBrainsMono/blob/v2.304/README.md
pub fn install(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "password".into(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/JetBrainsMonoNL-Regular.ttf"
        ))
        .into(),
    );
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "password".into());
    ctx.set_fonts(fonts);
    let mut style = egui::Style {
        visuals: egui::Visuals::dark(),
        ..Default::default()
    };
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.panel_fill = BACKGROUND;
    style.visuals.window_fill = PANEL;
    style.visuals.extreme_bg_color = Color32::from_rgb(8, 14, 35);
    style.visuals.selection.bg_fill = Color32::from_rgb(34, 75, 119);
    style.visuals.selection.stroke = Stroke::new(1.5, CYAN);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(26, 42, 76);
    style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(26, 42, 76);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(70, 100, 145));
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(35, 61, 100);
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(35, 61, 100);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, CYAN);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(34, 75, 119);
    style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(34, 75, 119);
    style.visuals.widgets.active.bg_stroke = Stroke::new(2.0, CYAN);
    style.spacing.item_spacing = Vec2::new(10.0, 12.0);
    style.spacing.button_padding = Vec2::new(16.0, 10.0);
    style.spacing.interact_size.y = 36.0;
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, FontId::proportional(28.0));
    ctx.set_style_of(egui::Theme::Dark, style);
}

pub fn panel() -> egui::Frame {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(120, 191, 255, 65),
        ))
        .corner_radius(16)
        .inner_margin(22)
}

pub fn strength_ring(ui: &mut egui::Ui, bits: Option<f64>) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(112.0), egui::Sense::hover());
    let center = rect.center();
    let radius = 46.0;
    ui.painter().circle_stroke(
        center,
        radius,
        Stroke::new(7.0, Color32::from_rgb(32, 44, 78)),
    );
    if let Some(bits) = bits {
        let fraction = (bits / 160.0).clamp(0.0, 1.0) as f32;
        for index in 0..80 {
            let t = index as f32 / 80.0;
            if t > fraction {
                break;
            }
            let start = -std::f32::consts::FRAC_PI_2 + t * std::f32::consts::TAU;
            let end = start + std::f32::consts::TAU / 80.0;
            let (from, to, blend) = if t < 0.5 {
                ([139.0, 92.0, 246.0], [236.0, 72.0, 153.0], t * 2.0)
            } else {
                ([236.0, 72.0, 153.0], [251.0, 146.0, 60.0], (t - 0.5) * 2.0)
            };
            let color =
                std::array::from_fn::<_, 3, _>(|i| (from[i] + (to[i] - from[i]) * blend) as u8);
            ui.painter().line_segment(
                [
                    center + Vec2::angled(start) * radius,
                    center + Vec2::angled(end) * radius,
                ],
                Stroke::new(7.0, Color32::from_rgb(color[0], color[1], color[2])),
            );
        }
    }
    ui.painter().text(
        center,
        egui::Align2::CENTER_CENTER,
        bits.map_or_else(|| "?".into(), |bits| format!("{bits:.0}")),
        FontId::proportional(27.0),
        TEXT,
    );
    ui.painter().text(
        center + Vec2::new(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        "Bit",
        FontId::proportional(12.0),
        MUTED,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_navy_style_overrides_a_light_system_preference() {
        let ctx = egui::Context::default();
        ctx.set_theme(egui::Theme::Light);
        install(&ctx);
        assert_eq!(ctx.theme(), egui::Theme::Dark);
        assert_eq!(
            ctx.global_style().visuals.extreme_bg_color,
            Color32::from_rgb(8, 14, 35)
        );
        assert_eq!(ctx.global_style().visuals.override_text_color, Some(TEXT));
    }
}
