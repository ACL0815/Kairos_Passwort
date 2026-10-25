//! Modelled after: KeePassXC KDBX4 authenticated metadata and content hashes.
//! <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/streams/HmacBlockStream.cpp>.
//! Modelled after: TUF manifests binding lengths, identities and content hashes.
//! <https://github.com/theupdateframework/specification/blob/v1.0.36/tuf-spec.md>.
//! Independent local format, not a KDBX/TUF implementation or publisher signature.

use std::{io::Read, path::PathBuf};

use hmac::{Hmac, Mac};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, Zeroizing};

use crate::{DatabaseKey, EncryptedStore, StoreError};

type HmacSha256 = Hmac<Sha256>;
const DOMAIN: &[u8] = b"Kairos word-list manifest v1\0";

/// A startup warning; absence is deliberately distinct from successful checking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "integrity warnings must be surfaced before using external word lists"]
pub enum IntegrityStatus {
    Unchanged,
    Changed,
    ManifestMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceStatus {
    Unchanged,
    Changed,
    /// The optional original file could not be read. The stored list is intact.
    Unavailable(std::io::ErrorKind),
    NoSourcePath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceCheck {
    pub list_id: i64,
    pub status: SourceStatus,
}

impl EncryptedStore {
    /// Authenticates the manifest and compares all list metadata and word rows
    /// within one database read snapshot. Does not silently repair or re-sign.
    /// SQLCipher corruption may instead return a database/authentication error.
    pub fn verify_integrity(&self) -> Result<IntegrityStatus, StoreError> {
        let tx = self.connection.unchecked_transaction()?;
        verify(&tx, &self.integrity_key)
    }

    /// Explicitly trusts the current database as a new baseline, for migration
    /// or user-reviewed recovery only. Never call automatically on startup.
    /// Prefer reimporting a known source into a new store after a warning.
    pub fn accept_current_integrity(&mut self) -> Result<(), StoreError> {
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        create_table(&tx)?;
        write_manifest(&tx, &self.integrity_key)?;
        tx.commit()?;
        Ok(())
    }

    /// Optional, potentially slow file I/O, separate from startup authentication.
    /// Only follows authenticated paths and reports source changes as advice.
    /// Call only for trusted local paths; this can touch removable/network media.
    pub fn verify_original_sources(&self) -> Result<Vec<SourceCheck>, StoreError> {
        let tx = self.connection.unchecked_transaction()?;
        require_unchanged(&tx, &self.integrity_key)?;
        let mut query =
            tx.prepare("SELECT id, source_path, source_sha256 FROM lists ORDER BY id")?;
        let rows = query.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, [u8; 32]>(2)?,
            ))
        })?;
        rows.map(|row| {
            let (list_id, path, hash) = row?;
            let status = match path {
                None => SourceStatus::NoSourcePath,
                Some(path) => match hash_file(PathBuf::from(path)) {
                    Ok(current) if current == hash => SourceStatus::Unchanged,
                    Ok(_) => SourceStatus::Changed,
                    Err(error) => SourceStatus::Unavailable(error.kind()),
                },
            };
            Ok(SourceCheck { list_id, status })
        })
        .collect()
    }
}

fn hash_file(path: PathBuf) -> std::io::Result<[u8; 32]> {
    let mut file = std::fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = Zeroizing::new([0; 8192]);
    loop {
        let count = file.read(buffer.as_mut_slice())?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().into())
}

pub(crate) fn derive_key(key: &DatabaseKey) -> Box<Zeroizing<[u8; 32]>> {
    // HMAC is a PRF over the existing uniform 256-bit DPAPI-protected secret.
    // Domain separation avoids using the SQLCipher raw key directly as MAC key.
    let mut mac =
        HmacSha256::new_from_slice(key.0.as_slice()).expect("HMAC accepts any key length");
    mac.update(b"Kairos independent manifest authentication key v1\0");
    let mut output = Box::new(Zeroizing::new([0; 32]));
    let mut derived = mac.finalize().into_bytes();
    output.copy_from_slice(&derived);
    derived.as_mut_slice().zeroize();
    output
}

