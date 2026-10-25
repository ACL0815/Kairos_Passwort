#![cfg(windows)]

use kairos_hardening::{harden_process, Measure, Outcome};
use std::{mem::size_of, ptr};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, GetLastError, ERROR_ACCESS_DENIED, ERROR_NOT_ALL_ASSIGNED, ERROR_SUCCESS,
        HANDLE, LUID,
    },
    Security::{
        AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, PrivilegeCheck,
        TokenElevation, LUID_AND_ATTRIBUTES, PRIVILEGE_SET, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED,
        TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY,
    },
    System::Threading::{
        GetCurrentProcess, GetCurrentProcessId, OpenProcess, OpenProcessToken,
        PROCESS_CREATE_THREAD, PROCESS_DUP_HANDLE, PROCESS_QUERY_INFORMATION,
        PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SUSPEND_RESUME, PROCESS_TERMINATE,
        PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

struct TestToken {
    handle: HANDLE,
    debug: LUID,
}

impl TestToken {
    fn open() -> Self {
        let mut handle = ptr::null_mut();
        let mut debug = LUID::default();
        // SAFETY: current process token and valid output pointers, before DACL hardening.
        unsafe {
            assert_ne!(
                OpenProcessToken(
                    GetCurrentProcess(),
                    TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,
                    &mut handle
                ),
                0
            );
            assert_ne!(
                LookupPrivilegeValueW(ptr::null(), SE_DEBUG_NAME, &mut debug),
                0
            );
        }
        Self { handle, debug }
    }

    fn elevated(&self) -> bool {
        let mut elevation = TOKEN_ELEVATION::default();
        let mut len = 0;
        // SAFETY: live query-capable token and correctly sized output structure.
        assert_ne!(
            unsafe {
                GetTokenInformation(
                    self.handle,
                    TokenElevation,
                    (&mut elevation as *mut TOKEN_ELEVATION).cast(),
                    size_of::<TOKEN_ELEVATION>() as u32,
                    &mut len,
                )
            },
            0
        );
        elevation.TokenIsElevated != 0
    }

    fn debug_enabled(&self) -> bool {
        let mut required = PRIVILEGE_SET {
            PrivilegeCount: 1,
            Control: 0,
            Privilege: [LUID_AND_ATTRIBUTES {
                Luid: self.debug,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };
        let mut enabled = 0;
        // SAFETY: live token, one-entry privilege set and valid BOOL output.
        assert_ne!(
            unsafe { PrivilegeCheck(self.handle, &mut required, &mut enabled) },
            0
        );
        enabled != 0
    }

    fn set_debug(&self, enabled: bool) -> bool {
        let privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: self.debug,
                Attributes: if enabled { SE_PRIVILEGE_ENABLED } else { 0 },
            }],
        };
        // SAFETY: only this disposable child's token changes; no previous-state buffer.
        let success = unsafe {
            AdjustTokenPrivileges(
                self.handle,
                0,
                &privileges,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        let error = unsafe { GetLastError() };
        assert_ne!(success, 0, "AdjustTokenPrivileges: {error}");
        assert!(
            error == ERROR_SUCCESS || error == ERROR_NOT_ALL_ASSIGNED,
            "AdjustTokenPrivileges: {error}"
        );
        assert_eq!(self.debug_enabled(), enabled && error == ERROR_SUCCESS);
        error == ERROR_SUCCESS
    }
}

impl Drop for TestToken {
    fn drop(&mut self) {
        assert_ne!(unsafe { CloseHandle(self.handle) }, 0);
    }
}

fn check_handles(debug_enabled: bool) {
    for access in [
        PROCESS_CREATE_THREAD,
        PROCESS_VM_OPERATION,
        PROCESS_VM_READ,
        PROCESS_VM_WRITE,
        PROCESS_DUP_HANDLE,
        0x00040000, // WRITE_DAC
        0x00080000, // WRITE_OWNER
        PROCESS_QUERY_INFORMATION,
        PROCESS_SUSPEND_RESUME,
        PROCESS_TERMINATE,
    ] {
        // SAFETY: a fresh real handle exercises the DACL or documented debug bypass.
        let handle = unsafe { OpenProcess(access, 0, GetCurrentProcessId()) };
        let error = unsafe { GetLastError() };
        if !handle.is_null() {
            assert_ne!(unsafe { CloseHandle(handle) }, 0);
        }
        assert_eq!(
            !handle.is_null(),
            debug_enabled,
            "access={access:#x}, SeDebugPrivilege enabled={debug_enabled}, error={error}"
        );
        if !debug_enabled {
            assert_eq!(error, ERROR_ACCESS_DENIED);
        }
    }
    let handle =
        unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, GetCurrentProcessId()) };
    assert!(!handle.is_null());
    assert_ne!(unsafe { CloseHandle(handle) }, 0);
}

#[test]
fn hardening_in_isolated_child() {
    const CHILD: &str = "KAIROS_HARDENING_TEST_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let token = TestToken::open();
        let initially_enabled = token.debug_enabled();
        eprintln!(
            "process token: elevated={}, SeDebugPrivilege initially_enabled={initially_enabled}",
            token.elevated()
        );
        let report = harden_process();
        assert!(report.all_applied(), "{report:?}");
        for measure in [
            Measure::WerHeapAndSnapshots,
            Measure::ExtensionPoints,
            Measure::ImageLoads,
            Measure::ProcessDacl,
        ] {
            assert!(report.measures.contains(&(measure, Outcome::Applied)));
        }
        check_handles(initially_enabled);
        let available = token.set_debug(true);
        eprintln!("SeDebugPrivilege available={available}");
        if available {
            check_handles(true);
            eprintln!("SeDebugPrivilege enabled: all 10 dangerous handles bypass the DACL");
        }
        token.set_debug(false);
        check_handles(false);
        eprintln!("SeDebugPrivilege disabled/absent: all 10 dangerous handles denied, query-limited allowed");
    } else {
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "hardening_in_isolated_child", "--nocapture"])
            .env(CHILD, "1")
            .status()
            .unwrap();
        assert!(result.success(), "isolated hardening child: {result}");
    }
}
