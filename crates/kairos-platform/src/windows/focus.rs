//! Native target validation and literal Unicode input, without changing layouts.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/windows/AutoTypeWindows.cpp>
//! and enigo <https://github.com/enigo-rs/enigo/blob/v0.6.1/src/win/win_impl.rs>.

use crate::PlatformError;
use std::{
    ptr, thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::HWND,
    System::Threading::GetCurrentProcessId,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};
use zeroize::Zeroize;

/// Integers are opaque identity values, never dereferenced; this allows a target
/// captured on the message thread to be safely checked again on the app thread.
pub(crate) struct Snapshot {
    window: usize,
    control: usize,
    process: u32,
    thread: u32,
    captured: Instant,
}

fn identity(window: HWND) -> (u32, u32) {
    let mut process = 0;
    let thread = unsafe { GetWindowThreadProcessId(window, &mut process) };
    (process, thread)
}

fn gui_info(thread: u32) -> Result<GUITHREADINFO, PlatformError> {
    let mut info = GUITHREADINFO {
        cbSize: size_of::<GUITHREADINFO>() as u32,
        ..Default::default()
    };
    if unsafe { GetGUIThreadInfo(thread, &mut info) } == 0
        || info.hwndFocus.is_null()
        || info.flags & (GUI_INMENUMODE | GUI_INMOVESIZE | GUI_POPUPMENUMODE | GUI_SYSTEMMENUMODE)
            != 0
    {
        return Err(PlatformError::TargetUnavailable);
    }
    Ok(info)
}

pub(crate) fn capture() -> Result<Snapshot, PlatformError> {
    let window = unsafe { GetForegroundWindow() };
    if window.is_null() {
        return Err(PlatformError::TargetUnavailable);
    }
    let (process, thread) = identity(window);
    if process == 0 || thread == 0 {
        return Err(PlatformError::TargetUnavailable);
    }
    let info = gui_info(thread)?;
    let snapshot = Snapshot {
        window: window as usize,
        control: info.hwndFocus as usize,
        process,
        thread,
        captured: Instant::now(),
    };
    snapshot.check_current(false)?;
    Ok(snapshot)
}

impl Snapshot {
    fn check_alive(&self) -> Result<(), PlatformError> {
        let window = self.window as HWND;
        let control = self.control as HWND;
        if unsafe { IsWindow(window) } == 0
            || unsafe { IsWindow(control) } == 0
            || identity(window) != (self.process, self.thread)
            || identity(control) != (self.process, self.thread)
        {
            return Err(PlatformError::TargetUnavailable);
        }
        Ok(())
    }

    fn check_current(&self, expiring: bool) -> Result<(), PlatformError> {
        if expiring && self.captured.elapsed() > Duration::from_secs(5) {
            return Err(PlatformError::TargetExpired);
        }
        self.check_alive()?;
        let info = gui_info(self.thread)?;
        if unsafe { GetForegroundWindow() } as usize != self.window
            || info.hwndFocus as usize != self.control
            || info.hwndActive as usize != self.window
        {
            return Err(PlatformError::TargetChanged);
        }
        Ok(())
    }

    pub(crate) fn restore(&self) -> Result<(), PlatformError> {
        self.check_alive()?;
        let foreground = unsafe { GetForegroundWindow() };
        if foreground as usize != self.window {
            if foreground.is_null() || identity(foreground).0 != unsafe { GetCurrentProcessId() } {
                return Err(PlatformError::TargetChanged);
            }
            if unsafe { SetForegroundWindow(self.window as HWND) } == 0 {
                return Err(PlatformError::ForegroundDenied);
            }
        }
        // A successful API return alone is insufficient; never use an Alt-key
        // trick, AttachThreadInput, or a foreground-lock timeout override.
        self.check_current(false)
    }
}

fn modifiers_down() -> bool {
    [VK_SHIFT, VK_CONTROL, VK_MENU, VK_LWIN, VK_RWIN]
        .iter()
        .any(|key| unsafe { GetAsyncKeyState(*key as i32) } < 0)
}

/// Heap copies of scan units are scrubbed even when validation or SendInput fails.
struct InputBatch(Vec<INPUT>);

impl InputBatch {
    fn new(text: &str) -> Self {
        let mut inputs = Vec::with_capacity(text.encode_utf16().count() * 2);
        for unit in text.encode_utf16() {
            for flags in [KEYEVENTF_UNICODE, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP] {
                inputs.push(INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: 0,
                            wScan: unit,
                            dwFlags: flags,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                });
            }
        }
        Self(inputs)
    }
}

impl Drop for InputBatch {
    fn drop(&mut self) {
        for input in &mut self.0 {
            // SAFETY: all items were initialized as keyboard INPUTs above.
            unsafe {
                input.Anonymous.ki.wScan.zeroize();
            }
        }
    }
}

pub(crate) fn type_text(target: &Snapshot, text: &str) -> Result<(), PlatformError> {
    target.check_current(true)?;
    let inputs = InputBatch::new(text);
    let started = Instant::now();
    let mut released_since = None;
    loop {
        target.check_current(true)?;
        if modifiers_down() {
            released_since = None;
        } else if released_since.get_or_insert_with(Instant::now).elapsed()
            >= Duration::from_millis(20)
        {
            break;
        }
        if started.elapsed() >= Duration::from_secs(2) {
            return Err(PlatformError::ModifiersHeld);
        }
        thread::sleep(Duration::from_millis(5));
    }
    target.check_current(true)?;
    if modifiers_down() {
        return Err(PlatformError::ModifiersHeld);
    }
    // A single batch prevents interleaving physical keyboard/mouse input. Windows
    // still cannot bind it atomically to this target; programmatic focus races
    // and multiple DOM controls within the same HWND remain platform limitations.
    let sent = unsafe {
        SendInput(
            inputs.0.len() as u32,
            inputs.0.as_ptr(),
            size_of::<INPUT>() as i32,
        )
    } as usize;
    if sent != inputs.0.len() {
        if sent % 2 == 1 && sent < inputs.0.len() {
            // Release a partially inserted Unicode key-down, never resend text.
            unsafe {
                SendInput(1, ptr::addr_of!(inputs.0[sent]), size_of::<INPUT>() as i32);
            }
        }
        return Err(PlatformError::InputIncomplete);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_input_has_paired_down_up_and_zero_virtual_keys() {
        let batch = InputBatch::new("Ä@🦀");
        assert_eq!(batch.0.len(), 8);
        for (index, unit) in "Ä@🦀".encode_utf16().enumerate() {
            let pair = &batch.0[index * 2..index * 2 + 2];
            unsafe {
                assert_eq!(pair[0].r#type, INPUT_KEYBOARD);
                assert_eq!(pair[0].Anonymous.ki.wVk, 0);
                assert_eq!(pair[0].Anonymous.ki.wScan, unit);
                assert_eq!(pair[1].Anonymous.ki.wScan, unit);
                assert_eq!(pair[0].Anonymous.ki.dwFlags, KEYEVENTF_UNICODE);
                assert_eq!(
                    pair[1].Anonymous.ki.dwFlags,
                    KEYEVENTF_UNICODE | KEYEVENTF_KEYUP
                );
            }
        }
    }
}
