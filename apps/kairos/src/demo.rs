//! Public screenshot fixture, excluded from all release builds.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples

use kairos_core::{GeneratedPassword, Segment, SegmentKind};
use zeroize::Zeroizing;

pub fn configure(app: &mut crate::KairosApp) {
    let id = std::env::var("KAIROS_DEMO_PROFILE")
        .ok()
        .and_then(|value| value.parse::<u8>().ok())
        .and_then(|id| kairos_core::ProfileId::new(id).ok())
        .unwrap_or_else(|| kairos_core::ProfileId::new(8).expect("demo profile"));
    app.config.profile = id;
    app.help_profile = id.get();
    app.show_profiles = std::env::var_os("KAIROS_DEMO_HELP").is_some();
}

/// egui floating windows need a sizing frame before painting. Capture only
/// after several real frames; eframe's immediate two-pass export can miss them.
pub fn capture(ctx: &eframe::egui::Context) {
    use eframe::egui;
    let Some(path) = std::env::var_os("KAIROS_DEMO_CAPTURE_TO") else {
        return;
    };
    let frames = ctx.data_mut(|data| {
        let counter = data.get_temp_mut_or_default::<u32>(egui::Id::unique("demo_capture_frames"));
        *counter += 1;
        *counter
    });
    if frames == 5 {
        let done = ctx.clone();
        ctx.request_screenshot(move |image| {
            let pixels: Vec<u8> = image
                .pixels
                .iter()
                .flat_map(|pixel| pixel.to_array())
                .collect();
            image::save_buffer(
                path,
                &pixels,
                image.size[0] as u32,
                image.size[1] as u32,
                image::ColorType::Rgba8,
            )
            .expect("public demo screenshot could not be saved");
            done.send_viewport_cmd(egui::ViewportCommand::Close);
        });
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(30));
}

pub fn public_example(result: GeneratedPassword, profile: u8) -> GeneratedPassword {
    let text = crate::help::PROFILES[profile as usize].example.to_owned();
    let segments = text
        .char_indices()
        .map(|(start, character)| Segment {
            range: start..start + character.len_utf8(),
            kind: if !character.is_alphanumeric() {
                SegmentKind::Separator
            } else if character.is_ascii_digit() {
                SegmentKind::Digit
            } else if character.is_uppercase() {
                SegmentKind::Uppercase
            } else {
                SegmentKind::Word
            },
        })
        .collect();
    GeneratedPassword {
        secret: Zeroizing::new(text),
        segments,
        entropy_bits: result.entropy_bits,
    }
}
