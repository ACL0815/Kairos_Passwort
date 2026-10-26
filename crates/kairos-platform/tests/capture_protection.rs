//! Own-window affinity checks. No clipboard, hotkeys, focus changes, or real secrets.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp>.

#![cfg(windows)]

use kairos_platform::{set_window_capture_policy, CapturePolicy, CaptureProtection, CaptureStatus};
use raw_window_handle::{
    HandleError, HasWindowHandle, RawWindowHandle, Win32WindowHandle, WindowHandle,
};
use std::{cell::Cell, num::NonZeroIsize, ptr};
use windows_sys::{
    core::w,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::*,
    },
};

thread_local! {
    static PROTECTED_BEFORE_SHOW: Cell<Option<bool>> = const { Cell::new(None) };
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_SHOWWINDOW && wparam != 0 {
        let mut actual = WDA_NONE;
        let protected = unsafe { GetWindowDisplayAffinity(window, &mut actual) } != 0
            && matches!(actual, WDA_EXCLUDEFROMCAPTURE | WDA_MONITOR);
        PROTECTED_BEFORE_SHOW.with(|observed| observed.set(Some(protected)));
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

struct TestWindow(HWND);

impl TestWindow {
    fn new(owner: HWND, child: bool) -> Self {
        let class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            lpszClassName: w!("Kairos capture protection test"),
            ..Default::default()
        };
        unsafe {
            RegisterClassW(&class);
            let window = CreateWindowExW(
                0,
                class.lpszClassName,
                w!("PUBLIC CAPTURE FIXTURE"),
                if child { WS_CHILD } else { WS_OVERLAPPEDWINDOW },
                -32000,
                -32000,
                100,
                100,
                owner,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            );
            assert!(!window.is_null(), "own test window must be created");
            Self(window)
        }
    }

    fn affinity(&self) -> u32 {
        let mut actual = WDA_NONE;
        assert_ne!(unsafe { GetWindowDisplayAffinity(self.0, &mut actual) }, 0);
        actual
    }

    fn show_without_focus(&self) {
        PROTECTED_BEFORE_SHOW.with(|observed| observed.set(None));
        unsafe { ShowWindow(self.0, SW_SHOWNOACTIVATE) };
    }
}

impl HasWindowHandle for TestWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = Win32WindowHandle::new(NonZeroIsize::new(self.0 as isize).unwrap());
        // TestWindow owns the HWND for the entire returned borrow.
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) })
    }
}

impl Drop for TestWindow {
    fn drop(&mut self) {
        unsafe { DestroyWindow(self.0) };
    }
}

#[test]
fn borrowed_own_window_round_trips_protection_and_opt_out() {
    let window = TestWindow::new(ptr::null_mut(), false);
    let status = set_window_capture_policy(&window, CapturePolicy::Protect);
    assert!(
        matches!(status, CaptureStatus::Excluded | CaptureStatus::MonitorOnly),
        "{status:?}"
    );
    assert_eq!(
        window.affinity(),
        if status == CaptureStatus::Excluded {
            WDA_EXCLUDEFROMCAPTURE
        } else {
            WDA_MONITOR
        }
    );
    assert_eq!(
        set_window_capture_policy(&window, CapturePolicy::Allow),
        CaptureStatus::Allowed
    );
    assert_eq!(window.affinity(), WDA_NONE);
}

#[test]
fn later_viewports_and_owned_dialogs_are_protected_before_show_without_activation() {
    let guard = CaptureProtection::new(CapturePolicy::Protect);
    assert_eq!(guard.status(), CaptureStatus::Pending);
    let root = TestWindow::new(ptr::null_mut(), false);
    let status = guard.apply_to(&root);
    assert!(
        matches!(status, CaptureStatus::Excluded | CaptureStatus::MonitorOnly),
        "{status:?}"
    );
    for owner in [ptr::null_mut(), root.0] {
        let later = TestWindow::new(owner, false);
        later.show_without_focus();
        PROTECTED_BEFORE_SHOW.with(|observed| assert_eq!(observed.get(), Some(true)));
        assert_eq!(later.affinity(), root.affinity());
        unsafe { ShowWindow(later.0, SW_HIDE) };
        later.show_without_focus();
        PROTECTED_BEFORE_SHOW.with(|observed| assert_eq!(observed.get(), Some(true)));
    }
    let child = TestWindow::new(root.0, true);
    child.show_without_focus();
    assert_eq!(
        guard.status(),
        status,
        "child controls must not cause top-level affinity errors"
    );
    drop(guard);
    let unprotected = TestWindow::new(ptr::null_mut(), false);
    unprotected.show_without_focus();
    assert_eq!(
        unprotected.affinity(),
        WDA_NONE,
        "dropping the guard removes its hook"
    );
    let replacement = CaptureProtection::new(CapturePolicy::Protect);
    assert_eq!(replacement.status(), CaptureStatus::Pending);
}

#[test]
fn opt_out_leaves_later_windows_capturable() {
    let guard = CaptureProtection::new(CapturePolicy::Allow);
    let later = TestWindow::new(ptr::null_mut(), false);
    later.show_without_focus();
    assert_eq!(later.affinity(), WDA_NONE);
    assert_eq!(guard.status(), CaptureStatus::Allowed);
}

#[test]
fn popup_created_visible_is_already_protected_in_its_first_show_message() {
    let guard = CaptureProtection::new(CapturePolicy::Protect);
    let root = TestWindow::new(ptr::null_mut(), false);
    PROTECTED_BEFORE_SHOW.with(|observed| observed.set(None));
    let popup = TestWindow(unsafe {
        CreateWindowExW(
            WS_EX_NOACTIVATE,
            w!("Kairos capture protection test"),
            w!("PUBLIC VISIBLE POPUP FIXTURE"),
            WS_POPUP | WS_VISIBLE,
            -32000,
            -32000,
            100,
            100,
            root.0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
        )
    });
    assert!(!popup.0.is_null());
    PROTECTED_BEFORE_SHOW.with(|observed| assert_eq!(observed.get(), Some(true)));
    assert!(matches!(
        guard.status(),
        CaptureStatus::Excluded | CaptureStatus::MonitorOnly
    ));
}

#[test]
fn rejected_child_handle_does_not_turn_into_a_success_after_root_protection() {
    let guard = CaptureProtection::new(CapturePolicy::Protect);
    let root = TestWindow::new(ptr::null_mut(), false);
    let child = TestWindow::new(root.0, true);
    let failure = guard.apply_to(&child);
    assert!(
        matches!(failure, CaptureStatus::Unavailable(_)),
        "{failure:?}"
    );
    assert!(matches!(
        guard.apply_to(&root),
        CaptureStatus::Excluded | CaptureStatus::MonitorOnly
    ));
    assert_eq!(guard.status(), failure);
}

#[test]
fn second_guard_reports_failure_without_removing_the_first_hook() {
    let first = CaptureProtection::new(CapturePolicy::Protect);
    let second = CaptureProtection::new(CapturePolicy::Protect);
    assert_eq!(
        second.status(),
        CaptureStatus::Unavailable(kairos_platform::CaptureError::HookAlreadyInstalled)
    );
    drop(second);
    let later = TestWindow::new(ptr::null_mut(), false);
    later.show_without_focus();
    assert!(matches!(
        first.status(),
        CaptureStatus::Excluded | CaptureStatus::MonitorOnly
    ));
    PROTECTED_BEFORE_SHOW.with(|observed| assert_eq!(observed.get(), Some(true)));
}
