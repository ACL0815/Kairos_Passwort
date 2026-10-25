//! Small Win32 boundary, independently implemented from the documented contracts.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp>.

use crate::{capture::CaptureState, CaptureError, CapturePolicy, CaptureStatus};
use std::{cell::Cell, ptr, rc::Rc};
use windows_sys::{
    Wdk::System::SystemServices::RtlGetVersion,
    Win32::{
        Foundation::{GetLastError, HWND, LPARAM, LRESULT, WPARAM},
        System::{SystemInformation::OSVERSIONINFOW, Threading::GetCurrentThreadId},
        UI::WindowsAndMessaging::*,
    },
};

thread_local! {
    static STATE: Cell<Option<Rc<CaptureState>>> = const { Cell::new(None) };
    static NESTED_SHOW: Cell<bool> = const { Cell::new(false) };
}

fn affinity_for_version(major: u32, build: u32) -> u32 {
    if major > 10 || (major == 10 && build >= 19041) {
        WDA_EXCLUDEFROMCAPTURE
    } else {
        WDA_MONITOR
    }
}

fn protected_affinity() -> Result<u32, CaptureError> {
    let mut version = OSVERSIONINFOW {
        dwOSVersionInfoSize: size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };
    // RtlGetVersion avoids manifest-dependent version virtualization.
    let result = unsafe { RtlGetVersion(&mut version) };
    if result < 0 {
        Err(CaptureError::VersionUnavailable(result))
    } else {
        Ok(affinity_for_version(
            version.dwMajorVersion,
            version.dwBuildNumber,
        ))
    }
}

pub(crate) fn apply(window: HWND, policy: CapturePolicy) -> CaptureStatus {
    let affinity = match policy {
        CapturePolicy::Allow => WDA_NONE,
        CapturePolicy::Protect => match protected_affinity() {
            Ok(affinity) => affinity,
            Err(error) => return CaptureStatus::Unavailable(error),
        },
    };
    // The borrowed handle or synchronous hook message keeps this own-process HWND live.
    unsafe {
        if SetWindowDisplayAffinity(window, affinity) == 0 {
            return CaptureStatus::Unavailable(CaptureError::SetAffinityFailed(GetLastError()));
        }
        let mut actual = WDA_NONE;
        if GetWindowDisplayAffinity(window, &mut actual) == 0 {
            return CaptureStatus::Unavailable(CaptureError::ReadAffinityFailed(GetLastError()));
        }
        match (policy, actual) {
            (CapturePolicy::Allow, WDA_NONE) => CaptureStatus::Allowed,
            (CapturePolicy::Protect, WDA_EXCLUDEFROMCAPTURE) => CaptureStatus::Excluded,
            (CapturePolicy::Protect, WDA_MONITOR) => CaptureStatus::MonitorOnly,
            _ => CaptureStatus::Unavailable(CaptureError::UnexpectedAffinity(actual)),
        }
    }
}

pub(crate) struct Hook(HHOOK);

impl Hook {
    pub(crate) fn install(state: Rc<CaptureState>) -> Result<Self, CaptureError> {
        let occupied = STATE.with(|slot| {
            let previous = slot.take();
            let occupied = previous.is_some();
            slot.set(previous);
            occupied
        });
        if occupied {
            return Err(CaptureError::HookAlreadyInstalled);
        }
        // A thread-specific in-process callback needs no DLL or global injection.
        let hook = unsafe {
            SetWindowsHookExW(
                WH_CALLWNDPROC,
                Some(before_window_message),
                ptr::null_mut(),
                GetCurrentThreadId(),
            )
        };
        if hook.is_null() {
            return Err(CaptureError::HookInstallationFailed(unsafe {
                GetLastError()
            }));
        }
        STATE.with(|slot| slot.set(Some(state)));
        NESTED_SHOW.with(|skipped| skipped.set(false));
        Ok(Self(hook))
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        unsafe { UnhookWindowsHookEx(self.0) };
        STATE.with(|slot| slot.set(None));
        NESTED_SHOW.with(|skipped| skipped.set(false));
    }
}

unsafe extern "system" fn before_window_message(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code == HC_ACTION as i32 {
        // Windows supplies CWPSTRUCT, and WINDOWPOS only for the corresponding message.
        let message = unsafe { &*(lparam as *const CWPSTRUCT) };
        let showing = match message.message {
            WM_SHOWWINDOW => message.wParam != 0,
            WM_WINDOWPOSCHANGING if message.lParam != 0 => {
                let position = unsafe { &*(message.lParam as *const WINDOWPOS) };
                position.flags & SWP_SHOWWINDOW != 0
            }
            _ => false,
        };
        if showing && unsafe { GetAncestor(message.hwnd, GA_ROOT) } == message.hwnd {
            // Remove the slot while calling Win32, so nested messages cannot reenter it.
            STATE.with(|slot| {
                if let Some(state) = slot.take() {
                    let result = apply(message.hwnd, CapturePolicy::Protect);
                    let result = if NESTED_SHOW.with(|skipped| skipped.replace(false)) {
                        CaptureStatus::Unavailable(CaptureError::NestedWindowShow)
                    } else {
                        result
                    };
                    slot.set(Some(state.clone()));
                    state.record(result);
                } else {
                    NESTED_SHOW.with(|skipped| skipped.set(true));
                }
            });
        }
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, wparam, lparam) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_windows_uses_monitor_instead_of_claiming_capture_exclusion() {
        for (major, build) in [(6, 7601), (10, 18363), (10, 19040)] {
            assert_eq!(affinity_for_version(major, build), WDA_MONITOR);
        }
        for (major, build) in [(10, 19041), (10, 26100), (11, 1)] {
            assert_eq!(affinity_for_version(major, build), WDA_EXCLUDEFROMCAPTURE);
        }
    }

    #[test]
    fn skipped_nested_show_does_not_leave_a_successful_status() {
        let guard = crate::CaptureProtection::new(CapturePolicy::Protect);
        let wakeups = Rc::new(Cell::new(0));
        let observed = wakeups.clone();
        guard.set_wakeup(move || observed.set(observed.get() + 1));
        let window = unsafe {
            CreateWindowExW(
                0,
                windows_sys::core::w!("STATIC"),
                windows_sys::core::w!("PUBLIC NESTED SHOW"),
                WS_OVERLAPPEDWINDOW,
                -32000,
                -32000,
                100,
                100,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            )
        };
        assert!(!window.is_null());
        // Simulate the removed slot during an outer apply, using a real hook notification.
        STATE.with(|slot| {
            let outer = slot.take().unwrap();
            unsafe { SendMessageW(window, WM_SHOWWINDOW, 1, 0) };
            slot.set(Some(outer));
        });
        unsafe { SendMessageW(window, WM_SHOWWINDOW, 1, 0) };
        unsafe { DestroyWindow(window) };
        assert_eq!(
            guard.status(),
            CaptureStatus::Unavailable(CaptureError::NestedWindowShow)
        );
        assert!(wakeups.get() > 0, "a hook failure must wake the root UI");
    }
}
