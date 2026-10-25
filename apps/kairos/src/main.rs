//! Native entry point with the embedded German word source.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples/hello_world

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    let target = kairos_platform::FocusTarget::capture();
    let compact = std::env::args().any(|argument| argument == "--popup");
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(if compact {
                [720.0, 640.0]
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
            let mut app =
                kairos::KairosApp::new(&creation.egui_ctx, kairos::german_source(), compact);
            app.connect_desktop(&creation.egui_ctx, target);
            Ok(Box::new(app))
        }),
    )
}
