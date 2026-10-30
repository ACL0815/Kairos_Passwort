//! Suppress unsolicited native shows while the root is deliberately in the tray.
//! eframe 0.37's first-render path unconditionally calls set_visible(true).
//! Modelled from the documented SetWindowSubclass/WM_WINDOWPOSCHANGING contracts.
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{cell::Cell, io, rc::Rc};
use windows_sys::Win32::{
    Foundation::*,
    UI::{
        Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
        WindowsAndMessaging::*,
    },
};

/// GUI-thread-only guard; attach while the borrowed native window is alive.
/// The callback holds its own Rc until removal or WM_NCDESTROY, whichever is first.
pub struct WindowVisibility {
    window: HWND,
    state: Rc<Cell<bool>>,
    token: usize,
}
impl WindowVisibility {
    pub fn attach(window: &impl HasWindowHandle, hidden: bool) -> io::Result<Self> {
        let handle = window
            .window_handle()
            .map_err(|_| io::Error::other("native window handle unavailable"))?;
        let RawWindowHandle::Win32(handle) = handle.as_raw() else {
            return Err(io::Error::other("not a Win32 window"));
        };
        let window = handle.hwnd.get() as HWND;
        let state = Rc::new(Cell::new(hidden));
        let token = Rc::into_raw(state.clone()) as usize;
        if unsafe { SetWindowSubclass(window, Some(window_proc), token, token) } == 0 {
            unsafe {
                drop(Rc::from_raw(token as *const Cell<bool>));
            }
            return Err(io::Error::other(
                "window visibility subclass could not be installed",
            ));
        }
        Ok(Self {
            window,
            state,
            token,
        })
    }
    pub fn set_hidden(&self, hidden: bool) {
        self.state.set(hidden);
    }
}
impl Drop for WindowVisibility {
    fn drop(&mut self) {
        // If WM_NCDESTROY already removed it, that callback released the raw Rc.
        // On an unexpected removal failure a still-installed callback retains its
        // reference, so it cannot access freed memory after this guard is dropped.
        if unsafe { RemoveWindowSubclass(self.window, Some(window_proc), self.token) } != 0 {
            unsafe {
                drop(Rc::from_raw(self.token as *const Cell<bool>));
            }
        }
    }
}
unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    token: usize,
    data: usize,
) -> LRESULT {
    let state = unsafe { &*(data as *const Cell<bool>) };
    if message == WM_WINDOWPOSCHANGING && lparam != 0 && state.get() {
        let position = unsafe { &mut *(lparam as *mut WINDOWPOS) };
        if position.flags & SWP_SHOWWINDOW != 0 {
            position.flags = (position.flags & !SWP_SHOWWINDOW) | SWP_HIDEWINDOW | SWP_NOACTIVATE;
        }
    }
    if message == WM_NCDESTROY
        && unsafe { RemoveWindowSubclass(window, Some(window_proc), token) } != 0
    {
        unsafe {
            drop(Rc::from_raw(data as *const Cell<bool>));
        }
    }
    unsafe { DefSubclassProc(window, message, wparam, lparam) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raw_window_handle::{HandleError, Win32WindowHandle, WindowHandle};
    use std::{num::NonZeroIsize, ptr};
    use windows_sys::core::w;
    struct Window(HWND);
    impl Window {
        fn new() -> Self {
            let class = WNDCLASSW {
                lpfnWndProc: Some(DefWindowProcW),
                lpszClassName: w!("KairosTest-visibility"),
                ..Default::default()
            };
            unsafe {
                RegisterClassW(&class);
            }
            let window = unsafe {
                CreateWindowExW(
                    WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                    class.lpszClassName,
                    w!("PUBLIC OFFSCREEN FIXTURE"),
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
            Self(window)
        }
    }
    impl HasWindowHandle for Window {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            Ok(unsafe {
                WindowHandle::borrow_raw(RawWindowHandle::Win32(Win32WindowHandle::new(
                    NonZeroIsize::new(self.0 as isize).unwrap(),
                )))
            })
        }
    }
    impl Drop for Window {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    DestroyWindow(self.0);
                }
            }
        }
    }
    #[test]
    fn automatic_first_show_is_suppressed_explicit_open_works_and_all_references_close() {
        let capture = crate::CaptureProtection::new(crate::CapturePolicy::Protect);
        let window = Window::new();
        let visibility = WindowVisibility::attach(&window, true).unwrap();
        let weak = Rc::downgrade(&visibility.state);
        unsafe {
            ShowWindow(window.0, SW_SHOWNOACTIVATE);
        }
        assert_eq!(
            unsafe { IsWindowVisible(window.0) },
            0,
            "eframe's automatic show must remain hidden"
        );
        visibility.set_hidden(false);
        unsafe {
            ShowWindow(window.0, SW_SHOWNOACTIVATE);
        }
        assert_ne!(unsafe { IsWindowVisible(window.0) }, 0);
        assert!(matches!(
            capture.status(),
            crate::CaptureStatus::Excluded | crate::CaptureStatus::MonitorOnly
        ));
        visibility.set_hidden(true);
        unsafe {
            ShowWindow(window.0, SW_HIDE);
            ShowWindow(window.0, SW_SHOWNOACTIVATE);
        }
        assert_eq!(unsafe { IsWindowVisible(window.0) }, 0);
        visibility.set_hidden(false);
        unsafe {
            ShowWindow(window.0, SW_SHOWNOACTIVATE);
        }
        let mut affinity = 0;
        assert_ne!(
            unsafe { GetWindowDisplayAffinity(window.0, &mut affinity) },
            0
        );
        assert!(matches!(affinity, WDA_EXCLUDEFROMCAPTURE | WDA_MONITOR));
        drop(visibility);
        assert!(weak.upgrade().is_none());
        let visibility = WindowVisibility::attach(&window, true).unwrap();
        let weak = Rc::downgrade(&visibility.state);
        unsafe {
            DestroyWindow(window.0);
        }
        let mut window = window;
        window.0 = ptr::null_mut();
        drop(visibility);
        assert!(
            weak.upgrade().is_none(),
            "WM_NCDESTROY must release the callback reference"
        );
    }
}
