//! Capture the destination before a popup can take focus.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/windows/AutoTypeWindows.cpp>.

use crate::PlatformError;
#[cfg(any(windows, test))]
use std::time::Duration;

#[cfg(any(windows, test))]
pub(crate) fn expired(age: Duration) -> bool {
    age > Duration::from_secs(5)
}

#[cfg(any(windows, test))]
pub(crate) fn release_ready(
    elapsed: Duration,
    released_for: Option<Duration>,
) -> Result<bool, PlatformError> {
    if elapsed >= Duration::from_secs(2) {
        return Err(PlatformError::ModifiersHeld);
    }
    Ok(released_for.is_some_and(|duration| duration >= Duration::from_millis(20)))
}

#[cfg(any(windows, test))]
pub(crate) fn release_index(sent: usize, total: usize) -> Option<usize> {
    (sent < total && sent % 2 == 1).then_some(sent)
}

#[cfg(any(windows, test))]
pub(crate) fn allow_integrity(own: u32, target: u32) -> Result<(), PlatformError> {
    if target > own {
        Err(PlatformError::TargetElevated)
    } else {
        Ok(())
    }
}

#[cfg(any(windows, test))]
pub(crate) fn restore_ready(
    elapsed: Duration,
    current: Result<(), PlatformError>,
) -> Result<bool, PlatformError> {
    match current {
        Ok(()) => Ok(true),
        Err(error) if elapsed >= Duration::from_millis(500) => Err(error),
        Err(_) => Ok(false),
    }
}

/// Opaque window/process/thread/control identity, containing no text.
/// Autotype expires after five seconds; restoration may be used later.
pub struct FocusTarget {
    #[cfg(windows)]
    pub(crate) snapshot: crate::windows::focus::Snapshot,
}

impl FocusTarget {
    /// Whether the captured destination belongs to this process. Applications
    /// can refuse to type their own generated secrets back into their editor.
    pub fn is_current_process(&self) -> bool {
        #[cfg(windows)]
        return self.snapshot.is_current_process();
        #[cfg(not(windows))]
        false
    }

    /// Captures the foreground window and its focused native control.
    pub fn capture() -> Result<Self, PlatformError> {
        #[cfg(windows)]
        return crate::windows::focus::capture().map(|snapshot| Self { snapshot });
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    /// Restore before destroying the app window, after copying or when closing
    /// is requested. Refuses to steal focus from
    /// an unrelated application; reports a denied `SetForegroundWindow` call.
    pub fn restore(&self) -> Result<(), PlatformError> {
        #[cfg(windows)]
        return self.snapshot.restore();
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    /// Explicit editor confirmation after potentially lengthy manual editing.
    /// Restores and revalidates the original window/process/thread/control, then
    /// starts a fresh five-second input deadline. Never captures a new target.
    /// Use only for a new user click on Insert, never for queued hotkey events.
    /// All `type_text` input, integrity, modifier and partial-input rules apply.
    ///
    /// Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp>.
    pub fn restore_and_type_text(&self, text: &str) -> Result<(), PlatformError> {
        #[cfg(windows)]
        {
            crate::validate_text(text, true)?;
            let confirmed = self.snapshot.confirm_from_editor()?;
            crate::windows::focus::type_text(&confirmed, text)
        }
        #[cfg(not(windows))]
        {
            let _ = text;
            Err(PlatformError::UnsupportedPlatform)
        }
    }

    /// Types literal Unicode into this same field, waiting at most two seconds
    /// for Ctrl/Alt/Shift/Win and the hotkey's main key to be released.
    /// No clipboard fallback or automatic retry. Higher-integrity targets fail.
    /// Call as `event.target?.type_text(text)`; never recapture a delayed target.
    /// Controls (Tab/Enter etc.), NUL, empty text and >1024 UTF-16 units fail.
    /// `Ok(())` means Windows accepted the events, not verified delivery. Some
    /// apps ignore VK_PACKET. `InputIncomplete` may leave partial text, including
    /// a lone UTF-16 surrogate; do not resend or erase automatically.
    ///
    /// `SendInput` cannot atomically bind delivery to an HWND. Browser DOM fields
    /// may share one native control. See the README for these Windows limits.
    ///
    /// Modelled after: enigo <https://github.com/enigo-rs/enigo/blob/v0.6.1/src/win/win_impl.rs>.
    pub fn type_text(&self, text: &str) -> Result<(), PlatformError> {
        #[cfg(windows)]
        {
            crate::validate_text(text, true)?;
            crate::windows::focus::type_text(&self.snapshot, text)
        }
        #[cfg(not(windows))]
        {
            let _ = text;
            Err(PlatformError::UnsupportedPlatform)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn release_wait_resets_for_held_keys_and_times_out() {
        assert_eq!(release_ready(Duration::ZERO, None), Ok(false));
        assert_eq!(
            release_ready(Duration::from_secs(1), Some(Duration::from_millis(19))),
            Ok(false)
        );
        assert_eq!(
            release_ready(Duration::from_secs(1), Some(Duration::from_millis(20))),
            Ok(true)
        );
        assert_eq!(release_ready(Duration::from_secs(1), None), Ok(false));
        assert_eq!(
            release_ready(Duration::from_secs(2), None),
            Err(PlatformError::ModifiersHeld)
        );
    }
    #[test]
    fn target_expiry_has_a_bounded_lifetime() {
        assert!(!expired(Duration::from_secs(5)));
        assert!(expired(Duration::from_millis(5001)));
    }
    #[test]
    fn partial_input_releases_only_an_unpaired_down() {
        for (sent, total, expected) in [
            (0, 8, None),
            (1, 8, Some(1)),
            (2, 8, None),
            (3, 8, Some(3)),
            (7, 8, Some(7)),
            (8, 8, None),
            (9, 8, None),
            (0, 0, None),
        ] {
            assert_eq!(release_index(sent, total), expected);
        }
        assert_eq!(release_index(99, 8), None);
        // A high surrogate can remain without a held key: no safe erase/retry.
        assert_eq!(release_index(2, 4), None);
    }
    #[test]
    fn integrity_never_allows_a_more_privileged_target() {
        assert_eq!(allow_integrity(0x2000, 0x1000), Ok(()));
        assert_eq!(allow_integrity(0x2000, 0x2000), Ok(()));
        assert_eq!(
            allow_integrity(0x2000, 0x3000),
            Err(PlatformError::TargetElevated)
        );
    }
    #[test]
    fn restoration_waits_for_activation_and_stops_at_the_deadline() {
        let pending = Err(PlatformError::TargetChanged);
        assert_eq!(restore_ready(Duration::ZERO, pending), Ok(false));
        assert_eq!(
            restore_ready(Duration::from_millis(490), pending),
            Ok(false)
        );
        assert_eq!(
            restore_ready(Duration::from_millis(500), pending),
            Err(PlatformError::TargetChanged)
        );
        assert_eq!(restore_ready(Duration::from_millis(200), Ok(())), Ok(true));
    }
}
