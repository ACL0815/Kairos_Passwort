//! Real clipboard checks on a private, invisible window station. Run this test
//! alone: `cargo test -p kairos-platform --test isolated_clipboard -- --ignored`
//!
//! Modelled after: Microsoft Win32 <https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations>.

#![cfg(windows)]

use kairos_platform::{copy_secret, ClearOutcome};
use std::{ptr, thread, time::Duration};
use windows_sys::Win32::UI::WindowsAndMessaging::WINSTA_ALL_ACCESS;
use windows_sys::{
    core::w,
    Win32::System::{
        DataExchange::*, Memory::*, StationsAndDesktops::*, Threading::GetCurrentThreadId,
    },
};

struct PrivateStation {
    original: HWINSTA,
    original_desktop: HDESK,
    station: HWINSTA,
    desktop: HDESK,
}
impl PrivateStation {
    fn enter() -> Self {
        unsafe {
            let original = GetProcessWindowStation();
            let original_desktop = GetThreadDesktop(GetCurrentThreadId());
            assert!(!original.is_null() && !original_desktop.is_null());
            let station =
                CreateWindowStationW(ptr::null(), 0, WINSTA_ALL_ACCESS as u32, ptr::null());
            assert!(
                !station.is_null(),
                "private station creation failed: {}",
                std::io::Error::last_os_error()
            );
            let mut guard = Self {
                original,
                original_desktop,
                station,
                desktop: ptr::null_mut(),
            };
            assert_ne!(SetProcessWindowStation(station), 0);
            // GENERIC_ALL: a fresh unnamed station owned by this test process.
            guard.desktop = CreateDesktopW(
                w!("Default"),
                ptr::null(),
                ptr::null(),
                0,
                0x10000000,
                ptr::null(),
            );
            assert!(
                !guard.desktop.is_null(),
                "private desktop creation failed: {}",
                std::io::Error::last_os_error()
            );
            assert_ne!(SetThreadDesktop(guard.desktop), 0);
            assert_eq!(GetProcessWindowStation(), station);
            guard
        }
    }
}
impl Drop for PrivateStation {
    fn drop(&mut self) {
        unsafe {
            SetProcessWindowStation(self.original);
            SetThreadDesktop(self.original_desktop);
            if !self.desktop.is_null() {
                CloseDesktop(self.desktop);
            }
            CloseWindowStation(self.station);
        }
    }
}

#[test]
#[ignore = "creates a private window station; run alone in its own test process"]
fn protected_clipboard_on_private_window_station() {
    let _station = PrivateStation::enter();
    let first = copy_secret("DUMMY-isolated-Äpfel", None).unwrap();
    unsafe {
        assert_ne!(OpenClipboard(ptr::null_mut()), 0);
        for name in [
            w!("ExcludeClipboardContentFromMonitorProcessing"),
            w!("CanIncludeInClipboardHistory"),
            w!("CanUploadToCloudClipboard"),
        ] {
            let format = RegisterClipboardFormatW(name);
            let handle = GetClipboardData(format);
            assert!(!handle.is_null());
            assert!(GlobalSize(handle) >= 4);
            let data = GlobalLock(handle).cast::<u32>();
            assert!(!data.is_null());
            assert_eq!(data.read(), 0);
            GlobalUnlock(handle);
        }
        // Force Windows' synthesized ANSI/locale formats before checking ownership.
        assert!(!GetClipboardData(1).is_null()); // CF_TEXT
        let _ = GetClipboardData(16); // CF_LOCALE
        CloseClipboard();
    }
    assert_eq!(first.clear_if_current(), Ok(ClearOutcome::Cleared));
    assert_eq!(first.clear_if_current(), Ok(ClearOutcome::NotPresent));
    let old = copy_secret("DUMMY-identical", Some(Duration::from_secs(30))).unwrap();
    let mut latest = old;
    for _ in 0..20 {
        latest = copy_secret("DUMMY-identical", Some(Duration::from_secs(30))).unwrap();
    }
    assert_eq!(old.clear_if_current(), Ok(ClearOutcome::Replaced));
    assert_eq!(latest.clear_if_current(), Ok(ClearOutcome::Cleared));
    let expired = copy_secret("DUMMY-timeout", Some(Duration::from_millis(50))).unwrap();
    thread::sleep(Duration::from_millis(500));
    assert_eq!(expired.clear_if_current(), Ok(ClearOutcome::NotPresent));
    assert_eq!(unsafe { IsClipboardFormatAvailable(13) }, 0);
}
