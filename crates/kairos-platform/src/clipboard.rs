//! Clipboard protection and ownership receipts, without retaining a text copy.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp>.

use crate::PlatformError;
use std::time::Duration;

/// Identifies our clipboard write, without storing the secret or its hash.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClipboardReceipt {
    pub(crate) sequence: u32,
    #[cfg(any(windows, test))]
    pub(crate) marker: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClearOutcome {
    Cleared,
    /// Another write occurred, even if it contains identical text.
    Replaced,
}

impl ClipboardReceipt {
    /// Checks sequence and marker under the Windows clipboard lock, leaving
    /// replaced items untouched.
    pub fn clear_if_current(self) -> Result<ClearOutcome, PlatformError> {
        #[cfg(windows)]
        return crate::windows::clipboard::clear(self);
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    pub fn sequence_number(self) -> u32 {
        self.sequence
    }

    #[cfg(any(windows, test))]
    pub(crate) fn matches(self, sequence: u32, marker: Option<u64>) -> bool {
        self.sequence != 0 && self.sequence == sequence && marker == Some(self.marker)
    }
}

/// Copies on explicit user action, setting all three Windows history/cloud
/// exclusions before publishing UTF-16 text under the same clipboard lock.
///
/// `Some(Duration::ZERO)` schedules an immediate clear; `None` disables the
/// timeout. Durations above 24 hours are rejected. The single timeout worker
/// retains only a receipt and retries contention until cleared or replaced.
/// Callers can also clear explicitly by receipt, including before app shutdown.
/// Windows and other applications may retain their own copies.
pub fn copy_secret(
    text: &str,
    clear_after: Option<Duration>,
) -> Result<ClipboardReceipt, PlatformError> {
    #[cfg(windows)]
    {
        crate::validate_text(text, false)?;
        crate::windows::clipboard::copy(text, clear_after)
    }
    #[cfg(not(windows))]
    {
        let _ = (text, clear_after);
        Err(PlatformError::UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_only_the_exact_owned_write() {
        let receipt = ClipboardReceipt {
            sequence: 42,
            marker: 7,
        };
        assert!(receipt.matches(42, Some(7)));
        assert!(!receipt.matches(43, Some(7)));
        assert!(!receipt.matches(42, Some(8)));
        assert!(!receipt.matches(42, None));
    }

    #[test]
    fn unknown_or_wrapped_sequence_does_not_authorize_clear() {
        assert!(!ClipboardReceipt {
            sequence: 0,
            marker: 7
        }
        .matches(0, Some(7)));
        assert!(!ClipboardReceipt {
            sequence: u32::MAX,
            marker: 7
        }
        .matches(1, Some(7)));
    }

    #[cfg(not(windows))]
    #[test]
    fn clipboard_stub_is_explicit() {
        assert_eq!(
            copy_secret("dummy", None),
            Err(PlatformError::UnsupportedPlatform)
        );
    }
}
