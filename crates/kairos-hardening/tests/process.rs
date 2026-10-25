#![cfg(windows)]

use kairos_hardening::{harden_process, Measure, Outcome};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, ERROR_ACCESS_DENIED},
    System::Threading::{
        GetCurrentProcessId, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_DUP_HANDLE,
        PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SUSPEND_RESUME,
        PROCESS_TERMINATE, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

#[test]
fn hardening_in_isolated_child() {
    const CHILD: &str = "KAIROS_HARDENING_TEST_CHILD";
    if std::env::var_os(CHILD).is_some() {
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
            // SAFETY: opening a fresh handle checks the same-user DACL.
            let handle = unsafe { OpenProcess(access, 0, GetCurrentProcessId()) };
            if !handle.is_null() {
                unsafe {
                    CloseHandle(handle);
                }
            }
            assert!(
                handle.is_null(),
                "dangerous new handle allowed: {access:#x}"
            );
            assert_eq!(unsafe { GetLastError() }, ERROR_ACCESS_DENIED);
        }
        let handle =
            unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, GetCurrentProcessId()) };
        assert!(!handle.is_null());
        assert_ne!(unsafe { CloseHandle(handle) }, 0);
    } else {
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "hardening_in_isolated_child", "--nocapture"])
            .env(CHILD, "1")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
