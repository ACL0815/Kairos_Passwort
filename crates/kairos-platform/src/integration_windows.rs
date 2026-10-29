use super::*;
use ::windows::{
    core::{Interface, PCWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
            COINIT_APARTMENTTHREADED, STGM_READ,
        },
        UI::Shell::{IShellLinkW, ShellLink, SLGP_RAWPATH},
    },
};
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr};
use windows_sys::Win32::{
    Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS},
    System::Registry::*,
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APPROVAL_KEY: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
const DESCRIPTION: &str = "Kairos Passwort – Wortpasswort-Generator";
fn wide(text: impl AsRef<OsStr>) -> Vec<u16> {
    text.as_ref().encode_wide().chain(Some(0)).collect()
}
fn status(code: u32) -> io::Result<()> {
    if code == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(code as i32))
    }
}
struct Key(HKEY);
impl Drop for Key {
    fn drop(&mut self) {
        unsafe {
            RegCloseKey(self.0);
        }
    }
}
fn open(path: &str, write: bool) -> io::Result<Key> {
    let mut key = ptr::null_mut();
    let path = wide(path);
    let result = unsafe {
        if write {
            RegCreateKeyExW(
                HKEY_CURRENT_USER,
                path.as_ptr(),
                0,
                ptr::null(),
                0,
                KEY_SET_VALUE | KEY_QUERY_VALUE,
                ptr::null(),
                &mut key,
                ptr::null_mut(),
            )
        } else {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                path.as_ptr(),
                0,
                KEY_QUERY_VALUE,
                &mut key,
            )
        }
    };
    status(result)?;
    Ok(Key(key))
}
fn read_value(path: &str, name: &str) -> io::Result<Option<(u32, Vec<u8>)>> {
    let key = match open(path, false) {
        Ok(k) => k,
        Err(e) if e.raw_os_error() == Some(ERROR_FILE_NOT_FOUND as i32) => return Ok(None),
        Err(e) => return Err(e),
    };
    let name = wide(name);
    let (mut kind, mut len) = (0, 0);
    let code = unsafe {
        RegQueryValueExW(
            key.0,
            name.as_ptr(),
            ptr::null(),
            &mut kind,
            ptr::null_mut(),
            &mut len,
        )
    };
    if code == ERROR_FILE_NOT_FOUND {
        return Ok(None);
    }
    status(code)?;
    if len > 65536 {
        return Err(io::Error::other("registry value too large"));
    }
    let mut bytes = vec![0; len as usize];
    status(unsafe {
        RegQueryValueExW(
            key.0,
            name.as_ptr(),
            ptr::null(),
            &mut kind,
            bytes.as_mut_ptr(),
            &mut len,
        )
    })?;
    bytes.truncate(len as usize);
    Ok(Some((kind, bytes)))
}
pub fn read_run(name: &str, exe: &Path) -> io::Result<RunState> {
    let command = read_value(RUN_KEY, name)?
        .map(|(kind, bytes)| {
            if kind != REG_SZ || bytes.len() % 2 != 0 {
                return Err(io::Error::other("Run value must be REG_SZ"));
            }
            let mut units: Vec<_> = bytes
                .as_chunks::<2>()
                .0
                .iter()
                .map(|&b| u16::from_le_bytes(b))
                .collect();
            if units.last() == Some(&0) {
                units.pop();
            }
            String::from_utf16(&units).map_err(io::Error::other)
        })
        .transpose()?;
    let approval = read_value(APPROVAL_KEY, name)?;
    let approval = match approval.as_ref() {
        Some((REG_BINARY, bytes)) => startup_approval(Some(bytes)),
        None => StartupApproval::NotSet,
        _ => StartupApproval::Unknown,
    };
    Ok(RunState {
        path_matches: command.as_ref().is_some_and(|c| run_path_matches(c, exe)),
        command_matches: command
            .as_ref()
            .is_some_and(|c| run_command_matches(c, exe)),
        command,
        approval,
    })
}
pub fn set_run(name: &str, exe: &Path) -> io::Result<()> {
    if matches!(
        read_run(name, exe)?.approval,
        StartupApproval::Disabled | StartupApproval::Unknown
    ) {
        return Err(io::Error::other(
            "autostart approval must be changed in Windows Task Manager",
        ));
    }
    let command = wide(run_command(exe)?);
    let key = open(RUN_KEY, true)?;
    status(unsafe {
        RegSetValueExW(
            key.0,
            wide(name).as_ptr(),
            0,
            REG_SZ,
            command.as_ptr().cast(),
            (command.len() * 2) as u32,
        )
    })
}
pub fn remove_run(name: &str) -> io::Result<()> {
    // Opening an existing key avoids creating registry keys during cleanup.
    let mut raw = ptr::null_mut();
    let code = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            wide(RUN_KEY).as_ptr(),
            0,
            KEY_SET_VALUE,
            &mut raw,
        )
    };
    if code == ERROR_FILE_NOT_FOUND {
        return Ok(());
    }
    status(code)?;
    let key = Key(raw);
    let code = unsafe { RegDeleteValueW(key.0, wide(name).as_ptr()) };
    if code == ERROR_FILE_NOT_FOUND {
        Ok(())
    } else {
        status(code)
    }
}

