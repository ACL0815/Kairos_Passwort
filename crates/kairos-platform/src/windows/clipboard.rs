//! Transactional Windows clipboard writes, publishing the secret last.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp>
//! and arboard <https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/src/platform/windows.rs>.

use crate::{ClearOutcome, ClipboardReceipt, PlatformError};
use std::{
    ptr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime},
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
static TIMER: Mutex<Option<Arc<Timer>>> = Mutex::new(None);

struct Schedule {
    receipt: ClipboardReceipt,
    deadline: Instant,
    wall: SystemTime,
    retries: u8,
}
struct Timer {
    pending: Mutex<Option<Schedule>>,
    changed: Condvar,
}

fn retryable(error: PlatformError, retries: u8) -> bool {
    error == PlatformError::ClipboardUnavailable && retries < 50
}

fn timer() -> Result<Arc<Timer>, PlatformError> {
    let mut shared = TIMER.lock().map_err(|_| PlatformError::WorkerUnavailable)?;
    if let Some(timer) = shared.as_ref() {
        return Ok(Arc::clone(timer));
    }
    let timer = Arc::new(Timer {
        pending: Mutex::new(None),
        changed: Condvar::new(),
    });
    let worker = Arc::clone(&timer);
    thread::Builder::new()
        .name("kairos-clipboard-timeout".into())
        .spawn(move || {
            loop {
                let Ok(mut pending) = worker.pending.lock() else {
                    return;
                };
                let job = loop {
                    let wait = match pending.as_ref() {
                        Some(job)
                            if Instant::now() >= job.deadline || SystemTime::now() >= job.wall =>
                        {
                            break pending.take().unwrap()
                        }
                        Some(job) => job
                            .deadline
                            .saturating_duration_since(Instant::now())
                            .min(Duration::from_secs(1)),
                        None => {
                            pending = match worker.changed.wait(pending) {
                                Ok(guard) => guard,
                                Err(_) => return,
                            };
                            continue;
                        }
                    };
                    pending = match worker.changed.wait_timeout(pending, wait) {
                        Ok((guard, _)) => guard,
                        Err(_) => return,
                    };
                };
                drop(pending); // Never acquire OPERATIONS while holding the schedule lock.
                if let Err(error) = clear(job.receipt) {
                    if retryable(error, job.retries) {
                        let Ok(mut pending) = worker.pending.lock() else {
                            return;
                        };
                        // A new user copy wins over retrying an older receipt.
                        if pending.is_none() {
                            *pending = Some(Schedule {
                                receipt: job.receipt,
                                deadline: Instant::now() + Duration::from_millis(100),
                                wall: SystemTime::now() + Duration::from_millis(100),
                                retries: job.retries + 1,
                            });
                        }
                    }
                }
            }
        })
        .map_err(|_| PlatformError::WorkerUnavailable)?;
    // Failed starts leave None, so a later user action may try again.
    *shared = Some(Arc::clone(&timer));
    Ok(timer)
}

pub(crate) fn copy(
    text: &str,
    clear_after: Option<Duration>,
) -> Result<ClipboardReceipt, PlatformError> {
    let deadline = match clear_after {
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
        if let (Some(timer), Some(deadline), Some(duration)) =
            (timeout_sender, deadline, clear_after)
        {
            let wall = SystemTime::now()
                .checked_add(duration)
                .ok_or(PlatformError::InvalidTimeout)?;
            *timer
                .pending
                .lock()
                .map_err(|_| PlatformError::WorkerUnavailable)? = Some(Schedule {
                receipt,
                deadline,
                wall,
                retries: 0,
            });
            timer.changed.notify_one();
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
    if unsafe { CountClipboardFormats() } == 0 {
        return Ok(ClearOutcome::NotPresent);
    }
    let marker_format = registered_formats()?[3];
    let marker = unsafe {
        if IsClipboardFormatAvailable(marker_format) == 0 {
            return Ok(ClearOutcome::Replaced);
        }
        let handle = GetClipboardData(marker_format);
        if handle.is_null() {
            return Err(PlatformError::ClipboardUnavailable);
        }
        if GlobalSize(handle) < size_of::<u64>() {
            None
        } else {
            let data = GlobalLock(handle);
            if data.is_null() {
                return Err(PlatformError::ClipboardUnavailable);
            } else {
                let value = ptr::read_unaligned(data.cast::<u64>());
                GlobalUnlock(handle);
                Some(u64::from_le(value))
            }
        }
    };
    if !receipt.matches(marker) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retries_only_transient_failures_with_a_finite_budget() {
        assert!(retryable(PlatformError::ClipboardUnavailable, 49));
        assert!(!retryable(PlatformError::ClipboardUnavailable, 50));
        assert!(!retryable(PlatformError::ClipboardProtectionFailed, 0));
        assert!(!retryable(PlatformError::WorkerUnavailable, 0));
    }
}
