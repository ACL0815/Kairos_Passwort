//! Capture policy for borrowed native windows and future windows on the GUI thread.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp>.
//! Only the default-on policy and Win32 display-affinity principle are modelled.

use raw_window_handle::HasWindowHandle;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// A per-process launch choice, never persisted as an insecure default.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CapturePolicy {
    #[default]
    Protect,
    Allow,
}

/// Public diagnostics contain only an operation and an OS code, never window text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptureError {
    UnsupportedPlatform,
    WindowHandleUnavailable,
    HookAlreadyInstalled,
    HookInstallationFailed(u32),
    VersionUnavailable(i32),
    SetAffinityFailed(u32),
    ReadAffinityFailed(u32),
    UnexpectedAffinity(u32),
    NestedWindowShow,
}

/// The weakest observed result across windows. Failures remain visible for this run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptureStatus {
    Pending,
    Excluded,
    MonitorOnly,
    Allowed,
    Unavailable(CaptureError),
}

impl CaptureStatus {
    pub(crate) fn record(self, next: Self) -> Self {
        match (self, next) {
            (Self::Unavailable(_), _) => self,
            (_, Self::Unavailable(_)) => next,
            (Self::MonitorOnly, _) | (_, Self::MonitorOnly) => Self::MonitorOnly,
            _ => next,
        }
    }
}

/// Applies and reads back the policy while the caller keeps the native window alive.
/// This is an OS capture hint, not a boundary against cameras or privileged tools.
pub fn set_window_capture_policy(
    window: &impl HasWindowHandle,
    policy: CapturePolicy,
) -> CaptureStatus {
    let Ok(handle) = window.window_handle() else {
        return CaptureStatus::Unavailable(CaptureError::WindowHandleUnavailable);
    };
    #[cfg(windows)]
    {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
            return crate::windows::capture::apply(handle.hwnd.get() as _, policy);
        }
    }
    let _ = (handle, policy);
    CaptureStatus::Unavailable(CaptureError::UnsupportedPlatform)
}

/// Owns a thread-local window hook. Create before `run_native`, then keep it in the app.
/// All top-level windows shown on that thread, including later viewports and native
/// dialogs, receive protection before display. egui windows drawn inside a viewport
/// share its protection. A separate GUI thread needs its own guard.
/// The guard cannot move between threads. It does not inspect other applications.
/// Visibility set only through `GWL_STYLE | WS_VISIBLE` is not observed. Apply the
/// policy explicitly before such a style change on a content-bearing window.
/// Register `set_wakeup` with the GUI's root repaint callback so hook failures
/// become visible immediately, including failures in later native viewports.
pub struct CaptureProtection {
    policy: CapturePolicy,
    state: Rc<CaptureState>,
    #[cfg(windows)]
    _hook: Option<crate::windows::capture::Hook>,
}

pub(crate) struct CaptureState {
    status: Cell<CaptureStatus>,
    wakeup: RefCell<Option<Box<dyn Fn()>>>,
}

impl CaptureState {
    pub(crate) fn record(&self, next: CaptureStatus) {
        self.status.set(self.status.get().record(next));
        if matches!(next, CaptureStatus::Unavailable(_)) {
            if let Some(wakeup) = self.wakeup.borrow().as_ref() {
                wakeup();
            }
        }
    }
}

impl CaptureProtection {
    pub fn new(policy: CapturePolicy) -> Self {
        let state = Rc::new(CaptureState {
            status: Cell::new(match policy {
                CapturePolicy::Protect => CaptureStatus::Pending,
                CapturePolicy::Allow => CaptureStatus::Allowed,
            }),
            wakeup: RefCell::new(None),
        });
        #[cfg(windows)]
        let hook = if policy == CapturePolicy::Protect {
            match crate::windows::capture::Hook::install(state.clone()) {
                Ok(hook) => Some(hook),
                Err(error) => {
                    state.record(CaptureStatus::Unavailable(error));
                    None
                }
            }
        } else {
            None
        };
        #[cfg(not(windows))]
        if policy == CapturePolicy::Protect {
            state.record(CaptureStatus::Unavailable(
                CaptureError::UnsupportedPlatform,
            ));
        }
        Self {
            policy,
            state,
            #[cfg(windows)]
            _hook: hook,
        }
    }

    /// Initialize the root from eframe's borrowed raw handle before rendering secrets.
    pub fn apply_to(&self, window: &impl HasWindowHandle) -> CaptureStatus {
        let result = set_window_capture_policy(window, self.policy);
        self.state.record(result);
        result
    }

    /// Called on the GUI thread after a capture failure has been recorded.
    /// The callback should only enqueue a root repaint, without opening windows.
    pub fn set_wakeup(&self, wakeup: impl Fn() + 'static) {
        *self.state.wakeup.borrow_mut() = Some(Box::new(wakeup));
        if matches!(self.status(), CaptureStatus::Unavailable(_)) {
            self.state.record(self.status());
        }
    }

    pub fn status(&self) -> CaptureStatus {
        self.state.status.get()
    }

    pub fn policy(&self) -> CapturePolicy {
        self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_protects_and_any_window_failure_remains_visible() {
        assert_eq!(CapturePolicy::default(), CapturePolicy::Protect);
        let failure = CaptureStatus::Unavailable(CaptureError::SetAffinityFailed(5));
        let mut status = CaptureStatus::Pending.record(CaptureStatus::Excluded);
        assert_eq!(status, CaptureStatus::Excluded);
        status = status.record(CaptureStatus::MonitorOnly);
        assert_eq!(
            status.record(CaptureStatus::Excluded),
            CaptureStatus::MonitorOnly
        );
        status = status.record(failure);
        assert_eq!(status.record(CaptureStatus::Excluded), failure);
    }

    #[test]
    fn explicit_opt_out_is_immediate() {
        assert_eq!(
            CaptureProtection::new(CapturePolicy::Allow).status(),
            CaptureStatus::Allowed
        );
    }
}
