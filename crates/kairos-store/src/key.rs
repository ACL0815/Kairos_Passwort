//! CurrentUser DPAPI wrapping with no plaintext key file.
//!
//! Modelled after: Chromium os_crypt
//! <https://github.com/chromium/chromium/blob/130.0.6723.58/components/os_crypt/sync/os_crypt_win.cc>.
//! Modelled after: libsodium
//! <https://github.com/jedisct1/libsodium/blob/60b751cd14418fae2d17e6f1003e43622b3e5a25/src/libsodium/sodium/utils.c>.

use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroizing;

use crate::StoreError;

/// A 256-bit database secret. Intentionally neither `Debug` nor `Clone`.
pub struct DatabaseKey(pub(crate) Zeroizing<[u8; 32]>);

impl DatabaseKey {
    pub fn generate() -> Result<Self, StoreError> {
        let mut bytes = Zeroizing::new([0; 32]);
        OsRng
            .try_fill_bytes(bytes.as_mut())
            .map_err(|_| StoreError::RandomSourceUnavailable)?;
        Ok(Self(bytes))
    }

    /// Accepts an externally managed key; ownership includes zeroizing on drop.
    pub fn from_bytes(bytes: Zeroizing<[u8; 32]>) -> Self {
        Self(bytes)
    }
}

#[cfg(windows)]
pub(crate) fn load_or_create(paths: &crate::StorePaths) -> Result<DatabaseKey, StoreError> {
    use std::io::Write;

    let protected = match std::fs::read(&paths.key_file) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if paths.database.try_exists()? {
                // Another initializer may have published the key and created
                // the database since our first read. Re-read before failing.
                std::fs::read(&paths.key_file).map_err(|error| {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        StoreError::MissingKey
                    } else {
                        StoreError::Io(error)
                    }
                })?
            } else {
                let parent = paths
                    .key_file
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .ok_or(StoreError::InvalidPaths)?;
                std::fs::create_dir_all(parent)?;
                let key = DatabaseKey::generate()?;
                let protected = dpapi(key.0.as_ref(), true)?;
                let mut pending = tempfile::NamedTempFile::new_in(parent)?;
                pending.write_all(&protected)?;
                pending.as_file().sync_all()?;
                match pending.persist_noclobber(&paths.key_file) {
                    Ok(_) => return Ok(key),
                    Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                        std::fs::read(&paths.key_file)?
                    }
                    Err(error) => return Err(StoreError::Io(error.error)),
                }
            }
        }
        Err(error) => return Err(error.into()),
    };
    let plaintext = dpapi(&protected, false)?;
    if plaintext.len() != 32 {
        return Err(StoreError::KeyProtection);
    }
    let mut key = Zeroizing::new([0; 32]);
    key.copy_from_slice(&plaintext);
    Ok(DatabaseKey(key))
}

#[cfg(not(windows))]
pub(crate) fn load_or_create(_: &crate::StorePaths) -> Result<DatabaseKey, StoreError> {
    Err(StoreError::UnsupportedPlatform)
}

#[cfg(windows)]
fn dpapi(input: &[u8], protect: bool) -> Result<Zeroizing<Vec<u8>>, StoreError> {
    use std::{ptr, slice};
    use windows_sys::Win32::{
        Foundation::LocalFree,
        Security::Cryptography::{
            CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
        },
    };
    use zeroize::Zeroize;

    let input = CRYPT_INTEGER_BLOB {
        cbData: input
            .len()
            .try_into()
            .map_err(|_| StoreError::KeyProtection)?,
        pbData: input.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };
    // SAFETY: DPAPI borrows the live input slice for this call, allocates output
    // with LocalAlloc and never mutates input. No LOCAL_MACHINE flag is passed.
    let success = unsafe {
        if protect {
            CryptProtectData(
                &input,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        } else {
            CryptUnprotectData(
                &input,
                ptr::null_mut(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        }
    };
    if success == 0 {
        return Err(StoreError::KeyProtection);
    }
    if output.pbData.is_null() {
        return if output.cbData == 0 {
            Ok(Zeroizing::new(Vec::new()))
        } else {
            Err(StoreError::KeyProtection)
        };
    }
    // SAFETY: Successful DPAPI calls return cbData initialized bytes owned by
    // LocalAlloc. Copy into a zeroizing owner, scrub native bytes, then free once.
    let result = unsafe {
        let bytes = slice::from_raw_parts_mut(output.pbData, output.cbData as usize);
        let result = Zeroizing::new(bytes.to_vec());
        bytes.zeroize();
        LocalFree(output.pbData.cast());
        result
    };
    Ok(result)
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn dpapi_roundtrip_wraps_all_key_bytes() {
        let key = DatabaseKey::generate().unwrap();
        let protected = dpapi(key.0.as_ref(), true).unwrap();
        assert!(!protected
            .windows(32)
            .any(|window| window == key.0.as_slice()));
        assert!(dpapi(&protected, false).unwrap().as_slice() == key.0.as_slice());
    }

    #[test]
    fn authenticated_blob_with_wrong_key_length_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let paths = crate::StorePaths {
            database: dir.path().join("words.db"),
            key_file: dir.path().join("key.dpapi"),
        };
        let protected = dpapi(b"short", true).unwrap();
        std::fs::write(&paths.key_file, &protected).unwrap();
        assert!(matches!(
            load_or_create(&paths),
            Err(StoreError::KeyProtection)
        ));
        assert_eq!(
            std::fs::read(&paths.key_file).unwrap(),
            protected.as_slice()
        );
        assert!(!paths.database.exists());
    }
}
