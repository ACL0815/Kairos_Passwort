//! Encrypted external word sources implementing [`kairos_core::WordSource`].
//!
//! Modelled after: SQLCipher
//! <https://github.com/sqlcipher/sqlcipher/tree/v4.19.0>.
//! Imports are transactional; encryption and random-source errors fail closed.

mod import;
mod key;

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use kairos_core::{Language, WordSource};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use rusqlite::{params, Connection, TransactionBehavior};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

pub use import::{ImportReport, ImportRules};
pub use key::DatabaseKey;

/// Operational errors never contain key bytes or rejected words.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StoreError {
    #[error("file operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("encrypted database operation failed: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("the system random source is unavailable")]
    RandomSourceUnavailable,
    #[error("DPAPI protection or key validation failed")]
    KeyProtection,
    #[error("the database exists but its protected key is missing")]
    MissingKey,
    #[error("DPAPI requires Windows")]
    UnsupportedPlatform,
    #[error("database and key paths must be distinct, with a key parent directory")]
    InvalidPaths,
    #[error("APPDATA is unavailable")]
    MissingAppData,
    #[error("invalid import rules")]
    InvalidRules,
    #[error("the source must be UTF-8")]
    InvalidEncoding,
    #[error("the list name must not be empty")]
    InvalidListName,
    #[error("no words passed validation")]
    EmptyImport(ImportReport),
    #[error("unsupported or inconsistent database schema")]
    UnsupportedSchema,
    #[error("invalid language metadata")]
    InvalidLanguage,
    #[error("the system clock or an integer value is out of range")]
    OutOfRange,
    #[error("SQLCipher is unavailable")]
    EncryptionUnavailable,
}

/// Explicit paths allow portable installations and isolated integration tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorePaths {
    pub database: PathBuf,
    pub key_file: PathBuf,
}

impl StorePaths {
    /// Uses `%APPDATA%\Kairos\words.db` and `database-key.dpapi`.
    pub fn for_current_user() -> Result<Self, StoreError> {
        let root = std::env::var_os("APPDATA")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .ok_or(StoreError::MissingAppData)?
            .join("Kairos");
        Ok(Self {
            database: root.join("words.db"),
            key_file: root.join("database-key.dpapi"),
        })
    }
}

/// Provenance refers to the original bytes, before BOM/CRLF normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListMetadata {
    pub id: i64,
    pub name: String,
    pub imported_at_unix_seconds: i64,
    pub source_sha256: [u8; 32],
    pub word_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportOutcome {
    pub list: ListMetadata,
    pub report: ImportReport,
}

/// A stable, distinct-word snapshot, independent of subsequent database changes.
///
/// Modelled after: KeePassXC
/// <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp>.
pub struct StoredWordList {
    words: Zeroizing<Vec<String>>,
    languages: Vec<Language>,
}

impl WordSource for StoredWordList {
    fn len(&self) -> usize {
        self.words.len()
    }

    fn word(&self, index: usize) -> &str {
        &self.words[index]
    }

    fn language(&self, index: usize) -> Language {
        self.languages[index].clone()
    }
}

/// An open SQLCipher connection. The native codec owns its working keys.
pub struct EncryptedStore {
    connection: Connection,
}

impl EncryptedStore {
    /// Loads or atomically creates a DPAPI CurrentUser key before opening SQLite.
    /// A missing/corrupt key for an existing database is never replaced.
    pub fn open(paths: &StorePaths) -> Result<Self, StoreError> {
        if paths.database == paths.key_file {
            return Err(StoreError::InvalidPaths);
        }
        let key = key::load_or_create(paths)?;
        Self::open_with_key(&paths.database, &key)
    }

