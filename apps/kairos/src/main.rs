//! Early process hardening and native entry point with profile-dependent sources.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples/hello_world
//! Modelled after: KeePassXC https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hardening = kairos_hardening::harden_process();
    let target = kairos_platform::FocusTarget::capture();
    let arguments = Arguments::parse(std::env::args_os().skip(1));
    #[cfg(windows)]
    if arguments.remove_integration {
        use kairos_platform::integration::{remove_run, start_menu_path, Shortcut, RUN_VALUE};
        // Attempt both removals even if one fails. Never touch StartupApproved.
        let run = remove_run(RUN_VALUE);
        let shortcut = std::env::var_os("APPDATA")
            .ok_or_else(|| std::io::Error::other("APPDATA unavailable"))
            .and_then(|p| start_menu_path(&std::path::PathBuf::from(p)))
            .and_then(|p| Shortcut::remove(&p));
        run?;
        shortcut?;
        return Ok(());
    }
    #[cfg(windows)]
    let instance = {
        use kairos_platform::instance::{session_name, InstanceRequest, SingleInstance};
        let request = if arguments.compact {
            InstanceRequest::Popup
        } else {
            InstanceRequest::Show
        };
        let acquired = session_name().and_then(|name| SingleInstance::acquire(&name, request));
        let acquired = match acquired {
            Ok(instance) => instance,
            Err(error) => {
                // Fail closed instead of registering a duplicate set of hotkeys.
                // The release GUI subsystem has no stderr console to show this.
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
                        std::ptr::null_mut(),
                        windows_sys::core::w!("Kairos kann die Einzelinstanz nicht öffnen. Eine laufende Instanz beenden oder Kairos mit denselben Windows-Rechten erneut starten. Es wurden keine Hotkeys oder Integrationseinträge angelegt."),
                        windows_sys::core::w!("Kairos Passwort – Start nicht möglich"),
                        windows_sys::Win32::UI::WindowsAndMessaging::MB_OK | windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONERROR,
                    );
                }
                return Err(error.into());
            }
        };
        let Some(instance) = acquired else {
            return Ok(());
        };
        instance
    };
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
            .with_visible(!arguments.tray)
            .with_icon(kairos::icon_data())
            .with_title("Kairos Passwort"),
        persist_window: false,
        clipboard_shortcuts: false,
        window_builder: Some(Box::new(move |builder| {
            builder.with_active(!arguments.tray)
        })),
        ..Default::default()
    };
    #[cfg(all(feature = "demo", debug_assertions))]
    let options = if std::env::var_os("KAIROS_DEMO_LARGE").is_some() {
        eframe::NativeOptions {
            viewport: options.viewport.with_inner_size([1000.0, 1050.0]),
            ..options
        }
    } else {
        options
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
            app.connect_startup(&creation.egui_ctx, hardening);
            app.connect_capture_protection(capture);
            app.connect_desktop(&creation.egui_ctx, target);
            #[cfg(windows)]
            app.connect_lifecycle(creation, instance, arguments.tray);
            Ok(Box::new(app))
        }),
    )?;
    Ok(())
}

/// Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp>.
#[derive(Default, Debug, Eq, PartialEq)]
struct Arguments {
    compact: bool,
    tray: bool,
    remove_integration: bool,
    capture_policy: kairos_platform::CapturePolicy,
}

impl Arguments {
    fn parse(arguments: impl IntoIterator<Item = std::ffi::OsString>) -> Self {
        let mut result = Self::default();
        for argument in arguments {
            if argument == "--popup" {
                result.compact = true;
            } else if argument == "--tray" {
                result.tray = true;
            } else if argument == "--remove-integration" {
                result.remove_integration = true;
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
                    capture_policy: CapturePolicy::Allow,
                    ..Default::default()
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

    #[test]
    fn tray_cleanup_popup_and_capture_flags_combine_in_any_order() {
        let flags = [
            "--tray",
            "--remove-integration",
            "--popup",
            "--allow-screencapture",
        ];
        for mask in 0..16 {
            let chosen: Vec<_> = flags
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, flag)| std::ffi::OsString::from(flag))
                .collect();
            let parsed = Arguments::parse(chosen.clone());
            assert_eq!(parsed.tray, mask & 1 != 0);
            assert_eq!(parsed.remove_integration, mask & 2 != 0);
            assert_eq!(parsed.compact, mask & 4 != 0);
            assert_eq!(parsed.capture_policy == CapturePolicy::Allow, mask & 8 != 0);
            assert_eq!(Arguments::parse(chosen.into_iter().rev()), parsed);
        }
        for flag in ["--tray=false", "--remove-integration=false", "--TRAY"] {
            assert_eq!(Arguments::parse([flag.into()]), Arguments::default());
        }
    }
}
