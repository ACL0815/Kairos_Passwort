//! Validated hotkeys, a dedicated message pump and a bounded event channel.
//!
//! Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/egui.rs>.

use crate::{FocusTarget, PlatformError};
use keyboard_types::{Code, Modifiers};
use std::str::FromStr;
#[cfg(any(windows, test))]
use std::sync::Mutex;
#[cfg(any(windows, test))]
use std::time::Instant;

/// App-owned actions; this crate neither generates nor contacts KeePassXC.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    OpenEditor,
    GenerateTypeCopy,
    GenerateTypeKeePassXC,
}

/// Keys use `Code` names (`KeyQ`, `F9`). Printable Ctrl+Alt chords are rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hotkey {
    pub(crate) modifiers: Modifiers,
    pub(crate) key: Code,
}

impl Hotkey {
    pub fn modifiers(self) -> Modifiers {
        self.modifiers
    }
    pub fn key(self) -> Code {
        self.key
    }
}

impl FromStr for Hotkey {
    type Err = PlatformError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split('+').map(str::trim).peekable();
        let mut modifiers = Modifiers::empty();
        let mut key = None;
        while let Some(token) = tokens.next() {
            if tokens.peek().is_none() {
                key = Some(
                    token
                        .parse::<Code>()
                        .map_err(|_| PlatformError::InvalidHotkey)?,
                );
            } else {
                let modifier = match token.to_ascii_lowercase().as_str() {
                    "ctrl" | "control" => Modifiers::CONTROL,
                    "shift" => Modifiers::SHIFT,
                    "alt" => Modifiers::ALT,
                    _ => return Err(PlatformError::InvalidHotkey),
                };
                if modifiers.contains(modifier) {
                    return Err(PlatformError::InvalidHotkey);
                }
                modifiers |= modifier;
            }
        }
        let key = key.ok_or(PlatformError::InvalidHotkey)?;
        let name = key.to_string();
        let printable = (name.len() == 4 && name.starts_with("Key"))
            || (name.len() == 6 && name.starts_with("Digit"));
        let function = name.strip_prefix('F').and_then(|n| n.parse::<u8>().ok());
        if !(printable || function.is_some_and(|n| (1..=24).contains(&n)))
            || key == Code::F12 // Reserved by Windows for a debugger.
            || modifiers.bits().count_ones() < 2
            || (key == Code::F4 && modifiers == Modifiers::ALT)
        {
            return Err(PlatformError::InvalidHotkey);
        }
        if printable && modifiers.contains(Modifiers::CONTROL | Modifiers::ALT) {
            return Err(PlatformError::AltGrConflict);
        }
        Ok(Self { modifiers, key })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotkeyBinding {
    pub action: Action,
    pub hotkey: Hotkey,
}

impl HotkeyBinding {
    /// Avoids AltGr and reserved Win chords. OS registration still checks for
    /// conflicts with other running applications.
    pub fn defaults() -> [Self; 3] {
        [
            (Action::OpenEditor, Code::F9),
            (Action::GenerateTypeCopy, Code::F10),
            (Action::GenerateTypeKeePassXC, Code::F11),
        ]
        .map(|(action, key)| Self {
            action,
            hotkey: Hotkey {
                modifiers: Modifiers::CONTROL | Modifiers::SHIFT,
                key,
            },
        })
    }
}

/// Checks local conflicts before starting a thread or registering OS hotkeys.
pub fn validate_hotkeys(bindings: &[HotkeyBinding]) -> Result<(), PlatformError> {
    if bindings.is_empty() || bindings.len() > 3 {
        return Err(PlatformError::InvalidHotkey);
    }
    for (i, binding) in bindings.iter().enumerate() {
        for earlier in &bindings[..i] {
            if binding.hotkey == earlier.hotkey {
                return Err(PlatformError::DuplicateHotkey);
            }
            if binding.action == earlier.action {
                return Err(PlatformError::DuplicateAction);
            }
        }
    }
    Ok(())
}

/// Captured on the message thread before notifying the GUI.
pub struct HotkeyEvent {
    pub action: Action,
    /// OpenEditor remains available even when no input target can be captured.
    pub target: Result<FocusTarget, PlatformError>,
    #[cfg(any(windows, test))]
    pub(crate) queued: Instant,
}

#[cfg(any(windows, test))]
pub(crate) struct EventSlot(Mutex<Option<HotkeyEvent>>);

#[cfg(any(windows, test))]
impl EventSlot {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(None))
    }
    pub(crate) fn push(&self, event: HotkeyEvent) -> Result<(), PlatformError> {
        *self
            .0
            .lock()
            .map_err(|_| PlatformError::WorkerUnavailable)? = Some(event);
        Ok(())
    }
    pub(crate) fn take(&self) -> Result<Option<HotkeyEvent>, PlatformError> {
        let event = self
            .0
            .lock()
            .map_err(|_| PlatformError::WorkerUnavailable)?
            .take();
        Ok(event.filter(|event| !crate::focus::expired(event.queued.elapsed())))
    }
}

impl Hotkey {
    #[cfg(any(windows, test))]
    pub(crate) fn virtual_key(self) -> Option<u16> {
        let name = self.key.to_string();
        if let Some(letter) = name.strip_prefix("Key").and_then(|s| s.bytes().next()) {
            return Some(letter as u16);
        }
        if let Some(digit) = name.strip_prefix("Digit").and_then(|s| s.bytes().next()) {
            return Some(digit as u16);
        }
        name.strip_prefix('F')
            .and_then(|s| s.parse::<u16>().ok())
            .filter(|n| (1..=24).contains(n))
            .map(|n| 0x70 + n - 1)
    }
}

