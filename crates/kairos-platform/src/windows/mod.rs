//! Minimal unsafe Win32 boundary. OS handles never contain secret text.
//!
//! Modelled after: arboard <https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/src/platform/windows.rs>.

pub(crate) mod clipboard;
pub(crate) mod focus;
pub(crate) mod hotkeys;
