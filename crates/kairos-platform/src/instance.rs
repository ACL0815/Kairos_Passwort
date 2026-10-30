//! Session-local, user-specific single instance with queued show/popup events.
//! No windows, polling, UI libraries, or hotkey registration are needed here.
use std::{io, os::windows::ffi::OsStrExt, ptr, sync::Arc, thread};
use windows_sys::Win32::{
    Foundation::*,
    Security::{
        Authorization::ConvertSidToStringSidW, GetTokenInformation, TokenUser, TOKEN_QUERY,
        TOKEN_USER,
    },
    System::Threading::*,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceRequest {
    Show,
    Popup,
}
struct Handle(HANDLE);
// Kernel handles are reference-counted by Windows and support cross-thread waits.
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
fn handle(raw: HANDLE) -> io::Result<Handle> {
    if raw.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(Handle(raw))
    }
}
fn wide(text: &str) -> Vec<u16> {
    std::ffi::OsStr::new(text)
        .encode_wide()
        .chain(Some(0))
        .collect()
}
fn event(name: Option<&str>) -> io::Result<Arc<Handle>> {
    let name = name.map(wide);
    handle(unsafe {
        CreateEventW(
            ptr::null(),
            0,
            0,
            name.as_ref().map_or(ptr::null(), |n| n.as_ptr()),
        )
    })
    .map(Arc::new)
}

pub fn session_name() -> io::Result<String> {
    let mut raw = ptr::null_mut();
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut raw) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let token = handle(raw)?;
    let mut length = 0;
    unsafe {
        GetTokenInformation(token.0, TokenUser, ptr::null_mut(), 0, &mut length);
    }
    if length == 0 {
        return Err(io::Error::last_os_error());
    }
    // usize storage supplies the alignment required by TOKEN_USER.
    let mut buffer = vec![0_usize; (length as usize).div_ceil(size_of::<usize>())];
    if unsafe {
        GetTokenInformation(
            token.0,
            TokenUser,
            buffer.as_mut_ptr().cast(),
            length,
            &mut length,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let user = unsafe { &*buffer.as_ptr().cast::<TOKEN_USER>() };
    let mut sid = ptr::null_mut();
    if unsafe { ConvertSidToStringSidW(user.User.Sid, &mut sid) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut len = 0;
    unsafe {
        while *sid.add(len) != 0 {
            len += 1;
        }
    }
    let text = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(sid, len) });
    unsafe {
        LocalFree(sid.cast());
    }
    Ok(format!(r"Local\Kairos.Passwort.v1.{text}"))
}

pub struct SingleInstance {
    _mutex: Handle,
    show: Arc<Handle>,
    popup: Arc<Handle>,
    stop: Arc<Handle>,
    worker: Option<thread::JoinHandle<()>>,
}
impl SingleInstance {
    /// None means another instance was signalled successfully. Events are created
    /// before the mutex, so a second launch during GUI startup cannot lose its wakeup.
    pub fn acquire(name: &str, request: InstanceRequest) -> io::Result<Option<Self>> {
        let show = event(Some(&format!("{name}.show")))?;
        let popup = event(Some(&format!("{name}.popup")))?;
        let mutex = unsafe { CreateMutexW(ptr::null(), 0, wide(name).as_ptr()) };
        let last_error = unsafe { GetLastError() };
        let mutex = handle(mutex)?;
        if last_error == ERROR_ALREADY_EXISTS {
            let signal = match request {
                InstanceRequest::Show => &show,
                InstanceRequest::Popup => &popup,
            };
            if unsafe { SetEvent(signal.0) } == 0 {
                return Err(io::Error::last_os_error());
            }
            return Ok(None);
        }
        Ok(Some(Self {
            _mutex: mutex,
            show,
            popup,
            stop: event(None)?,
            worker: None,
        }))
    }
    pub fn listen(&mut self, notify: impl Fn(InstanceRequest) + Send + 'static) -> io::Result<()> {
        if self.worker.is_some() {
            return Err(io::Error::other("instance listener already running"));
        }
        let (show, popup, stop) = (self.show.clone(), self.popup.clone(), self.stop.clone());
        self.worker = Some(
            thread::Builder::new()
                .name("kairos-instance".into())
                .spawn(move || {
                    let handles = [stop.0, show.0, popup.0];
                    loop {
                        match unsafe { WaitForMultipleObjects(3, handles.as_ptr(), 0, INFINITE) } {
                            WAIT_OBJECT_0 => break,
                            n if n == WAIT_OBJECT_0 + 1 => notify(InstanceRequest::Show),
                            n if n == WAIT_OBJECT_0 + 2 => notify(InstanceRequest::Popup),
                            _ => break,
                        }
                    }
                })?,
        );
        Ok(())
    }
}
impl Drop for SingleInstance {
    fn drop(&mut self) {
        unsafe {
            SetEvent(self.stop.0);
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn secondary_signals_both_requests_before_listener_and_handles_are_cleaned_up() {
        let name = format!(r"Local\KairosTest-{}-instance", std::process::id());
        let mut first = SingleInstance::acquire(&name, InstanceRequest::Show)
            .unwrap()
            .unwrap();
        assert!(SingleInstance::acquire(&name, InstanceRequest::Popup)
            .unwrap()
            .is_none());
        let (send, recv) = std::sync::mpsc::channel();
        first
            .listen(move |r| {
                send.send(r).unwrap();
            })
            .unwrap();
        assert_eq!(
            recv.recv_timeout(std::time::Duration::from_secs(2))
                .unwrap(),
            InstanceRequest::Popup
        );
        assert!(SingleInstance::acquire(&name, InstanceRequest::Show)
            .unwrap()
            .is_none());
        assert_eq!(
            recv.recv_timeout(std::time::Duration::from_secs(2))
                .unwrap(),
            InstanceRequest::Show
        );
        drop(first);
        let reopened = unsafe { OpenMutexW(SYNCHRONIZATION_SYNCHRONIZE, 0, wide(&name).as_ptr()) };
        assert!(reopened.is_null(), "all test mutex handles must close");
        let again = SingleInstance::acquire(&name, InstanceRequest::Show).unwrap();
        assert!(again.is_some());
        drop(again);
        println!("CLEANED: {name} and .show/.popup events (kernel handles closed)");
    }
}