    /// Opens/creates a database with an explicitly managed 256-bit secret.
    /// This does not save the key; normal Windows callers should use [`Self::open`].
    ///
    /// Modelled after: rusqlite / SQLCipher
    /// <https://github.com/rusqlite/rusqlite/tree/v0.41.0>.
    pub fn open_with_key(path: &Path, key: &DatabaseKey) -> Result<Self, StoreError> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        let mut connection = Connection::open(path)?;
        // SAFETY: The live connection is exclusively borrowed here. sqlite3_key
        // copies all 32 bytes synchronously, including NULs. No SQL string or
        // passphrase formatting makes another Rust plaintext key allocation.
        let status =
            unsafe { rusqlite::ffi::sqlite3_key(connection.handle(), key.0.as_ptr().cast(), 32) };
        if status != rusqlite::ffi::SQLITE_OK {
            return Err(StoreError::KeyProtection);
        }
        // rusqlite's pragma helpers consult SQLite's list, which omits codec pragmas.
        let cipher: String = connection.query_row("PRAGMA cipher_version", [], |r| r.get(0))?;
        if cipher.is_empty() {
            return Err(StoreError::EncryptionUnavailable);
        }
        // Reading a page verifies the key before any schema or file mutation.
        connection.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;
        connection.execute_batch(
            "PRAGMA cipher_memory_security = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA journal_mode = DELETE;
             PRAGMA foreign_keys = ON;
             PRAGMA secure_delete = ON;",
        )?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let version: i64 = tx.pragma_query_value(None, "user_version", |r| r.get(0))?;
        let app: i64 = tx.pragma_query_value(None, "application_id", |r| r.get(0))?;
        match (version, app) {
            (0, 0) => {
                let tables: i64 =
                    tx.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get(0))?;
                if tables != 0 {
                    return Err(StoreError::UnsupportedSchema);
                }
                tx.execute_batch(
                    "CREATE TABLE lists (
                        id INTEGER PRIMARY KEY,
                        name TEXT NOT NULL,
                        imported_at INTEGER NOT NULL,
                        source_sha256 BLOB NOT NULL CHECK(length(source_sha256) = 32)
                     );
                     CREATE TABLE words (
                        id INTEGER PRIMARY KEY,
                        list_id INTEGER NOT NULL REFERENCES lists(id),
                        word TEXT NOT NULL CHECK(length(word) > 0),
                        language TEXT NOT NULL
                     );
                     CREATE INDEX words_list ON words(list_id);
                     PRAGMA application_id = 1262573906;
                     PRAGMA user_version = 1;",
                )?;
            }
            (1, 1262573906) => {}
            _ => return Err(StoreError::UnsupportedSchema),
        }
        tx.commit()?;
        Ok(Self { connection })
    }

    /// Reads a UTF-8 file once so its hash and imported contents cannot diverge.
    pub fn import_file(
        &mut self,
        path: &Path,
        name: &str,
        language: Language,
        rules: &ImportRules,
    ) -> Result<ImportOutcome, StoreError> {
        let bytes = Zeroizing::new(std::fs::read(path)?);
        self.import_bytes(&bytes, name, language, rules)
    }

    /// Imports plain lines or 4–6 base-six Diceware digits followed by a word.
    /// All accepted entries are shuffled before receiving row IDs.
    ///
    /// Modelled after: rand SliceRandom
    /// <https://github.com/rust-random/rand/blob/0.8.5/src/seq/mod.rs>.
    pub fn import_bytes(
        &mut self,
        source: &[u8],
        name: &str,
        language: Language,
        rules: &ImportRules,
    ) -> Result<ImportOutcome, StoreError> {
        if name.trim().is_empty() {
            return Err(StoreError::InvalidListName);
        }
        let language = encode_language(&language)?;
        let (mut words, report) = import::parse(source, rules)?;
        if words.is_empty() {
            return Err(StoreError::EmptyImport(report));
        }
        let mut rng =
            StdRng::from_rng(rand::rngs::OsRng).map_err(|_| StoreError::RandomSourceUnavailable)?;
        words.shuffle(&mut rng);
        let source_sha256: [u8; 32] = Sha256::digest(source).into();
        let imported_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| StoreError::OutOfRange)?
            .as_secs()
            .try_into()
            .map_err(|_| StoreError::OutOfRange)?;
        let tx = self.connection.transaction()?;
        tx.execute(
            "INSERT INTO lists(name, imported_at, source_sha256) VALUES (?1, ?2, ?3)",
            params![name, imported_at, source_sha256.as_slice()],
        )?;
        let id = tx.last_insert_rowid();
        {
            let mut insert =
                tx.prepare("INSERT INTO words(list_id, word, language) VALUES (?1, ?2, ?3)")?;
            for word in words.iter() {
                insert.execute(params![id, word, language])?;
            }
        }
        tx.commit()?;
        Ok(ImportOutcome {
            list: ListMetadata {
                id,
                name: name.into(),
                imported_at_unix_seconds: imported_at,
                source_sha256,
                word_count: words.len(),
            },
            report,
        })
    }

    pub fn lists(&self) -> Result<Vec<ListMetadata>, StoreError> {
        let mut query = self.connection.prepare(
            "SELECT l.id, l.name, l.imported_at, l.source_sha256, count(w.id)
             FROM lists l LEFT JOIN words w ON w.list_id = l.id GROUP BY l.id ORDER BY l.id",
        )?;
        let rows = query.query_map([], |row| {
            let count: i64 = row.get(4)?;
            Ok(ListMetadata {
                id: row.get(0)?,
                name: row.get(1)?,
                imported_at_unix_seconds: row.get(2)?,
                source_sha256: row.get(3)?,
                word_count: count
                    .try_into()
                    .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(4, count))?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Filters by optional list ID and/or language and removes overlaps across
    /// lists. For an overlap the earliest stored entry supplies the language.
    pub fn load(
        &self,
        list_id: Option<i64>,
        language: Option<&Language>,
    ) -> Result<StoredWordList, StoreError> {
        let language = language.map(encode_language).transpose()?;
        let mut query = self.connection.prepare(
            "SELECT word, language FROM words
             WHERE (?1 IS NULL OR list_id = ?1) AND (?2 IS NULL OR language = ?2)
             ORDER BY id",
        )?;
        let mut rows = query.query(params![list_id, language])?;
        let mut source = StoredWordList {
            words: Zeroizing::new(Vec::new()),
            languages: Vec::new(),
        };
        let mut unique = HashSet::new();
        while let Some(row) = rows.next()? {
            let word: String = row.get(0)?;
            if word.is_empty() {
                return Err(StoreError::UnsupportedSchema);
            }
            if unique.insert(word.clone()) {
                source.words.push(word);
                source
                    .languages
                    .push(decode_language(&row.get::<_, String>(1)?)?);
            }
        }
        Ok(source)
    }
}

fn encode_language(language: &Language) -> Result<String, StoreError> {
    match language {
        Language::German => Ok("de".into()),
        Language::English => Ok("en".into()),
        Language::Latin => Ok("la".into()),
        Language::Other(tag) if !tag.trim().is_empty() => Ok(format!("other:{tag}")),
        _ => Err(StoreError::InvalidLanguage),
    }
}

fn decode_language(value: &str) -> Result<Language, StoreError> {
    match value {
        "de" => Ok(Language::German),
        "en" => Ok(Language::English),
        "la" => Ok(Language::Latin),
        _ => value
            .strip_prefix("other:")
            .filter(|tag| !tag.trim().is_empty())
            .map(|tag| Language::Other(tag.into()))
            .ok_or(StoreError::InvalidLanguage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_failure_rolls_back_list_and_words_together() {
        let dir = tempfile::tempdir().unwrap();
        let key = DatabaseKey::generate().unwrap();
        let mut store = EncryptedStore::open_with_key(&dir.path().join("words.db"), &key).unwrap();
        store.connection.execute_batch(
            "CREATE TRIGGER reject_word BEFORE INSERT ON words BEGIN SELECT RAISE(ABORT, 'test'); END;"
        ).unwrap();
        assert!(store
            .import_bytes(
                b"apfel\nbirne",
                "Fail",
                Language::German,
                &ImportRules::default()
            )
            .is_err());
        assert!(store.lists().unwrap().is_empty());
        assert!(store.load(None, None).unwrap().is_empty());
    }

    #[test]
    fn future_schema_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("words.db");
        let key = DatabaseKey::generate().unwrap();
        let store = EncryptedStore::open_with_key(&path, &key).unwrap();
        store
            .connection
            .pragma_update(None, "user_version", 2)
            .unwrap();
        drop(store);
        assert!(matches!(
            EncryptedStore::open_with_key(&path, &key),
            Err(StoreError::UnsupportedSchema)
        ));
    }

    #[test]
    fn rollback_journal_contains_no_plaintext_words() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("words.db");
        let key = DatabaseKey::generate().unwrap();
        let mut store = EncryptedStore::open_with_key(&path, &key).unwrap();
        store
            .import_bytes(
                b"apfel\nbirne",
                "Test",
                Language::German,
                &ImportRules::default(),
            )
            .unwrap();
        store
            .connection
            .execute_batch("BEGIN IMMEDIATE; UPDATE words SET word = 'kirsche';")
            .unwrap();
        let journal = std::fs::read(dir.path().join("words.db-journal")).unwrap();
        assert!(!journal.is_empty());
        for marker in [b"apfel".as_slice(), b"birne", b"kirsche"] {
            assert!(!journal.windows(marker.len()).any(|window| window == marker));
        }
        store.connection.execute_batch("ROLLBACK").unwrap();
        assert_eq!(store.load(None, None).unwrap().len(), 2);
    }
}