/// Drop unregisters and joins the message thread. Only one instance per process
/// is allowed. Reserve `global-hotkey`'s process-global receiver for this service;
/// do not install a handler or consume its receiver elsewhere in the app.
pub struct GlobalHotkeys {
    #[cfg(windows)]
    inner: crate::windows::hotkeys::Service,
}

impl GlobalHotkeys {
    pub fn new(bindings: &[HotkeyBinding]) -> Result<Self, PlatformError> {
        Self::with_wakeup(bindings, || {})
    }

    /// Callback runs on the message thread when an event is queued. Eframe can
    /// pass `move || ctx.request_repaint()`. It must not block or panic. At most
    /// one event is pending; the newest press replaces the old event. Events
    /// older than five seconds are discarded before the UI opens a popup.
    pub fn with_wakeup(
        bindings: &[HotkeyBinding],
        wakeup: impl Fn() + Send + 'static,
    ) -> Result<Self, PlatformError> {
        validate_hotkeys(bindings)?;
        #[cfg(windows)]
        return crate::windows::hotkeys::Service::start(bindings, wakeup)
            .map(|inner| Self { inner });
        #[cfg(not(windows))]
        {
            let _ = wakeup;
            Err(PlatformError::UnsupportedPlatform)
        }
    }

    /// Nonblocking; no ownership of the GUI event loop is needed.
    pub fn try_recv(&self) -> Result<Option<HotkeyEvent>, PlatformError> {
        #[cfg(windows)]
        return self.inner.try_recv();
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_code_names_and_modifier_aliases() {
        assert_eq!(
            "Ctrl+Shift+F9".parse::<Hotkey>(),
            "shift + control + F9".parse()
        );
        assert_eq!(
            "Ctrl+Shift+KeyQ".parse::<Hotkey>().unwrap().key(),
            Code::KeyQ
        );
        assert_eq!(
            "Alt+Shift+Digit1".parse::<Hotkey>().unwrap().modifiers(),
            Modifiers::ALT | Modifiers::SHIFT
        );
    }
    #[test]
    fn rejects_altgr_printable_chords_including_shift() {
        for chord in ["Ctrl+Alt+KeyQ", "Ctrl+Alt+Digit7", "Ctrl+Alt+Shift+KeyE"] {
            assert_eq!(chord.parse::<Hotkey>(), Err(PlatformError::AltGrConflict));
        }
        assert!("Ctrl+Alt+F9".parse::<Hotkey>().is_ok());
    }
    #[test]
    fn rejects_malformed_and_unsafe_chords() {
        for chord in [
            "",
            "KeyQ",
            "Shift+KeyQ",
            "Ctrl++KeyQ",
            "Ctrl+Ctrl+F9",
            "Ctrl+F9+Shift",
            "Win+KeyQ",
            "Alt+F4",
            "Ctrl+F12",
            "Ctrl+Enter",
            "Ctrl+garbage",
            "Ctrl+Shift+KeyboardLayoutSelect",
            "Ctrl+KeyC",
            "Ctrl+KeyV",
            "Ctrl+KeyX",
            "Ctrl+KeyZ",
            "Ctrl+KeyA",
            "Alt+KeyE",
        ] {
            assert_eq!(
                chord.parse::<Hotkey>(),
                Err(PlatformError::InvalidHotkey),
                "{chord}"
            );
        }
    }
    #[test]
    fn defaults_have_no_local_conflicts_or_altgr_chords() {
        let defaults = HotkeyBinding::defaults();
        assert_eq!(validate_hotkeys(&defaults), Ok(()));
        for binding in defaults {
            assert!(!binding.hotkey.modifiers().contains(Modifiers::ALT));
        }
    }
    #[test]
    fn detects_duplicate_chords_and_actions() {
        let mut bindings = HotkeyBinding::defaults();
        bindings[1].hotkey = bindings[0].hotkey;
        assert_eq!(
            validate_hotkeys(&bindings),
            Err(PlatformError::DuplicateHotkey)
        );
        bindings = HotkeyBinding::defaults();
        bindings[1].action = bindings[0].action;
        assert_eq!(
            validate_hotkeys(&bindings),
            Err(PlatformError::DuplicateAction)
        );
        assert_eq!(validate_hotkeys(&[]), Err(PlatformError::InvalidHotkey));
    }

    #[test]
    fn maps_configurable_main_keys_for_release_waiting() {
        for (chord, vk) in [
            ("Ctrl+Shift+KeyQ", 0x51),
            ("Ctrl+Shift+Digit9", 0x39),
            ("Ctrl+Shift+F9", 0x78),
        ] {
            assert_eq!(chord.parse::<Hotkey>().unwrap().virtual_key(), Some(vk));
        }
    }

    #[test]
    fn capture_failure_keeps_the_action_and_latest_press_wins() {
        let slot = EventSlot::new();
        for action in [Action::GenerateTypeCopy, Action::OpenEditor] {
            slot.push(HotkeyEvent {
                action,
                target: Err(PlatformError::TargetUnavailable),
                queued: Instant::now(),
            })
            .unwrap();
        }
        let event = slot.take().unwrap().unwrap();
        assert_eq!(event.action, Action::OpenEditor);
        assert!(matches!(
            event.target,
            Err(PlatformError::TargetUnavailable)
        ));
        assert!(slot.take().unwrap().is_none());
    }

    #[test]
    fn stale_events_do_not_open_an_editor_for_an_old_window() {
        let slot = EventSlot::new();
        slot.push(HotkeyEvent {
            action: Action::OpenEditor,
            target: Err(PlatformError::TargetUnavailable),
            queued: Instant::now() - std::time::Duration::from_secs(6),
        })
        .unwrap();
        assert!(slot.take().unwrap().is_none());
    }
}
