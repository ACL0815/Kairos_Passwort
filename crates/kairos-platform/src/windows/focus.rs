//! Native target validation and literal Unicode input, without changing layouts.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/windows/AutoTypeWindows.cpp>
//! and enigo <https://github.com/enigo-rs/enigo/blob/v0.6.1/src/win/win_impl.rs>.

use crate::{
    focus::{allow_integrity, expired, release_index, release_ready, restore_ready},
    PlatformError,
};
use std::{
    ptr, thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, HWND},
    Security::*,
    System::Threading::{
        GetCurrentProcess, GetCurrentProcessId, OpenProcess, OpenProcessToken,
        PROCESS_QUERY_LIMITED_INFORMATION,
    },
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
    pub(crate) trigger_key: Option<u16>,
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
        trigger_key: None,
    };
    snapshot.check_current(false)?;
    Ok(snapshot)
}

impl Snapshot {
    pub(crate) fn is_current_process(&self) -> bool {
        self.process == unsafe { GetCurrentProcessId() }
    }

    pub(crate) fn confirm_from_editor(&self) -> Result<Self, PlatformError> {
        self.restore()?;
        Ok(Self {
            window: self.window,
            control: self.control,
            process: self.process,
            thread: self.thread,
            captured: Instant::now(),
            trigger_key: self.trigger_key,
        })
    }

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
        if expiring && expired(self.captured.elapsed()) {
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
        let started = Instant::now();
        loop {
            let foreground = unsafe { GetForegroundWindow() };
            if !foreground.is_null()
                && foreground as usize != self.window
                && identity(foreground).0 != unsafe { GetCurrentProcessId() }
            {
                return Err(PlatformError::TargetChanged);
            }
            if restore_ready(started.elapsed(), self.check_current(false))? {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn keys_down(trigger_key: Option<u16>) -> bool {
    [VK_SHIFT, VK_CONTROL, VK_MENU, VK_LWIN, VK_RWIN]
        .iter()
        .copied()
        .chain(trigger_key)
        .any(|key| unsafe { GetAsyncKeyState(key as i32) } < 0)
}

struct OwnedHandle(HANDLE);
impl Drop for OwnedHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

fn integrity(process: HANDLE) -> Result<u32, PlatformError> {
    let mut token = ptr::null_mut();
    if unsafe { OpenProcessToken(process, TOKEN_QUERY, &mut token) } == 0 {
        return Err(PlatformError::TargetUnavailable);
    }
    let token = OwnedHandle(token);
    let mut bytes = 0;
    unsafe {
        GetTokenInformation(token.0, TokenIntegrityLevel, ptr::null_mut(), 0, &mut bytes);
    }
    if bytes < size_of::<TOKEN_MANDATORY_LABEL>() as u32 || bytes > 65_536 {
        return Err(PlatformError::TargetUnavailable);
    }
    // usize alignment covers TOKEN_MANDATORY_LABEL and its SID. No secret data.
    let mut buffer = vec![0usize; (bytes as usize).div_ceil(size_of::<usize>())];
    if unsafe {
        GetTokenInformation(
            token.0,
            TokenIntegrityLevel,
            buffer.as_mut_ptr().cast(),
            bytes,
            &mut bytes,
        )
    } == 0
    {
        return Err(PlatformError::TargetUnavailable);
    }
    unsafe {
        let label = &*buffer.as_ptr().cast::<TOKEN_MANDATORY_LABEL>();
        let sid = label.Label.Sid;
        if IsValidSid(sid) == 0 {
            return Err(PlatformError::TargetUnavailable);
        }
        let count = GetSidSubAuthorityCount(sid);
        if count.is_null() || *count == 0 {
            return Err(PlatformError::TargetUnavailable);
        }
        let rid = GetSidSubAuthority(sid, (*count - 1) as u32);
        if rid.is_null() {
            return Err(PlatformError::TargetUnavailable);
        }
        Ok(*rid)
    }
}

fn check_integrity(target: &Snapshot) -> Result<(), PlatformError> {
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, target.process) };
    if process.is_null() {
        return Err(PlatformError::TargetUnavailable);
    }
    let process = OwnedHandle(process);
    allow_integrity(
        integrity(unsafe { GetCurrentProcess() })?,
        integrity(process.0)?,
    )
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
    check_integrity(target)?;
    let inputs = InputBatch::new(text);
    let started = Instant::now();
    let mut released_since = None;
    loop {
        target.check_current(true)?;
        if keys_down(target.trigger_key) {
            released_since = None;
        } else {
            released_since.get_or_insert_with(Instant::now);
        }
        if release_ready(started.elapsed(), released_since.map(|time| time.elapsed()))? {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }
    target.check_current(true)?;
    if keys_down(target.trigger_key) {
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
        if let Some(index) = release_index(sent, inputs.0.len()) {
            // Release a partially inserted Unicode key-down, never resend text.
            unsafe {
                SendInput(1, ptr::addr_of!(inputs.0[index]), size_of::<INPUT>() as i32);
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
    fn queries_real_process_integrity_without_desktop_interaction() {
        let own = integrity(unsafe { GetCurrentProcess() }).unwrap();
        let process =
            unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, GetCurrentProcessId()) };
        assert!(!process.is_null());
        let process = OwnedHandle(process);
        assert_eq!(integrity(process.0).unwrap(), own);
    }
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
