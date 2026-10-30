//! Explicit, per-user Windows integration. No writes occur while reading status.
//!
//! Modelled after KeePassXC's optional desktop integration and Microsoft's Run
//! and Shell Link contracts; see docs/sources/platform.md. No application code copied.

use std::{
    io,
    path::{Path, PathBuf},
};

pub const RUN_VALUE: &str = "Kairos Passwort";

/// Quote the executable separately from the fixed argument, including Unicode paths.
pub fn run_command(exe: &Path) -> io::Result<String> {
    let path = exe
        .to_str()
        .ok_or_else(|| io::Error::other("executable path is not Unicode"))?;
    if path.is_empty() || path.contains(['"', '\0', '\r', '\n']) {
        return Err(io::Error::other("invalid executable path"));
    }
    Ok(format!("\"{path}\" --tray"))
}

pub fn run_path_matches(command: &str, exe: &Path) -> bool {
    let command = command.trim();
    let path = if let Some(quoted) = command.strip_prefix('"') {
        quoted.split_once('"').map(|(path, _)| path)
    } else {
        command.split_whitespace().next()
    };
    path.zip(exe.to_str())
        .is_some_and(|(a, b)| a.to_lowercase() == b.to_lowercase())
}

pub fn run_command_matches(command: &str, exe: &Path) -> bool {
    let Some((_, arguments)) = command
        .trim()
        .strip_prefix('"')
        .and_then(|c| c.split_once('"'))
    else {
        return false;
    };
    run_path_matches(command, exe) && arguments.trim() == "--tray"
}

/// Append only Kairos's fixed filename to the resolved Programs directory.
/// This pure function neither resolves Windows folders nor accesses the filesystem.
pub fn start_menu_path(programs: &Path) -> io::Result<PathBuf> {
    if programs.as_os_str().is_empty() || !programs.is_absolute() {
        return Err(io::Error::other("Programs must be an absolute directory"));
    }
    Ok(programs.join("Kairos Passwort.lnk"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupApproval {
    NotSet,
    Enabled,
    Disabled,
    Unknown,
}

/// StartupApproved is an undocumented Windows detail: interpret known states
/// conservatively and never write this key, including when removing integration.
pub fn startup_approval(bytes: Option<&[u8]>) -> StartupApproval {
    match bytes {
        None => StartupApproval::NotSet,
        Some(bytes) if bytes.len() == 12 => match bytes[0] {
            2 | 6 => StartupApproval::Enabled,
            3 | 7 => StartupApproval::Disabled,
            _ => StartupApproval::Unknown,
        },
        _ => StartupApproval::Unknown,
    }
}

#[derive(Debug)]
pub struct RunState {
    pub command: Option<String>,
    pub path_matches: bool,
    pub command_matches: bool,
    pub approval: StartupApproval,
}

#[cfg(windows)]
#[path = "integration_windows.rs"]
mod windows;
#[cfg(windows)]
pub use windows::{programs_dir, read_run, remove_run, set_run, Shortcut};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quotes_spaces_and_umlauts_without_quoting_the_argument() {
        let exe = Path::new(r"C:\Programme\Jörg Müller\Kairos Passwort.exe");
        assert_eq!(
            run_command(exe).unwrap(),
            r#""C:\Programme\Jörg Müller\Kairos Passwort.exe" --tray"#
        );
        for invalid in ["", "bad\"path", "bad\0path", "bad\npath"] {
            assert!(run_command(Path::new(invalid)).is_err());
        }
    }
    #[test]
    fn detects_a_moved_executable_including_unicode_and_case() {
        let exe = Path::new(r"C:\Jörg Müller\Kairos.exe");
        assert!(run_path_matches(
            r#""c:\jörg müller\KAIROS.EXE" --tray"#,
            exe
        ));
        assert!(!run_path_matches(
            r#""D:\Jörg Müller\Kairos.exe" --tray"#,
            exe
        ));
        assert!(!run_path_matches(
            r#""C:\Jörg Müller\Kairos.exe.bak" --tray"#,
            exe
        ));
        assert!(!run_path_matches("", exe));
        assert!(run_command_matches(
            r#""c:\jörg müller\KAIROS.EXE" --tray"#,
            exe
        ));
        assert!(!run_command_matches(
            r#""c:\jörg müller\KAIROS.EXE" --TRAY"#,
            exe
        ));
        assert!(!run_command_matches(
            r#""c:\jörg müller\KAIROS.EXE" --popup"#,
            exe
        ));
    }
    #[test]
    fn resolves_only_the_own_start_menu_link() {
        let root = std::env::temp_dir().join("KairosTest-Ä/Redirected Programs");
        assert_eq!(
            start_menu_path(&root).unwrap(),
            root.join("Kairos Passwort.lnk")
        );
        assert!(start_menu_path(Path::new("")).is_err());
        assert!(start_menu_path(Path::new("relative")).is_err());
    }
    #[test]
    fn startup_approval_recognizes_only_known_full_records() {
        assert_eq!(startup_approval(None), StartupApproval::NotSet);
        for (state, expected) in [
            (2, StartupApproval::Enabled),
            (6, StartupApproval::Enabled),
            (3, StartupApproval::Disabled),
            (7, StartupApproval::Disabled),
            (9, StartupApproval::Unknown),
        ] {
            let mut bytes = [0; 12];
            bytes[0] = state;
            assert_eq!(startup_approval(Some(&bytes)), expected);
        }
        assert_eq!(startup_approval(Some(&[3])), StartupApproval::Unknown);
    }
}
