//! Interactive, dummy-only Win32 checks. Run individually on an idle test desktop:
//! `cargo test -p kairos-platform --test windows_interaction -- --ignored --test-threads=1`
//!
//! Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/winit.rs>.

#![cfg(windows)]

use kairos_platform::{
    copy_secret, Action, ClearOutcome, FocusTarget, GlobalHotkeys, HotkeyBinding, PlatformError,
};
use std::{
    ptr, thread,
    time::{Duration, Instant},
};
use windows_sys::{
    core::w,
    Win32::{
        Foundation::HWND,
        System::{DataExchange::*, Memory::*},
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
};

struct TestWindow {
    window: HWND,
    first: HWND,
    second: HWND,
}

impl TestWindow {
    fn new() -> Self {
        unsafe {
            let window = CreateWindowExW(
                0,
                w!("STATIC"),
                w!("Kairos: dummy interaction test"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100,
                100,
                600,
                180,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            );
            assert!(!window.is_null());
            let first = CreateWindowExW(
                0,
                w!("EDIT"),
                w!(""),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                10,
                10,
                550,
                30,
                window,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            );
            let second = CreateWindowExW(
                0,
                w!("EDIT"),
                w!(""),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                10,
                50,
                550,
                30,
                window,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            );
            assert!(!first.is_null() && !second.is_null());
            assert_ne!(
                SetForegroundWindow(window),
                0,
                "run on an idle interactive Windows test desktop"
            );
            SetFocus(first);
            pump();
            assert_eq!(GetForegroundWindow(), window);
            Self {
                window,
                first,
                second,
            }
        }
    }
}

impl Drop for TestWindow {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.window);
        }
    }
}

