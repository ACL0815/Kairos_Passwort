//! Modelled after: KeePassXC Bootstrap.cpp, independently implemented with Win32.
//! <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp>.

use std::{mem::size_of, ptr};
use windows_sys::Win32::{
    Foundation::{GetLastError, LocalFree, ERROR_INVALID_DATA},
    Security::{
        Authorization::ConvertStringSecurityDescriptorToSecurityDescriptorW,
        SetKernelObjectSecurity, DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    },
    System::{
        ErrorReporting::{
            WerGetFlags, WerSetFlags, WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH,
            WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG, WER_FAULT_REPORTING_FLAG_NOHEAP,
            WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE,
        },
        Threading::{
            GetCurrentProcess, GetProcessMitigationPolicy, ProcessExtensionPointDisablePolicy,
            ProcessImageLoadPolicy, SetProcessMitigationPolicy, PROCESS_MITIGATION_POLICY,
        },
    },
};

use crate::{HardeningReport, Measure, Outcome};

fn win32(success: i32) -> Outcome {
    if success != 0 {
        Outcome::Applied
    } else {
        // SAFETY: no intervening Win32 operation before retrieving this error.
        Outcome::Failed(unsafe { GetLastError() })
    }
}

fn wer() -> Outcome {
    let mut flags = 0;
    // SAFETY: writable DWORD and current process pseudo handle.
    let status = unsafe { WerGetFlags(GetCurrentProcess(), &mut flags) };
    // A fresh native process has no WER context yet. On Windows this read
    // returns HRESULT_FROM_WIN32(ERROR_NOT_FOUND), confirmed by the child test.
    // Only that initial absence is treated as default flags; all other errors
    // and every post-set read failure remain visible in the report.
    if status < 0 && status as u32 != 0x80070490 {
        return Outcome::Failed(status as u32);
    }
    let required = WER_FAULT_REPORTING_FLAG_NOHEAP
        | WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE
        | WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH
        | WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG;
    // SAFETY: only documented process-local reporting flags, retaining old flags.
    let status = unsafe { WerSetFlags(flags | required) };
    if status < 0 {
        return Outcome::Failed(status as u32);
    }
    // SAFETY: same valid output and pseudo handle as above.
    let status = unsafe { WerGetFlags(GetCurrentProcess(), &mut flags) };
    if status < 0 {
        Outcome::Failed(status as u32)
    } else if flags & required != required {
        Outcome::Failed(ERROR_INVALID_DATA)
    } else {
        Outcome::Applied
    }
}

/// These two Win32 policy structures contain exactly one DWORD of flags.
fn mitigation(policy: PROCESS_MITIGATION_POLICY, required: u32) -> Outcome {
    let mut flags: u32 = 0;
    // SAFETY: callers only pass the two policies with a single DWORD layout.
    let result = win32(unsafe {
        GetProcessMitigationPolicy(
            GetCurrentProcess(),
            policy,
            (&mut flags as *mut u32).cast(),
            size_of::<u32>(),
        )
    });
    if result != Outcome::Applied {
        return result;
    }
    flags |= required;
    // SAFETY: policy-specific documented flags with the correct DWORD size.
    let result = win32(unsafe {
        SetProcessMitigationPolicy(policy, (&flags as *const u32).cast(), size_of::<u32>())
    });
    if result != Outcome::Applied {
        return result;
    }
    // SAFETY: read back the same policy and check effective requested bits.
    let result = win32(unsafe {
        GetProcessMitigationPolicy(
            GetCurrentProcess(),
            policy,
            (&mut flags as *mut u32).cast(),
            size_of::<u32>(),
        )
    });
    if result != Outcome::Applied {
        result
    } else if flags & required != required {
        Outcome::Failed(ERROR_INVALID_DATA)
    } else {
        Outcome::Applied
    }
}

fn dacl() -> Outcome {
    // Deny new VM read/write/operation, thread creation, handle duplication and
    // ACL/owner changes. OWNER RIGHTS overrides implicit owner WRITE_DAC.
    // Keep query-limited, READ_CONTROL and SYNCHRONIZE for ordinary monitoring.
    let sddl: Vec<u16> = "D:P(D;;0x000C007A;;;WD)(A;;RC;;;OW)(A;;0x00121000;;;WD)"
        .encode_utf16()
        .chain(Some(0))
        .collect();
    let mut descriptor: PSECURITY_DESCRIPTOR = ptr::null_mut();
    // SAFETY: terminated UTF-16 input; API allocates a descriptor on success.
    let result = win32(unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            1,
            &mut descriptor,
            ptr::null_mut(),
        )
    });
    if result != Outcome::Applied {
        return result;
    }
    // SAFETY: live API-allocated descriptor and current process pseudo handle.
    let result = win32(unsafe {
        SetKernelObjectSecurity(GetCurrentProcess(), DACL_SECURITY_INFORMATION, descriptor)
    });
    // SAFETY: descriptor belongs to LocalAlloc, released exactly once.
    let free_failed = unsafe { !LocalFree(descriptor).is_null() };
    if free_failed && result == Outcome::Applied {
        Outcome::Failed(unsafe { GetLastError() })
    } else {
        result
    }
}

pub(super) fn harden() -> HardeningReport {
    HardeningReport {
        measures: vec![
            (Measure::WerHeapAndSnapshots, wer()),
            (
                Measure::ExtensionPoints,
                mitigation(ProcessExtensionPointDisablePolicy, 1),
            ),
            (
                Measure::ImageLoads,
                mitigation(ProcessImageLoadPolicy, 0b111),
            ),
            (Measure::ProcessDacl, dacl()),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win32_failure_keeps_error_code() {
        unsafe {
            windows_sys::Win32::Foundation::SetLastError(5);
        }
        assert_eq!(win32(0), Outcome::Failed(5));
        assert_eq!(win32(1), Outcome::Applied);
    }
}
