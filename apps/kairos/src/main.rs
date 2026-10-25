//! Native entry point with the embedded German word source.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples/hello_world

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    let target = kairos_platform::FocusTarget::capture();
    let arguments = Arguments::parse(std::env::args_os().skip(1));
    let compact = arguments.compact;
    let capture = kairos_platform::CaptureProtection::new(arguments.capture_policy);
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
            let root = creation.egui_ctx.clone();
            capture.set_wakeup(move || root.request_repaint_of(eframe::egui::ViewportId::ROOT));
            capture.apply_to(creation);
            let mut app =
                kairos::KairosApp::new(&creation.egui_ctx, kairos::german_source(), compact);
            app.connect_capture_protection(capture);
            app.connect_desktop(&creation.egui_ctx, target);
            Ok(Box::new(app))
        }),
    )
}

/// Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp>.
#[derive(Default, Debug, Eq, PartialEq)]
struct Arguments {
    compact: bool,
    capture_policy: kairos_platform::CapturePolicy,
}

impl Arguments {
    fn parse(arguments: impl IntoIterator<Item = std::ffi::OsString>) -> Self {
        let mut result = Self::default();
        for argument in arguments {
            if argument == "--popup" {
                result.compact = true;
            } else if argument == "--allow-screencapture" {
                result.capture_policy = kairos_platform::CapturePolicy::Allow;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kairos_platform::CapturePolicy;

    #[test]
    fn capture_protection_is_default_including_popup() {
        assert_eq!(Arguments::parse([]), Arguments::default());
        let popup = Arguments::parse(["--popup".into()]);
        assert!(popup.compact);
        assert_eq!(popup.capture_policy, CapturePolicy::Protect);
    }

    #[test]
    fn explicit_opt_out_combines_with_popup_in_either_order() {
        for arguments in [
            ["--popup", "--allow-screencapture"],
            ["--allow-screencapture", "--popup"],
        ] {
            assert_eq!(
                Arguments::parse(arguments.map(Into::into)),
                Arguments {
                    compact: true,
                    capture_policy: CapturePolicy::Allow
                }
            );
        }
        assert_eq!(
            Arguments::parse(["--allow-screencapture".into()]).capture_policy,
            CapturePolicy::Allow
        );
    }

    #[test]
    fn similar_flags_do_not_silently_disable_protection() {
        for argument in [
            "--allow-screencapture=false",
            "--allow-screen-capture",
            "--ALLOW-SCREENCAPTURE",
        ] {
            assert_eq!(
                Arguments::parse([argument.into()]).capture_policy,
                CapturePolicy::Protect
            );
        }
    }
}
