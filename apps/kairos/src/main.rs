//! Native entry point. The German word source is connected in the integration PR.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples/hello_world

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kairos_core::{Language, WordSource};

struct MissingWordSource;

impl WordSource for MissingWordSource {
    fn len(&self) -> usize {
        0
    }
    fn word(&self, _index: usize) -> &str {
        unreachable!("word source is not yet connected")
    }
    fn language(&self, _index: usize) -> Language {
        unreachable!("word source is not yet connected")
    }
}

fn main() -> eframe::Result {
    let compact = std::env::args().any(|argument| argument == "--popup");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(if compact {
                [720.0, 560.0]
            } else {
                [860.0, 820.0]
            })
            .with_min_inner_size([550.0, 360.0])
            .with_title("Kairos Passwort"),
        persist_window: false,
        clipboard_shortcuts: false,
        ..Default::default()
    };
    eframe::run_native(
        "Kairos Passwort",
        options,
        Box::new(move |creation| {
            Ok(Box::new(kairos::KairosApp::new(
                &creation.egui_ctx,
                Box::new(MissingWordSource),
                compact,
            )))
        }),
    )
}
