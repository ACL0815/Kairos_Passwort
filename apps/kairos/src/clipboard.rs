//! Temporary adapter until kairos-platform is wired in a later integration PR.
//!
//! Modelled after: KeePassXC https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp
//! Modelled after: arboard https://github.com/1Password/arboard/tree/5f5c8637911ba219edee619105a19b4f74e66a9f/src
//! A worker owns the clipboard timer so minimizing the GUI cannot suspend cleanup.

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender, SyncSender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy)]
struct Lease {
    sequence: u32,
    marker: [u8; 12],
}

#[cfg(windows)]
const LEASE_FORMAT: &str = "Kairos.Password.ClipboardLease";

enum Command {
    Copy(Zeroizing<String>, SyncSender<Result<(), ()>>),
    Paste(SyncSender<Result<Zeroizing<String>, ()>>),
    Shutdown,
}

pub struct ProtectedClipboard {
    sender: Sender<Command>,
    worker: Option<JoinHandle<()>>,
}

impl ProtectedClipboard {
    pub fn new() -> Self {
        Self::with_timeout(TIMEOUT)
    }

    fn with_timeout(timeout: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let worker = thread::spawn(move || run_worker(receiver, timeout));
        Self {
            sender,
            worker: Some(worker),
        }
    }

    pub fn copy(&self, text: &str) -> Result<(), ()> {
        if text.is_empty() || text.contains('\0') {
            return Err(());
        }
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .send(Command::Copy(Zeroizing::new(text.to_owned()), sender))
            .map_err(|_| ())?;
        receiver.recv().map_err(|_| ())?
    }

    pub fn paste(&self) -> Result<Zeroizing<String>, ()> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender.send(Command::Paste(sender)).map_err(|_| ())?;
        receiver.recv().map_err(|_| ())?
    }
}

