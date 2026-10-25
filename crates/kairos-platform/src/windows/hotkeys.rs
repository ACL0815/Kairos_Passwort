//! The global-hotkey manager and blocking message pump share one thread.
//!
//! Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/src/platform_impl/windows/mod.rs>.

use crate::{hotkeys::EventSlot, FocusTarget, HotkeyBinding, HotkeyEvent, PlatformError};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use std::{
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Instant,
};
use windows_sys::Win32::{System::Threading::GetCurrentThreadId, UI::WindowsAndMessaging::*};

static RUNNING: AtomicBool = AtomicBool::new(false);
struct RunningGuard;
impl Drop for RunningGuard {
    fn drop(&mut self) {
        RUNNING.store(false, Ordering::Release);
    }
}

pub(crate) struct Service {
    events: Arc<EventSlot>,
    thread_id: u32,
    worker: Option<thread::JoinHandle<()>>,
}

impl Service {
    pub(crate) fn start(
        bindings: &[HotkeyBinding],
        wakeup: impl Fn() + Send + 'static,
    ) -> Result<Self, PlatformError> {
        RUNNING
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| PlatformError::HotkeysAlreadyRunning)?;
        let guard = RunningGuard;
        let bindings = bindings.to_vec();
        let events = Arc::new(EventSlot::new());
        let output = Arc::clone(&events);
        let (ready, started) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .name("kairos-hotkeys".into())
            .spawn(move || {
                let _guard = guard;
                let manager = match GlobalHotKeyManager::new() {
                    Ok(manager) => manager,
                    Err(_) => {
                        let _ = ready.send(Err(PlatformError::HotkeyUnavailable));
                        return;
                    }
                };
                let chords: Vec<_> = bindings
                    .iter()
                    .map(|b| HotKey::new(Some(b.hotkey.modifiers), b.hotkey.key))
                    .collect();
                for (index, chord) in chords.iter().enumerate() {
                    if manager.register(*chord).is_err() {
                        for registered in &chords[..index] {
                            let _ = manager.unregister(*registered);
                        }
                        let _ = ready.send(Err(PlatformError::HotkeyUnavailable));
                        return;
                    }
                }
                while GlobalHotKeyEvent::receiver().try_recv().is_ok() {}
                // Manager creation has already created this thread's message queue.
                if ready.send(Ok(unsafe { GetCurrentThreadId() })).is_err() {
                    return;
                }
                let mut message = MSG::default();
                while unsafe { GetMessageW(&mut message, ptr::null_mut(), 0, 0) } > 0 {
                    unsafe {
                        TranslateMessage(&message);
                        DispatchMessageW(&message);
                    }
                    while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                        if event.state != HotKeyState::Pressed {
                            continue;
                        }
                        if let Some(index) = chords.iter().position(|chord| chord.id() == event.id)
                        {
                            let target = FocusTarget::capture().and_then(|mut target| {
                                target.snapshot.trigger_key = Some(
                                    bindings[index]
                                        .hotkey
                                        .virtual_key()
                                        .ok_or(PlatformError::InvalidHotkey)?,
                                );
                                Ok(target)
                            });
                            let event = HotkeyEvent {
                                action: bindings[index].action,
                                target,
                                queued: Instant::now(),
                            };
                            if output.push(event).is_ok() {
                                wakeup();
                            }
                        }
                    }
                }
                for chord in chords {
                    let _ = manager.unregister(chord);
                }
            })
            .map_err(|_| PlatformError::WorkerUnavailable)?;
        match started.recv() {
            Ok(Ok(thread_id)) => Ok(Self {
                events,
                thread_id,
                worker: Some(worker),
            }),
            Ok(Err(error)) => {
                let _ = worker.join();
                Err(error)
            }
            Err(_) => {
                let _ = worker.join();
                Err(PlatformError::WorkerUnavailable)
            }
        }
    }

    pub(crate) fn try_recv(&self) -> Result<Option<HotkeyEvent>, PlatformError> {
        let event = self.events.take()?;
        if event.is_none()
            && self
                .worker
                .as_ref()
                .is_some_and(|worker| worker.is_finished())
        {
            return Err(PlatformError::WorkerUnavailable);
        }
        Ok(event)
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        unsafe {
            PostThreadMessageW(self.thread_id, WM_QUIT, 0, 0);
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
