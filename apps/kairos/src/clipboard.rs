//! Platform-backed secret copying with a metadata-only shutdown receipt.
//!
//! Modelled after: KeePassXC https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp

use kairos_platform::{copy_secret, ClipboardReceipt, DEFAULT_CLEAR_AFTER};
use std::cell::Cell;
use zeroize::Zeroizing;

pub(crate) trait ClipboardAccess {
    fn copy(&self, text: &str) -> Result<(), ()>;
    fn paste(&self) -> Result<Zeroizing<String>, ()>;
}

#[derive(Default)]
pub(crate) struct ProtectedClipboard {
    receipt: Cell<Option<ClipboardReceipt>>,
}

impl ClipboardAccess for ProtectedClipboard {
    fn copy(&self, text: &str) -> Result<(), ()> {
        if text.is_empty() || text.chars().any(char::is_control) {
            return Err(());
        }
        let receipt = copy_secret(text, Some(DEFAULT_CLEAR_AFTER)).map_err(|_| ())?;
        self.receipt.set(Some(receipt));
        Ok(())
    }

    fn paste(&self) -> Result<Zeroizing<String>, ()> {
        #[cfg(windows)]
        {
            let _lock = clipboard_win::Clipboard::new_attempts(10).map_err(|_| ())?;
            let mut bytes = Zeroizing::new(Vec::new());
            clipboard_win::raw::get_string(&mut bytes).map_err(|_| ())?;
            let text = std::str::from_utf8(&bytes).map_err(|_| ())?;
            Ok(Zeroizing::new(text.to_owned()))
        }
        #[cfg(not(windows))]
        Err(())
    }
}

impl Drop for ProtectedClipboard {
    fn drop(&mut self) {
        if let Some(receipt) = self.receipt.take() {
            let _ = receipt.clear_if_current();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_text_is_rejected_before_calling_the_platform_clipboard() {
        let clipboard = ProtectedClipboard::default();
        for text in [
            "",
            "PUBLIC\0FIXTURE",
            "PUBLIC\nFIXTURE",
            "PUBLIC\rFIXTURE",
            "PUBLIC\tFIXTURE",
            "PUBLIC\u{1b}FIXTURE",
        ] {
            assert!(clipboard.copy(text).is_err());
        }
        assert!(clipboard.receipt.get().is_none());
    }
}
