//! Transactional Windows clipboard writes, publishing the secret last.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp>
//! and arboard <https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/src/platform/windows.rs>.

use crate::{ClearOutcome, ClipboardReceipt, PlatformError};
use std::{
    ptr,
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc, Mutex, OnceLock,
    },
    thread,
    time::{Duration, Instant},
};
use windows_sys::{
    core::w,
    Win32::{
        Foundation::{GlobalFree, HGLOBAL, HWND},
        System::{DataExchange::*, Memory::*, Threading::GetCurrentProcessId},
        UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, HWND_MESSAGE},
    },
};
use zeroize::Zeroize;

const CF_UNICODETEXT: u32 = 13;
static OPERATIONS: Mutex<()> = Mutex::new(());
static NEXT_MARKER: AtomicU64 = AtomicU64::new(1);
static TIMER: OnceLock<Result<mpsc::SyncSender<Schedule>, PlatformError>> = OnceLock::new();

struct Schedule {
    receipt: ClipboardReceipt,
    deadline: Instant,
}

fn timer() -> Result<&'static mpsc::SyncSender<Schedule>, PlatformError> {
    TIMER
        .get_or_init(|| {
            let (sender, receiver) = mpsc::sync_channel::<Schedule>(1);
            thread::Builder::new()
                .name("kairos-clipboard-timeout".into())
                .spawn(move || {
                    let mut pending: Option<Schedule> = None;
                    loop {
                        let message = match &pending {
                            Some(item) => receiver.recv_timeout(
                                item.deadline.saturating_duration_since(Instant::now()),
                            ),
                            None => receiver
                                .recv()
                                .map_err(|_| mpsc::RecvTimeoutError::Disconnected),
                        };
                        match message {
                            Ok(item) => pending = Some(item),
                            Err(mpsc::RecvTimeoutError::Timeout) => {
                                if let Some(item) = pending.take() {
                                    // Retry transient clipboard contention; a new request
                                    // supersedes this receipt without retaining text.
                                    if clear(item.receipt).is_err() {
                                        pending = Some(Schedule {
                                            receipt: item.receipt,
                                            deadline: Instant::now() + Duration::from_millis(100),
                                        });
                                    }
                                }
                            }
                            Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        }
                    }
                })
                .map_err(|_| PlatformError::WorkerUnavailable)?;
            Ok(sender)
        })
        .as_ref()
        .map_err(|e| *e)
}

pub(crate) fn copy(
    text: &str,
    clear_after: Option<Duration>,
) -> Result<ClipboardReceipt, PlatformError> {
    let deadline = match clear_after {
        Some(duration) if duration > Duration::from_secs(86_400) => {
            return Err(PlatformError::InvalidTimeout)
        }
        Some(duration) => Some(
            Instant::now()
                .checked_add(duration)
                .ok_or(PlatformError::InvalidTimeout)?,
        ),
        None => None,
    };
    let timeout_sender = if deadline.is_some() {
        Some(timer()?)
    } else {
        None
    };
    let formats = registered_formats()?;
    let marker = NEXT_MARKER.fetch_add(1, Ordering::Relaxed)
        ^ ((unsafe { GetCurrentProcessId() } as u64) << 32);
    // Allocate before changing the clipboard. These buffers scrub on failure.
    let flags = [
        GlobalBuffer::bytes(&0u32.to_le_bytes())?,
        GlobalBuffer::bytes(&0u32.to_le_bytes())?,
        GlobalBuffer::bytes(&0u32.to_le_bytes())?,
    ];
    let identity = GlobalBuffer::bytes(&marker.to_le_bytes())?;
    let secret = GlobalBuffer::text(text)?;
    let _operation = OPERATIONS
        .lock()
        .map_err(|_| PlatformError::ClipboardUnavailable)?;
    let _clipboard = OpenClipboardGuard::open()?;
    if unsafe { EmptyClipboard() } == 0 {
        return Err(PlatformError::ClipboardUnavailable);
    }
    let result = (|| {
        for (format, flag) in formats[..3].iter().zip(flags) {
            flag.publish(*format)
                .map_err(|_| PlatformError::ClipboardProtectionFailed)?;
        }
        identity
            .publish(formats[3])
            .map_err(|_| PlatformError::ClipboardProtectionFailed)?;
        // No secret can be exposed with missing protection metadata.
        secret.publish(CF_UNICODETEXT)?;
        let sequence = unsafe { GetClipboardSequenceNumber() };
        if sequence == 0 {
            return Err(PlatformError::ClipboardUnavailable);
        }
        let receipt = ClipboardReceipt { sequence, marker };
        if let (Some(sender), Some(deadline)) = (timeout_sender, deadline) {
            sender
                .try_send(Schedule { receipt, deadline })
                .map_err(|_| PlatformError::WorkerUnavailable)?;
        }
        Ok(receipt)
    })();
    if result.is_err() {
        // Still locked: rollback cannot erase a concurrent application's write.
        unsafe {
            EmptyClipboard();
        }
    }
    result
}