fn pump() {
    unsafe {
        let mut message = MSG::default();
        while PeekMessageW(&mut message, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn control_text(control: HWND) -> String {
    let mut buffer = [0u16; 100];
    let len = unsafe { GetWindowTextW(control, buffer.as_mut_ptr(), buffer.len() as i32) };
    String::from_utf16(&buffer[..len as usize]).unwrap()
}

#[test]
#[ignore = "changes the real clipboard; dummy data only, idle test desktop required"]
fn clipboard_flags_ownership_and_timeout() {
    let first = copy_secret("DUMMY-Äpfel", None).unwrap();
    assert_ne!(first.sequence_number(), 0);
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
        CloseClipboard();
    }
    // Identical text is still a new write: an old timeout must not clear it.
    let second = copy_secret("DUMMY-Äpfel", None).unwrap();
    assert_eq!(first.clear_if_current(), Ok(ClearOutcome::Replaced));
    assert_eq!(second.clear_if_current(), Ok(ClearOutcome::Cleared));
    let old = copy_secret("DUMMY-old", Some(Duration::from_millis(100))).unwrap();
    let replacement = copy_secret("DUMMY-replacement", None).unwrap();
    thread::sleep(Duration::from_millis(300));
    assert_eq!(old.clear_if_current(), Ok(ClearOutcome::Replaced));
    assert_eq!(replacement.clear_if_current(), Ok(ClearOutcome::Cleared));
    let expiring = copy_secret("DUMMY-timeout", Some(Duration::from_millis(50))).unwrap();
    thread::sleep(Duration::from_millis(300));
    assert_eq!(expiring.clear_if_current(), Ok(ClearOutcome::NotPresent));
    assert_eq!(unsafe { IsClipboardFormatAvailable(13) }, 0);
}

#[test]
#[ignore = "creates and focuses native test fields and injects fixed dummy Unicode"]
fn autotype_unicode_and_changed_control() {
    let window = TestWindow::new();
    let target = FocusTarget::capture().unwrap();
    unsafe {
        SetFocus(window.second);
    }
    assert_eq!(
        target.type_text("DUMMY-must-not-appear"),
        Err(PlatformError::TargetChanged)
    );
    assert_eq!(control_text(window.first), "");
    assert_eq!(control_text(window.second), "");
    unsafe {
        SetFocus(window.first);
    }
    let target = FocusTarget::capture().unwrap();
    target.type_text("DUMMY-Ä@🦀").unwrap();
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(2) && control_text(window.first) != "DUMMY-Ä@🦀"
    {
        pump();
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(control_text(window.first), "DUMMY-Ä@🦀");
    assert_eq!(control_text(window.second), "");
    assert_eq!(
        target.type_text("DUMMY\twrong-field"),
        Err(PlatformError::InvalidText)
    );
    target.restore().unwrap();
}

#[test]
#[ignore = "activates two native dummy windows to exercise real foreground restoration"]
fn restore_from_another_own_window() {
    let original = TestWindow::new();
    unsafe {
        SetFocus(original.window);
    }
    let target = FocusTarget::capture().unwrap();
    let popup = TestWindow::new();
    assert_eq!(unsafe { GetForegroundWindow() }, popup.window);
    // Restore before destroying the popup, while our app still owns foreground.
    target.restore().unwrap();
    assert_eq!(unsafe { GetForegroundWindow() }, original.window);
}

#[test]
#[ignore = "registers real global shortcuts and checks partial registration rollback"]
fn hotkey_registration_conflict_rollback_and_drop() {
    unsafe {
        assert_ne!(
            RegisterHotKey(
                ptr::null_mut(),
                1001,
                MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT,
                VK_F10 as u32
            ),
            0
        );
    }
    assert!(matches!(
        GlobalHotkeys::new(&HotkeyBinding::defaults()),
        Err(PlatformError::HotkeyUnavailable)
    ));
    unsafe {
        assert_ne!(
            RegisterHotKey(
                ptr::null_mut(),
                1002,
                MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT,
                VK_F9 as u32
            ),
            0,
            "first registration was not rolled back"
        );
        UnregisterHotKey(ptr::null_mut(), 1002);
        UnregisterHotKey(ptr::null_mut(), 1001);
    }
    let service = GlobalHotkeys::new(&HotkeyBinding::defaults()).unwrap();
    assert!(matches!(
        GlobalHotkeys::new(&HotkeyBinding::defaults()),
        Err(PlatformError::HotkeysAlreadyRunning)
    ));
    drop(service);
    drop(GlobalHotkeys::new(&HotkeyBinding::defaults()).unwrap());
}

#[test]
#[ignore = "sends a real hotkey into a fixed dummy test window, then types dummy text"]
fn hotkey_channel_captures_the_original_field() {
    let window = TestWindow::new();
    let service = GlobalHotkeys::new(&HotkeyBinding::defaults()).unwrap();
    let keys = [
        (VK_CONTROL, 0),
        (VK_SHIFT, 0),
        (VK_F9, 0),
        (VK_F9, KEYEVENTF_KEYUP),
        (VK_SHIFT, KEYEVENTF_KEYUP),
        (VK_CONTROL, KEYEVENTF_KEYUP),
    ];
    let inputs: Vec<_> = keys
        .into_iter()
        .map(|(key, flags)| INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: key,
                    dwFlags: flags,
                    ..Default::default()
                },
            },
        })
        .collect();
    assert_eq!(
        unsafe {
            SendInput(
                inputs.len() as u32,
                inputs.as_ptr(),
                size_of::<INPUT>() as i32,
            )
        },
        inputs.len() as u32
    );
    let started = Instant::now();
    let event = loop {
        pump();
        if let Some(event) = service.try_recv().unwrap() {
            break event;
        }
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "no hotkey event delivered"
        );
        thread::sleep(Duration::from_millis(5));
    };
    assert_eq!(event.action, Action::OpenEditor);
    event.target.unwrap().type_text("DUMMY-channel").unwrap();
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(2)
        && control_text(window.first) != "DUMMY-channel"
    {
        pump();
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(control_text(window.first), "DUMMY-channel");
}
