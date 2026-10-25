//! Pluggable association persistence, with an insert-only CurrentUser DPAPI file.
//!
//! Modelled after: Chromium os_crypt's CurrentUser protection boundary
//! <https://github.com/chromium/chromium/blob/130.0.6723.58/components/os_crypt/sync/os_crypt_win.cc>.
//! The Kairos store README informed the atomic-publication pattern; this module
//! has no dependency on kairos-store and contains an independent adapter.

use crate::{protocol::encode_json, Error};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

const MAX_STORED_BYTES: usize = 64 * 1024;

/// The protocol's identification PUBLIC key is a bearer secret in practice.
/// No `Debug` or `Clone`; all owned credential buffers zeroize on drop.
#[derive(Serialize, Deserialize)]
pub struct Association {
    version: u8,
    database_hash: String,
    id: Zeroizing<String>,
    id_key: Box<Zeroizing<[u8; 32]>>,
}

impl Association {
    pub fn new(
        database_hash: String,
        id: Zeroizing<String>,
        id_key: Zeroizing<[u8; 32]>,
    ) -> Result<Self, Error> {
        let result = Self {
            version: 1,
            database_hash,
            id,
            id_key: Box::new(id_key),
        };
        result.validate()?;
        Ok(result)
    }

    pub fn database_hash(&self) -> &str {
        &self.database_hash
    }

    /// Sensitive identifier; never log it.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sensitive bearer key, although it originated as the public half of a pair.
    pub fn id_key(&self) -> &[u8; 32] {
        &self.id_key
    }

    /// For custom stores. The returned plaintext MUST be protected before writing.
    pub fn to_bytes(&self) -> Result<Zeroizing<Vec<u8>>, Error> {
        self.validate()?;
        let bytes = encode_json(self)?;
        if bytes.len() > MAX_STORED_BYTES {
            return Err(Error::Storage);
        }
        Ok(bytes)
    }

    /// For custom stores, after authenticating/decrypting their stored data.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > MAX_STORED_BYTES {
            return Err(Error::Storage);
        }
        let record: Self = serde_json::from_slice(bytes).map_err(|_| Error::Storage)?;
        record.validate()?;
        Ok(record)
    }

    fn validate(&self) -> Result<(), Error> {
        if self.version != 1
            || self.database_hash.is_empty()
            || self.database_hash.len() > 4096
            || self.id.is_empty()
            || self.id.len() > 4096
            || self.id_key.iter().all(|b| *b == 0)
        {
            return Err(Error::Storage);
        }
        Ok(())
    }
}

/// One association per store. Load errors are fatal; never substitute a fresh
/// identity for corrupt data. `save` must atomically insert without overwriting.
/// Applications can supply separate stores for deliberately paired databases.
pub trait AssociationStore {
    fn load(&mut self) -> Result<Option<Association>, Error>;
    fn save(&mut self, association: &Association) -> Result<(), Error>;
}

/// `%APPDATA%\Kairos\keepassxc-association.dpapi`, or an explicit isolated path.
/// No plaintext fallback on other platforms. Parent directory ACLs are inherited.
/// DPAPI protects against other accounts, not code running as the same user.
pub struct DpapiStore {
    path: PathBuf,
}

impl DpapiStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn for_current_user() -> Result<Self, Error> {
        #[cfg(windows)]
        {
            let root = std::env::var_os("APPDATA").ok_or(Error::Storage)?;
            if root.is_empty() || !Path::new(&root).is_absolute() {
                return Err(Error::Storage);
            }
            Ok(Self::new(
                PathBuf::from(root)
                    .join("Kairos")
                    .join("keepassxc-association.dpapi"),
            ))
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AssociationStore for DpapiStore {
    fn load(&mut self) -> Result<Option<Association>, Error> {
        #[cfg(windows)]
        {
            use std::io::Read;
            let file = match std::fs::File::open(&self.path) {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(_) => return Err(Error::Storage),
            };
            let mut protected = Vec::new();
            file.take((MAX_STORED_BYTES + 1) as u64)
                .read_to_end(&mut protected)
                .map_err(|_| Error::Storage)?;
            if protected.len() > MAX_STORED_BYTES {
                return Err(Error::Storage);
            }
            let plaintext = dpapi(&protected, false)?;
            Association::from_bytes(&plaintext).map(Some)
        }
        #[cfg(not(windows))]
        {
            Err(Error::UnsupportedPlatform)
        }
    }

    fn save(&mut self, association: &Association) -> Result<(), Error> {
        #[cfg(windows)]
        {
            use std::io::Write;
            let plaintext = association.to_bytes()?;
            let protected = dpapi(&plaintext, true)?;
            if protected.len() > MAX_STORED_BYTES {
                return Err(Error::Storage);
            }
            let parent = self
                .path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .ok_or(Error::Storage)?;
            std::fs::create_dir_all(parent).map_err(|_| Error::Storage)?;
            let mut pending =
                tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Storage)?;
            pending.write_all(&protected).map_err(|_| Error::Storage)?;
            pending.as_file().sync_all().map_err(|_| Error::Storage)?;
            pending.persist_noclobber(&self.path).map_err(|error| {
                if error.error.kind() == std::io::ErrorKind::AlreadyExists {
                    Error::AlreadyAssociated
                } else {
                    Error::Storage
                }
            })?;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            let _ = association;
            Err(Error::UnsupportedPlatform)
        }
    }
}

#[cfg(windows)]
fn dpapi(bytes: &[u8], protect: bool) -> Result<Zeroizing<Vec<u8>>, Error> {
    use std::{ptr, slice};
    use windows_sys::Win32::{
        Foundation::LocalFree,
        Security::Cryptography::{
            CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
        },
    };
    use zeroize::Zeroize;

    let input = CRYPT_INTEGER_BLOB {
        cbData: u32::try_from(bytes.len()).map_err(|_| Error::KeyProtection)?,
        pbData: bytes.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };
    // SAFETY: DPAPI borrows initialized input and allocates output. It does not
    // mutate the input. No LOCAL_MACHINE flag, description, entropy or UI is used.
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
        return Err(Error::KeyProtection);
    }
    if output.pbData.is_null() {
        return Err(Error::KeyProtection);
    }
    // SAFETY: Successful DPAPI output owns cbData initialized bytes from
    // LocalAlloc. Both protection directions scrub that allocation before free.
    let result = unsafe {
        let allocation = slice::from_raw_parts_mut(output.pbData, output.cbData as usize);
        let result = Zeroizing::new(allocation.to_vec());
        allocation.zeroize();
        LocalFree(output.pbData.cast());
        result
    };
    Ok(result)
}