/// COM objects and apartment guard stay on the calling thread and outlive each call.
struct Apartment;
impl Apartment {
    fn enter() -> io::Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .map_err(io::Error::other)?;
        }
        Ok(Self)
    }
}
impl Drop for Apartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
pub struct Shortcut;
impl Shortcut {
    pub fn create(path: &Path, exe: &Path) -> io::Result<()> {
        if path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "shortcut already exists",
            ));
        }
        let _apartment = Apartment::enter()?;
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::other("no shortcut parent"))?;
        std::fs::create_dir_all(parent)?;
        let staged = tempfile::Builder::new()
            .prefix(".Kairos-")
            .suffix(".lnk")
            .tempfile_in(parent)?
            .into_temp_path();
        unsafe {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                .map_err(io::Error::other)?;
            link.SetPath(PCWSTR(wide(exe).as_ptr()))
                .map_err(io::Error::other)?;
            link.SetWorkingDirectory(PCWSTR(
                wide(
                    exe.parent()
                        .ok_or_else(|| io::Error::other("no exe parent"))?,
                )
                .as_ptr(),
            ))
            .map_err(io::Error::other)?;
            link.SetDescription(PCWSTR(wide(DESCRIPTION).as_ptr()))
                .map_err(io::Error::other)?;
            link.SetIconLocation(PCWSTR(wide(exe).as_ptr()), 0)
                .map_err(io::Error::other)?;
            let persist: IPersistFile = link.cast().map_err(io::Error::other)?;
            persist
                .Save(PCWSTR(wide(&staged).as_ptr()), true)
                .map_err(io::Error::other)?;
        }
        staged
            .persist_noclobber(path)
            .map_err(|error| error.error)?;
        Ok(())
    }
    /// Load without resolving or invoking the target; never launches anything.
    pub fn target(path: &Path) -> io::Result<PathBuf> {
        let _apartment = Apartment::enter()?;
        unsafe {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                .map_err(io::Error::other)?;
            let persist: IPersistFile = link.cast().map_err(io::Error::other)?;
            persist
                .Load(PCWSTR(wide(path).as_ptr()), STGM_READ)
                .map_err(io::Error::other)?;
            let mut path = vec![0; 32768];
            link.GetPath(&mut path, ptr::null_mut(), SLGP_RAWPATH.0 as u32)
                .map_err(io::Error::other)?;
            let len = path.iter().position(|&c| c == 0).unwrap_or(path.len());
            Ok(PathBuf::from(std::ffi::OsString::from_wide(&path[..len])))
        }
    }
    pub fn remove(path: &Path) -> io::Result<()> {
        if path.exists() {
            let _apartment = Apartment::enter()?;
            unsafe {
                let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                    .map_err(io::Error::other)?;
                let persist: IPersistFile = link.cast().map_err(io::Error::other)?;
                persist
                    .Load(PCWSTR(wide(path).as_ptr()), STGM_READ)
                    .map_err(io::Error::other)?;
                let mut description = [0; 256];
                link.GetDescription(&mut description)
                    .map_err(io::Error::other)?;
                let len = description
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(description.len());
                if String::from_utf16_lossy(&description[..len]) != DESCRIPTION {
                    return Err(io::Error::other(
                        "existing shortcut does not belong to Kairos",
                    ));
                }
            }
        }
        match std::fs::remove_file(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            result => result,
        }
    }
}
use std::os::windows::ffi::OsStringExt;
