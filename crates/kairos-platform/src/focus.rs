//! Capture the destination before a popup can take focus.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/windows/AutoTypeWindows.cpp>.

use crate::PlatformError;

/// Opaque window/process/thread/control identity, containing no text.
/// Autotype expires after five seconds; restoration may be used later.
pub struct FocusTarget {
    #[cfg(windows)]
    pub(crate) snapshot: crate::windows::focus::Snapshot,
}

impl FocusTarget {
    /// Captures the foreground window and its focused native control.
    pub fn capture() -> Result<Self, PlatformError> {
        #[cfg(windows)]
        return crate::windows::focus::capture().map(|snapshot| Self { snapshot });
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    /// Restores after an editor closes or copies. Refuses to steal focus from
    /// an unrelated application; reports a denied `SetForegroundWindow` call.
    pub fn restore(&self) -> Result<(), PlatformError> {
        #[cfg(windows)]
        return self.snapshot.restore();
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    /// Types literal Unicode into this same field, waiting at most two seconds
    /// for Ctrl/Alt/Shift/Win release. No clipboard fallback or automatic retry.
    /// Call as `event.target.type_text(text)`; never recapture a delayed target.
    /// Controls (Tab/Enter etc.), NUL, empty text and >1024 UTF-16 units fail.
    /// `InputIncomplete` may mean partial entry; do not resend.
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
