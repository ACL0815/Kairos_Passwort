//! Modelled after: Bitwarden SDK memory-testing, with an independent test harness.
//! <https://github.com/bitwarden/sdk-internal/blob/5cf919837e83b71c2508bf99344ea57cf5f84e88/crates/memory-testing/src/lib.rs>.
#![cfg(windows)]

use kairos_hardening::LockedSecret;
use rand::TryRngCore;
use std::{fs::File, io::Read, os::windows::io::AsRawHandle, path::Path, ptr};
use windows_sys::Win32::System::{
    Diagnostics::Debug::{MiniDumpWithFullMemory, MiniDumpWriteDump},
    Threading::{GetCurrentProcess, GetCurrentProcessId},
};

// Deterministic synthetic marker, never a production password generator. The
// seed crosses the process boundary, the complete marker never does.
fn next_byte(state: &mut u64) -> u8 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    b'!' + (*state % 90) as u8
}

fn dump(path: &Path) {
    let file = File::create(path).unwrap();
    // SAFETY: own live process and writable owned file; no exception pointers.
    // Single-threaded child avoids concurrent DbgHelp calls. Self-dumping is a
    // diagnostic compromise; Microsoft's production guidance prefers a helper.
    let result = unsafe {
        MiniDumpWriteDump(
            GetCurrentProcess(),
            GetCurrentProcessId(),
            file.as_raw_handle(),
            MiniDumpWithFullMemory,
            ptr::null(),
            ptr::null(),
            ptr::null(),
        )
    };
    assert_ne!(result, 0, "{}", std::io::Error::last_os_error());
    file.sync_all().unwrap();
}

fn contains(path: &Path, marker: &[u8]) -> bool {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; 1024 * 1024 + marker.len()];
    let mut carry = 0;
    loop {
        let read = file.read(&mut buffer[carry..]).unwrap();
        let end = carry + read;
        if buffer[..end].windows(marker.len()).any(|w| w == marker) {
            return true;
        }
        if read == 0 {
            return false;
        }
        carry = (marker.len() - 1).min(end);
        buffer.copy_within(end - carry..end, 0);
    }
}

#[test]
#[ignore = "writes three full synthetic-process dumps; run explicitly in release"]
#[allow(
    clippy::assertions_on_constants,
    reason = "manual diagnostic must reject debug runs at runtime"
)]
fn released_secret_dump_has_positive_control() {
    assert!(
        !cfg!(debug_assertions),
        "run this diagnostic with cargo test --release"
    );
    if let Some(root) = std::env::var_os("KAIROS_DUMP_CHILD") {
        let root = std::path::PathBuf::from(root);
        let mut state: u64 = std::env::var("KAIROS_DUMP_SEED").unwrap().parse().unwrap();
        let mut utf8 = LockedSecret::new(64).unwrap();
        let mut utf16 = LockedSecret::new(128).unwrap();
        let mut control8 = LockedSecret::new(64).unwrap();
        let mut control16 = LockedSecret::new(128).unwrap();
        let mut control_state = state ^ 0x9e3779b97f4a7c15;
        for _ in 0..64 {
            let byte = next_byte(&mut state);
            utf8.extend_from_slice(&[byte]).unwrap();
            utf16.extend_from_slice(&[byte, 0]).unwrap();
            let control = next_byte(&mut control_state);
            control8.extend_from_slice(&[control]).unwrap();
            control16.extend_from_slice(&[control, 0]).unwrap();
        }
        dump(&root.join("positive.dmp"));
        utf8.clear();
        utf16.clear();
        // Both allocations must remain committed in this dump. Testing only
        // after close() would miss a broken scrub because VirtualFree unmaps.
        dump(&root.join("cleared.dmp"));
        utf8.close().unwrap();
        utf16.close().unwrap();
        dump(&root.join("released.dmp"));
        control8.close().unwrap();
        control16.close().unwrap();
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let seed = rand::rngs::OsRng.try_next_u64().unwrap().max(1);
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "released_secret_dump_has_positive_control",
            "--ignored",
            "--test-threads=1",
        ])
        .env("KAIROS_DUMP_CHILD", dir.path())
        .env("KAIROS_DUMP_SEED", seed.to_string())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    // Construct search strings only in the parent, after the dumped child exits.
    let mut state = seed;
    let utf8: Vec<u8> = (0..64).map(|_| next_byte(&mut state)).collect();
    let utf16: Vec<u8> = utf8.iter().flat_map(|b| [*b, 0]).collect();
    let mut control_state = seed ^ 0x9e3779b97f4a7c15;
    let control8: Vec<u8> = (0..64).map(|_| next_byte(&mut control_state)).collect();
    let control16: Vec<u8> = control8.iter().flat_map(|b| [*b, 0]).collect();
    for control in [control8, control16] {
        for phase in ["cleared", "released"] {
            assert!(
                contains(&dir.path().join(format!("{phase}.dmp")), &control),
                "missing live positive control after {phase}"
            );
        }
    }
    for (encoding, marker) in [("UTF-8", utf8), ("UTF-16LE", utf16)] {
        assert!(
            !contains(&dir.path().join("cleared.dmp"), &marker),
            "cleared {encoding} marker remains in committed memory"
        );
        assert!(
            contains(&dir.path().join("positive.dmp"), &marker),
            "missing {encoding} positive control"
        );
        assert!(
            !contains(&dir.path().join("released.dmp"), &marker),
            "released {encoding} marker remains"
        );
    }
    // TempDir also removes dumps on assertion unwinding. Explicit close checks
    // deletion on success; these files contain only this isolated test process.
    dir.close().unwrap();
}

#[test]
fn scanner_finds_patterns_across_read_boundaries() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("scan.bin");
    let mut bytes = vec![0; 1024 * 1024 + 5];
    let marker = b"runtime-marker";
    bytes.extend_from_slice(marker);
    std::fs::write(&path, bytes).unwrap();
    assert!(contains(&path, marker));
    assert!(!contains(&path, b"absent-marker"));
}
