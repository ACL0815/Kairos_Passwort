//! The global-hotkey manager and message pump always live on the same thread.
//!
//! Modelled after: global-hotkey <https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/src/platform_impl/windows/mod.rs>.

use crate::{FocusTarget, HotkeyBinding, HotkeyEvent, PlatformError};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use std::{
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

static RUNNING: AtomicBool = AtomicBool::new(false);
struct RunningGuard;
impl Drop for RunningGuard {
    fn drop(&mut self) {
        RUNNING.store(false, Ordering::Release);
    }
}

pub(crate) struct Service {
    receiver: mpsc::Receiver<Result<HotkeyEvent, PlatformError>>,
    stop: mpsc::Sender<()>,
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
        let (sender, receiver) = mpsc::sync_channel(1);
        let (stop, stopped) = mpsc::channel();
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
                // Drain release events from an earlier service instance.
                while GlobalHotKeyEvent::receiver().try_recv().is_ok() {}
                if ready.send(Ok(())).is_err() {
                    return;
                }
                'messages: loop {
                    let mut message = MSG::default();
                    while unsafe { PeekMessageW(&mut message, ptr::null_mut(), 0, 0, PM_REMOVE) }
                        != 0
                    {
                        if message.message == WM_QUIT {
                            break 'messages;
                        }
                        unsafe {
                            TranslateMessage(&message);
                            DispatchMessageW(&message);
                        }
                        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                            if event.state != HotKeyState::Pressed {
                                continue;
                            }
                            if let Some(index) =
                                chords.iter().position(|chord| chord.id() == event.id)
                            {
                                let event = FocusTarget::capture().map(|target| HotkeyEvent {
                                    action: bindings[index].action,
                                    target,
                                });
                                if sender.try_send(event).is_ok() {
                                    wakeup();
                                }
                            }
                        }
                    }
                    match stopped.recv_timeout(Duration::from_millis(5)) {
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        _ => break,
                    }
                }
                for chord in chords {
                    let _ = manager.unregister(chord);
                }
            })
            .map_err(|_| PlatformError::WorkerUnavailable)?;
        match started.recv() {
            Ok(Ok(())) => Ok(Self {
                receiver,
                stop,
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
        match self.receiver.try_recv() {
            Ok(event) => event.map(Some),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(PlatformError::WorkerUnavailable),
        }
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
