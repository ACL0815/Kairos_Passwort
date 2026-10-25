//! Dummy-only demonstration; the real editor and KeePassXC bridge live in the app.
//!
//! Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/egui.rs>.

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use kairos_platform::{copy_secret, Action, GlobalHotkeys, HotkeyBinding};
    use std::{
        ptr, thread,
        time::{Duration, Instant},
    };
    use windows_sys::{
        core::w,
        Win32::UI::WindowsAndMessaging::{MessageBoxW, IDOK, MB_OKCANCEL},
    };

    const DUMMY: &str = "Kairos-Demo+Äpfel7";
    let seconds = std::env::args()
        .nth(1)
        .map(|v| v.parse::<u64>())
        .transpose()?
        .unwrap_or(120);
    let hotkeys = GlobalHotkeys::new(&HotkeyBinding::defaults())?;
    println!("Dummy-Demo für {seconds} Sekunden. Strg+Umschalt+F9: Popup, F10: Tippen+Kopieren, F11: Tippen+PR-6-Hook.");
    println!("Ein leeres Testfeld fokussieren. Die Demo sendet niemals Enter.");
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(seconds) {
        match hotkeys.try_recv() {
            Ok(Some(event)) => {
                let result = match event.action {
                    Action::OpenEditor => {
                        // A native dummy popup demonstrates focus capture and
                        // return. Editing/regeneration belongs to GUI PR-3.
                        let answer = unsafe {
                            MessageBoxW(ptr::null_mut(),
                            w!("Fester Dummy-Text: Kairos-Demo+Äpfel7\nOK: geschützt kopieren und zurückspringen.\nDer echte Editor folgt in der App."),
                            w!("Kairos – OpenEditor-Demo"), MB_OKCANCEL)
                        };
                        let copied = if answer == IDOK {
                            copy_secret(DUMMY, Some(Duration::from_secs(15))).map(|_| ())
                        } else {
                            Ok(())
                        };
                        let restored = match event.target {
                            Ok(target) => target.restore(),
                            Err(_) => Ok(()),
                        };
                        copied.and(restored)
                    }
                    Action::GenerateTypeCopy => event
                        .target
                        .and_then(|target| target.type_text(DUMMY))
                        .and_then(|_| {
                            copy_secret(DUMMY, Some(Duration::from_secs(15))).map(|_| ())
                        }),
                    Action::GenerateTypeKeePassXC => event
                        .target
                        .and_then(|target| target.type_text(DUMMY))
                        .map(|_| {
                            println!("PR-6-Hook erreicht; diese Demo kontaktiert KeePassXC nicht.");
                        }),
                };
                if let Err(error) = result {
                    eprintln!("Aktion abgebrochen: {error}");
                }
            }
            Ok(None) => {}
            Err(error) => {
                eprintln!("Hotkey-Dienst: {error}");
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    eprintln!("Die Hotkey-Demo benötigt Windows.");
    std::process::exit(1);
}