pub(crate) fn create_table(db: &Connection) -> Result<(), StoreError> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS integrity_manifest (
        id INTEGER PRIMARY KEY CHECK(id = 1),
        payload BLOB NOT NULL,
        tag BLOB NOT NULL CHECK(length(tag) = 32)
    );",
    )?;
    Ok(())
}

fn field(output: &mut Vec<u8>, bytes: &[u8]) {
    output.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    output.extend_from_slice(bytes);
}

/// Canonical, versioned, length-prefixed binary manifest. Word hashes bind row
/// identities and languages as well as bytes; IDs preserve generator ordering.
fn snapshot(db: &Connection) -> Result<Vec<u8>, StoreError> {
    let mut output = DOMAIN.to_vec();
    let count: i64 = db.query_row("SELECT count(*) FROM lists", [], |r| r.get(0))?;
    output.extend_from_slice(&count.to_le_bytes());
    let all_words: i64 = db.query_row("SELECT count(*) FROM words", [], |r| r.get(0))?;
    output.extend_from_slice(&all_words.to_le_bytes());
    let mut query = db.prepare(
        "SELECT id, name, imported_at, source_sha256, source_path FROM lists ORDER BY id",
    )?;
    let mut rows = query.query([])?;
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        output.extend_from_slice(&id.to_le_bytes());
        field(&mut output, row.get::<_, String>(1)?.as_bytes());
        output.extend_from_slice(&row.get::<_, i64>(2)?.to_le_bytes());
        field(&mut output, &row.get::<_, Vec<u8>>(3)?);
        let path: Option<String> = row.get(4)?;
        output.push(u8::from(path.is_some()));
        field(&mut output, path.as_deref().unwrap_or("").as_bytes());
        let mut hash = Sha256::new();
        hash.update(b"Kairos stored words v1\0");
        let mut words =
            db.prepare("SELECT id, word, language FROM words WHERE list_id = ?1 ORDER BY id")?;
        let mut words = words.query([id])?;
        let mut count = 0u64;
        while let Some(word) = words.next()? {
            hash.update(word.get::<_, i64>(0)?.to_le_bytes());
            for index in [1, 2] {
                let text = Zeroizing::new(word.get::<_, String>(index)?);
                let bytes = text.as_bytes();
                hash.update((bytes.len() as u64).to_le_bytes());
                hash.update(bytes);
            }
            count += 1;
        }
        output.extend_from_slice(&count.to_le_bytes());
        output.extend_from_slice(&hash.finalize());
    }
    Ok(output)
}

pub(crate) fn write_manifest(db: &Connection, key: &[u8]) -> Result<(), StoreError> {
    let payload = snapshot(db)?;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(&payload);
    db.execute(
        "INSERT OR REPLACE INTO integrity_manifest(id, payload, tag) VALUES (1, ?1, ?2)",
        params![payload, mac.finalize().into_bytes().as_slice()],
    )?;
    Ok(())
}

fn verify(db: &Connection, key: &[u8]) -> Result<IntegrityStatus, StoreError> {
    let exists: bool = db.query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='integrity_manifest')", [], |r| r.get(0))?;
    if !exists {
        return Ok(IntegrityStatus::ManifestMissing);
    }
    let record = db
        .query_row(
            "SELECT payload, tag FROM integrity_manifest WHERE id = 1",
            [],
            |r| Ok((r.get::<_, Vec<u8>>(0)?, r.get::<_, Vec<u8>>(1)?)),
        )
        .optional()?;
    let Some((payload, tag)) = record else {
        return Ok(IntegrityStatus::ManifestMissing);
    };
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(&payload);
    if mac.verify_slice(&tag).is_err() {
        return Ok(IntegrityStatus::Changed);
    }
    Ok(if snapshot(db)? == payload {
        IntegrityStatus::Unchanged
    } else {
        IntegrityStatus::Changed
    })
}

pub(crate) fn require_unchanged(db: &Connection, key: &[u8]) -> Result<(), StoreError> {
    match verify(db, key)? {
        IntegrityStatus::Unchanged => Ok(()),
        warning => Err(StoreError::IntegrityWarning(warning)),
    }
}

#[cfg(test)]
mod tests;
