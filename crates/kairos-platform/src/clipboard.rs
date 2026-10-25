//! Ownership-checked clipboard clearing without a retained text or hash copy.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp>.

use crate::PlatformError;
use std::time::Duration;

/// Recommended timeout for the application's normal Copy action.
pub const DEFAULT_CLEAR_AFTER: Duration = Duration::from_secs(30);
/// Maximum explicitly configured timeout; `None` is an explicit opt-out.
pub const MAX_CLEAR_AFTER: Duration = Duration::from_secs(600);

#[cfg(any(windows, test))]
pub(crate) fn validate_timeout(timeout: Option<Duration>) -> Result<(), PlatformError> {
    if timeout.is_some_and(|value| value > MAX_CLEAR_AFTER) {
        Err(PlatformError::InvalidTimeout)
    } else {
        Ok(())
    }
}

/// Identifies our write without retaining the secret or a password hash.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClipboardReceipt {
    pub(crate) sequence: u32,
    #[cfg(any(windows, test))]
    pub(crate) marker: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClearOutcome {
    Cleared,
    /// A different marker is present, including a new copy of identical text.
    Replaced,
    /// No clipboard formats remain (for example, the timer already cleared it).
    NotPresent,
}

impl ClipboardReceipt {
    /// Checks the unique write marker under the Windows clipboard lock. This
    /// does not depend on when Windows finalizes its clipboard sequence number.
    pub fn clear_if_current(self) -> Result<ClearOutcome, PlatformError> {
        #[cfg(windows)]
        return crate::windows::clipboard::clear(self);
        #[cfg(not(windows))]
        Err(PlatformError::UnsupportedPlatform)
    }

    /// Observed during writing, for diagnostics only. Windows may subsequently
    /// synthesize formats; this number is not used to authorize clearing.
    pub fn sequence_number(self) -> u32 {
        self.sequence
    }

    #[cfg(any(windows, test))]
    pub(crate) fn matches(self, marker: Option<u64>) -> bool {
        marker == Some(self.marker)
    }
}

/// Copies only on explicit user action, setting all three Windows history/cloud
/// exclusions before publishing UTF-16 text under the same clipboard lock.
///
/// `Some(Duration::ZERO)` schedules an immediate clear; `None` explicitly disables
/// the timeout. Prefer `Some(DEFAULT_CLEAR_AFTER)`; at most ten minutes is allowed.
/// One worker holds only metadata and retries transient contention at most 50
/// times. Clear the last receipt before app shutdown. Copies held by Windows or
/// other applications may remain.
pub fn copy_secret(
    text: &str,
    clear_after: Option<Duration>,
) -> Result<ClipboardReceipt, PlatformError> {
    #[cfg(windows)]
    {
        crate::validate_text(text, false)?;
        validate_timeout(clear_after)?;
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
        assert!(receipt.matches(Some(7)));
        assert!(!receipt.matches(Some(8)));
        assert!(!receipt.matches(None));
    }
    #[test]
    fn sequence_finalization_and_wrap_do_not_change_write_identity() {
        for sequence in [0, 1, u32::MAX] {
            let receipt = ClipboardReceipt {
                sequence,
                marker: 7,
            };
            assert!(receipt.matches(Some(7)));
            assert!(!receipt.matches(Some(8)));
        }
    }
    #[test]
    fn bounds_timeouts_and_keeps_the_explicit_opt_out() {
        for value in [
            None,
            Some(Duration::ZERO),
            Some(DEFAULT_CLEAR_AFTER),
            Some(MAX_CLEAR_AFTER),
        ] {
            assert_eq!(validate_timeout(value), Ok(()));
        }
        assert_eq!(
            validate_timeout(Some(MAX_CLEAR_AFTER + Duration::from_nanos(1))),
            Err(PlatformError::InvalidTimeout)
        );
        assert_eq!(
            validate_timeout(Some(Duration::MAX)),
            Err(PlatformError::InvalidTimeout)
        );
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
