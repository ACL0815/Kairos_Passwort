//! Windows services triggered by explicit user actions; no generator or GUI dependency.
//!
//! Capture a [`FocusTarget`] before showing a popup. Use that same target for
//! restoration or autotype; never recapture after opening the popup.
//! Poll [`GlobalHotkeys::try_recv`] from the GUI or use its wakeup callback.
//! No API logs or retains secret text. Unsupported platforms return an error.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp>
//! and global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/src/lib.rs>.

mod clipboard;
mod focus;
mod hotkeys;
#[cfg(windows)]
mod windows;

pub use clipboard::{
    copy_secret, ClearOutcome, ClipboardReceipt, DEFAULT_CLEAR_AFTER, MAX_CLEAR_AFTER,
};
pub use focus::FocusTarget;
pub use hotkeys::{validate_hotkeys, Action, GlobalHotkeys, Hotkey, HotkeyBinding, HotkeyEvent};
pub use keyboard_types::{Code, Modifiers};

use std::fmt;

/// A failure that never includes secret text or untrusted OS error strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlatformError {
    UnsupportedPlatform,
    InvalidHotkey,
    AltGrConflict,
    DuplicateHotkey,
    DuplicateAction,
    HotkeyUnavailable,
    HotkeysAlreadyRunning,
    WorkerUnavailable,
    ClipboardUnavailable,
    ClipboardProtectionFailed,
    InvalidTimeout,
    InvalidText,
    TargetUnavailable,
    TargetChanged,
    TargetExpired,
    ForegroundDenied,
    ModifiersHeld,
    InputIncomplete,
    TargetElevated,
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedPlatform => "this operation requires Windows",
            Self::InvalidHotkey => "invalid or unsafe hotkey combination",
            Self::AltGrConflict => "the hotkey may intercept AltGr text input",
            Self::DuplicateHotkey => "two actions use the same hotkey",
            Self::DuplicateAction => "an action has more than one hotkey",
            Self::HotkeyUnavailable => {
                "a hotkey could not be registered; choose another combination"
            }
            Self::HotkeysAlreadyRunning => "the process already has a hotkey service",
            Self::WorkerUnavailable => "the platform worker is unavailable",
            Self::ClipboardUnavailable => "the clipboard is unavailable",
            Self::ClipboardProtectionFailed => "clipboard protection could not be applied",
            Self::InvalidTimeout => "the clipboard timeout must not exceed ten minutes",
            Self::InvalidText => {
                "the text is empty, too long, or contains unsupported control characters"
            }
            Self::TargetUnavailable => "the target window or focused control is unavailable",
            Self::TargetChanged => "the foreground window or focused control has changed",
            Self::TargetExpired => "the autotype request has expired; invoke the hotkey again",
            Self::ForegroundDenied => "Windows did not allow foreground restoration",
            Self::ModifiersHeld => "the hotkey keys were not released in time",
            Self::TargetElevated => "autotype into a higher-integrity process is refused",
            Self::InputIncomplete => {
                "Windows did not accept all input; text may be partially entered"
            }
        })
    }
}

impl std::error::Error for PlatformError {}

/// Bounds allocation and rejects NUL truncation. Autotype also rejects controls
/// that could navigate to another field or submit a form.
#[cfg(any(windows, test))]
pub(crate) fn validate_text(text: &str, autotype: bool) -> Result<(), PlatformError> {
    if text.is_empty()
        || text.encode_utf16().count() > 1024
        || text.contains('\0')
        || (autotype && text.chars().any(char::is_control))
    {
        Err(PlatformError::InvalidText)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_unicode_without_layout_assumptions() {
        assert_eq!(validate_text("Äpfel+Öl#🦀", true), Ok(()));
        assert_eq!(validate_text(&"🦀".repeat(512), true), Ok(()));
        assert_eq!(
            validate_text(&"🦀".repeat(513), true),
            Err(PlatformError::InvalidText)
        );
    }

    #[test]
    fn rejects_truncation_and_field_navigation() {
        for text in ["", "a\0b", "a\tb", "a\rb", "a\nb", "a\u{1b}b", "a\u{7f}b"] {
            assert_eq!(validate_text(text, true), Err(PlatformError::InvalidText));
        }
        assert_eq!(validate_text("a\nb", false), Ok(()));
    }

    #[test]
    fn errors_never_echo_the_supplied_secret() {
        let error = validate_text("private\0sentinel", true).unwrap_err();
        assert!(!format!("{error:?}: {error}").contains("sentinel"));
    }
}