impl Drop for ProtectedClipboard {
    fn drop(&mut self) {
        let _ = self.sender.send(Command::Shutdown);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn run_worker(receiver: Receiver<Command>, timeout: Duration) {
    let mut lease: Option<(Lease, Instant)> = None;
    loop {
        let wait = lease.map_or(Duration::from_secs(3600), |(_, deadline)| {
            deadline.saturating_duration_since(Instant::now())
        });
        match receiver.recv_timeout(wait) {
            Ok(Command::Paste(response)) => {
                let result = arboard::Clipboard::new()
                    .and_then(|mut clipboard| clipboard.get_text())
                    .map(Zeroizing::new)
                    .map_err(|_| ());
                let _ = response.send(result);
            }
            Ok(Command::Copy(text, response)) => {
                let result = set_protected(&text);
                if let Ok(sequence) = result {
                    lease = Some((sequence, Instant::now() + timeout));
                }
                let _ = response.send(result.map(|_| ()));
            }
            Ok(Command::Shutdown) | Err(RecvTimeoutError::Disconnected) => {
                if let Some((sequence, _)) = lease {
                    let _ = clear_if_current(sequence);
                }
                break;
            }
            Err(RecvTimeoutError::Timeout) => {
                if let Some((sequence, _)) = lease {
                    lease = if clear_if_current(sequence).is_err() {
                        Some((sequence, Instant::now() + Duration::from_millis(250)))
                    } else {
                        None
                    };
                }
            }
        }
    }
}

#[cfg(windows)]
fn set_protected(text: &str) -> Result<Lease, ()> {
    use arboard::SetExtWindows;
    let mut clipboard = arboard::Clipboard::new().map_err(|_| ())?;
    let result = clipboard
        .set()
        .exclude_from_cloud()
        .exclude_from_history()
        .text(text);
    let _lock = clipboard_win::Clipboard::new_attempts(10).map_err(|_| ())?;
    let mut current = Zeroizing::new(Vec::new());
    clipboard_win::raw::get_string(&mut current).map_err(|_| ())?;
    if current.as_slice() != text.as_bytes() {
        return Err(());
    }
    if result.is_err() {
        // arboard may have written the text before a protection format failed.
        let _ = clipboard_win::raw::empty();
        return Err(());
    }
    static NEXT_MARKER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut marker = [0; 12];
    marker[..4].copy_from_slice(&std::process::id().to_le_bytes());
    marker[4..].copy_from_slice(
        &NEXT_MARKER
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            .to_le_bytes(),
    );
    let format = clipboard_win::register_format(LEASE_FORMAT).ok_or(())?;
    if clipboard_win::raw::set_without_clear(format.get(), &marker).is_err() {
        let _ = clipboard_win::raw::empty();
        return Err(());
    }
    clipboard_win::raw::seq_num()
        .map(|value| Lease {
            sequence: value.get(),
            marker,
        })
        .ok_or(())
}

#[cfg(windows)]
fn clear_if_current(lease: Lease) -> Result<(), ()> {
    let _lock = clipboard_win::Clipboard::new_attempts(10).map_err(|_| ())?;
    let current = clipboard_win::raw::seq_num().map(|value| value.get());
    let format = clipboard_win::register_format(LEASE_FORMAT).ok_or(())?;
    let mut marker = Vec::new();
    let owned =
        clipboard_win::raw::get_vec(format.get(), &mut marker).is_ok() && marker == lease.marker;
    if lease_matches(lease.sequence, current, owned) {
        // Check and clear hold the same OS lock, preventing a check/clear race.
        clipboard_win::raw::empty().map_err(|_| ())?;
    }
    Ok(())
}

fn lease_matches(ours: u32, current: Option<u32>, owned: bool) -> bool {
    owned && current == Some(ours)
}

#[cfg(not(windows))]
fn set_protected(_text: &str) -> Result<Lease, ()> {
    // Fail closed where the Windows privacy formats cannot be guaranteed.
    Err(())
}

#[cfg(not(windows))]
fn clear_if_current(_lease: Lease) -> Result<(), ()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_never_clears_a_later_or_foreign_clipboard_write() {
        assert!(lease_matches(42, Some(42), true));
        assert!(!lease_matches(42, Some(43), true));
        assert!(!lease_matches(42, Some(42), false));
        assert!(!lease_matches(42, None, true));
    }

    #[test]
    fn empty_and_nul_containing_values_do_not_touch_the_clipboard() {
        let clipboard = ProtectedClipboard::new();
        assert!(clipboard.copy("").is_err());
        assert!(clipboard.copy("fake\0data").is_err());
    }

    #[cfg(windows)]
    #[test]
    #[ignore = "Writes public fixtures to the Windows clipboard; run explicitly and serially"]
    fn windows_privacy_flags_timeout_and_foreign_replacement() {
        let clipboard = ProtectedClipboard::with_timeout(Duration::from_millis(150));
        clipboard.copy("KAIROS PUBLIC CLIPBOARD FIXTURE").unwrap();
        {
            let _lock = clipboard_win::Clipboard::new_attempts(10).unwrap();
            for name in ["CanIncludeInClipboardHistory", "CanUploadToCloudClipboard"] {
                let format = clipboard_win::register_format(name).unwrap();
                let mut data = Vec::new();
                clipboard_win::raw::get_vec(format.get(), &mut data).unwrap();
                assert!(data.iter().all(|byte| *byte == 0));
                assert!(data.len() >= 4);
            }
        }
        let mut reader = arboard::Clipboard::new().unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        while reader.get_text().is_ok() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(20));
        }
        assert!(reader.get_text().is_err(), "timeout must clear our fixture");
        clipboard.copy("KAIROS PUBLIC SECOND FIXTURE").unwrap();
        reader.set_text("PUBLIC FOREIGN REPLACEMENT").unwrap();
        thread::sleep(Duration::from_millis(300));
        assert_eq!(reader.get_text().unwrap(), "PUBLIC FOREIGN REPLACEMENT");
        drop(clipboard);
        assert_eq!(reader.get_text().unwrap(), "PUBLIC FOREIGN REPLACEMENT");
        reader.clear().unwrap();
    }
}