pub(crate) fn clear(receipt: ClipboardReceipt) -> Result<ClearOutcome, PlatformError> {
    let _operation = OPERATIONS
        .lock()
        .map_err(|_| PlatformError::ClipboardUnavailable)?;
    let _clipboard = OpenClipboardGuard::open()?;
    let sequence = unsafe { GetClipboardSequenceNumber() };
    let marker_format = registered_formats()?[3];
    let marker = unsafe {
        let handle = GetClipboardData(marker_format);
        if handle.is_null() || GlobalSize(handle) < size_of::<u64>() {
            None
        } else {
            let data = GlobalLock(handle);
            if data.is_null() {
                None
            } else {
                let value = ptr::read_unaligned(data.cast::<u64>());
                GlobalUnlock(handle);
                Some(u64::from_le(value))
            }
        }
    };
    if !receipt.matches(sequence, marker) {
        return Ok(ClearOutcome::Replaced);
    }
    if unsafe { EmptyClipboard() } == 0 {
        return Err(PlatformError::ClipboardUnavailable);
    }
    Ok(ClearOutcome::Cleared)
}

fn registered_formats() -> Result<[u32; 4], PlatformError> {
    // Literal names are part of Microsoft's clipboard protocol.
    let formats = unsafe {
        [
            RegisterClipboardFormatW(w!("ExcludeClipboardContentFromMonitorProcessing")),
            RegisterClipboardFormatW(w!("CanIncludeInClipboardHistory")),
            RegisterClipboardFormatW(w!("CanUploadToCloudClipboard")),
            RegisterClipboardFormatW(w!("KairosPassword.ClipboardOwner.v1")),
        ]
    };
    if formats.contains(&0) {
        Err(PlatformError::ClipboardProtectionFailed)
    } else {
        Ok(formats)
    }
}

/// A real message-only owner avoids the documented NULL-owner failure after
/// EmptyClipboard. No delayed rendering; Windows owns published HGLOBALs.
struct OpenClipboardGuard {
    owner: HWND,
}

impl OpenClipboardGuard {
    fn open() -> Result<Self, PlatformError> {
        let owner = unsafe {
            CreateWindowExW(
                0,
                w!("STATIC"),
                ptr::null(),
                0,
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            )
        };
        if owner.is_null() {
            return Err(PlatformError::ClipboardUnavailable);
        }
        for _ in 0..20 {
            if unsafe { OpenClipboard(owner) } != 0 {
                return Ok(Self { owner });
            }
            thread::sleep(Duration::from_millis(5));
        }
        unsafe {
            DestroyWindow(owner);
        }
        Err(PlatformError::ClipboardUnavailable)
    }
}

impl Drop for OpenClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
            DestroyWindow(self.owner);
        }
    }
}

struct GlobalBuffer {
    handle: HGLOBAL,
    bytes: usize,
}

impl GlobalBuffer {
    fn allocate(bytes: usize) -> Result<Self, PlatformError> {
        let handle = unsafe { GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, bytes) };
        if handle.is_null() {
            Err(PlatformError::ClipboardUnavailable)
        } else {
            Ok(Self { handle, bytes })
        }
    }

    fn bytes(data: &[u8]) -> Result<Self, PlatformError> {
        let buffer = Self::allocate(data.len())?;
        let target = unsafe { GlobalLock(buffer.handle) };
        if target.is_null() {
            return Err(PlatformError::ClipboardUnavailable);
        }
        // SAFETY: allocated data.len() writable bytes; buffers do not overlap.
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), target.cast(), data.len());
            GlobalUnlock(buffer.handle);
        }
        Ok(buffer)
    }

    fn text(text: &str) -> Result<Self, PlatformError> {
        let units = text.encode_utf16().count() + 1;
        let buffer = Self::allocate(units * size_of::<u16>())?;
        let target = unsafe { GlobalLock(buffer.handle) }.cast::<u16>();
        if target.is_null() {
            return Err(PlatformError::ClipboardUnavailable);
        }
        // SAFETY: allocation has room for all UTF-16 units plus the zeroed NUL.
        for (index, unit) in text.encode_utf16().enumerate() {
            unsafe {
                target.add(index).write(unit);
            }
        }
        unsafe {
            GlobalUnlock(buffer.handle);
        }
        Ok(buffer)
    }

    fn publish(mut self, format: u32) -> Result<(), PlatformError> {
        if unsafe { SetClipboardData(format, self.handle) }.is_null() {
            return Err(PlatformError::ClipboardUnavailable);
        }
        // Successful SetClipboardData transfers allocation ownership to Windows.
        self.handle = ptr::null_mut();
        Ok(())
    }
}

impl Drop for GlobalBuffer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let data = GlobalLock(self.handle);
                if !data.is_null() {
                    // SAFETY: only our untransferred, initialized allocation.
                    std::slice::from_raw_parts_mut(data.cast::<u8>(), self.bytes).zeroize();
                    GlobalUnlock(self.handle);
                }
                GlobalFree(self.handle);
            }
        }
    }
}
