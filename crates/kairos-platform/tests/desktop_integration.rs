#![cfg(windows)]
//! Real HKCU and COM calls, restricted to a unique test value and TempDir.
use kairos_platform::integration::{read_run, remove_run, run_command, set_run, Shortcut};

struct RunCleanup(String);
impl Drop for RunCleanup {
    fn drop(&mut self) {
        let _ = remove_run(&self.0);
    }
}

#[test]
fn own_registry_value_roundtrips_detects_move_and_is_removed() {
    let name = format!("KairosTest-{}-run", std::process::id());
    let cleanup = RunCleanup(name.clone());
    let root = tempfile::tempdir().unwrap();
    let exe = root.path().join("Jörg Müller/Kairos Passwort.exe");
    assert!(read_run(&name, &exe).unwrap().command.is_none());
    set_run(&name, &exe).unwrap();
    let state = read_run(&name, &exe).unwrap();
    assert_eq!(
        state.command.as_deref(),
        Some(run_command(&exe).unwrap().as_str())
    );
    assert!(state.path_matches && state.command_matches);
    let moved = root.path().join("another/Kairos.exe");
    let state = read_run(&name, &moved).unwrap();
    assert!(!state.path_matches && !state.command_matches);
    remove_run(&name).unwrap();
    assert!(read_run(&name, &exe).unwrap().command.is_none());
    drop(cleanup);
    println!("CLEANED: HKCU Run value {name}; actual Kairos value never written");
}

#[test]
fn shell_link_is_created_in_temp_only_and_foreign_files_survive() {
    let root = tempfile::Builder::new()
        .prefix("KairosTest-")
        .tempdir()
        .unwrap();
    let root_path = root.path().to_owned();
    let exe = root.path().join("Kairos Ä Passwort.exe");
    std::fs::write(&exe, []).unwrap();
    let link = root.path().join("KairosTest.lnk");
    Shortcut::create(&link, &exe).unwrap();
    assert!(link.is_file());
    assert_eq!(Shortcut::target(&link).unwrap(), exe);
    assert!(Shortcut::create(&link, &exe).is_err());
    Shortcut::remove(&link).unwrap();
    assert!(!link.exists());
    Shortcut::remove(&link).unwrap();
    std::fs::write(&link, b"foreign file").unwrap();
    assert!(Shortcut::remove(&link).is_err());
    assert_eq!(std::fs::read(&link).unwrap(), b"foreign file");
    drop(root);
    assert!(!root_path.exists());
    println!(
        "CLEANED: {} (test link and test EXE); actual Start menu never written",
        root_path.display()
    );
}
