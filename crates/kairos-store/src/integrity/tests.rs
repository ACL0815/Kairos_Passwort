use super::*;
use crate::{ImportRules, Language};
use kairos_core::WordSource;

fn store() -> (tempfile::TempDir, EncryptedStore) {
    let dir = tempfile::tempdir().unwrap();
    let mut store = EncryptedStore::open_with_key(
        &dir.path().join("words.db"),
        &DatabaseKey::generate().unwrap(),
    )
    .unwrap();
    store
        .import_bytes(
            b"apfel\nbirne",
            "Obst",
            Language::German,
            &ImportRules::default(),
        )
        .unwrap();
    (dir, store)
}

#[test]
fn changed_words_metadata_membership_and_manifest_are_detected() {
    for sql in [
        "UPDATE words SET word = 'falsch' WHERE id = 1",
        "UPDATE words SET language = 'en' WHERE id = 1",
        "UPDATE words SET id = id + 10",
        "UPDATE lists SET name = 'Changed'",
        "UPDATE lists SET imported_at = imported_at + 1",
        "UPDATE lists SET source_sha256 = zeroblob(32)",
        "UPDATE lists SET source_path = 'C:/changed.txt'",
        "DELETE FROM words WHERE id = 1",
        "INSERT INTO words(list_id, word, language) VALUES (1, 'extra', 'de')",
        "DELETE FROM words; DELETE FROM lists",
        "UPDATE integrity_manifest SET tag = zeroblob(32)",
        "UPDATE integrity_manifest SET payload = X'00'",
        "CREATE TRIGGER inject_word AFTER INSERT ON words BEGIN UPDATE words SET word = 'falsch'; END",
        "CREATE VIEW injected_view AS SELECT * FROM words",
        "CREATE INDEX injected_index ON words(word)",
        "PRAGMA user_version = 1",
        "PRAGMA user_version = 3",
        "PRAGMA application_id = 0",
    ] {
        let (_dir, mut store) = store();
        assert_eq!(
            store.verify_integrity().unwrap(),
            IntegrityStatus::Unchanged
        );
        store.connection.execute_batch(sql).unwrap();
        assert_eq!(
            store.verify_integrity().unwrap(),
            IntegrityStatus::Changed,
            "{sql}"
        );
        assert!(matches!(
            store.load(None, None),
            Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
        ));
        assert!(matches!(
            store.lists(),
            Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
        ));
        assert!(matches!(
            store.import_bytes(b"haus", "New", Language::German, &ImportRules::default()),
            Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
        ));
    }
}

#[test]
fn removed_schema_two_manifest_is_changed_and_cannot_be_accepted() {
    for sql in [
        "DELETE FROM integrity_manifest",
        "DROP TABLE integrity_manifest",
    ] {
        let dir = tempfile::tempdir().unwrap();
        let key = DatabaseKey::generate().unwrap();
        let path = dir.path().join("words.db");
        let store = EncryptedStore::open_with_key(&path, &key).unwrap();
        store.connection.execute_batch(sql).unwrap();
        drop(store);
        let mut store = EncryptedStore::open_with_key(&path, &key).unwrap();
        assert_eq!(store.verify_integrity().unwrap(), IntegrityStatus::Changed);
        assert!(matches!(
            store.load(None, None),
            Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
        ));
        assert!(matches!(
            store.accept_current_integrity(),
            Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
        ));
    }
}

#[test]
fn legacy_schema_migrates_only_after_explicit_acceptance() {
    let dir = tempfile::tempdir().unwrap();
    let key = DatabaseKey::generate().unwrap();
    let path = dir.path().join("legacy.db");
    let mut legacy = EncryptedStore::open_with_key(&path, &key).unwrap();
    legacy
        .import_bytes(b"haus", "Legacy", Language::German, &ImportRules::default())
        .unwrap();
    // Exact legacy schema: no manifest table and user_version 1.
    legacy
        .connection
        .execute_batch("DROP TABLE integrity_manifest; PRAGMA user_version = 1")
        .unwrap();
    drop(legacy);
    let mut legacy = EncryptedStore::open_with_key(&path, &key).unwrap();
    assert_eq!(
        legacy.verify_integrity().unwrap(),
        IntegrityStatus::ManifestMissing
    );
    assert!(matches!(
        legacy.lists(),
        Err(StoreError::IntegrityWarning(
            IntegrityStatus::ManifestMissing
        ))
    ));
    assert!(matches!(
        legacy.load(None, None),
        Err(StoreError::IntegrityWarning(
            IntegrityStatus::ManifestMissing
        ))
    ));
    assert!(matches!(
        legacy.import_bytes(b"boot", "New", Language::German, &ImportRules::default()),
        Err(StoreError::IntegrityWarning(
            IntegrityStatus::ManifestMissing
        ))
    ));
    legacy.accept_current_integrity().unwrap();
    assert_eq!(
        legacy.verify_integrity().unwrap(),
        IntegrityStatus::Unchanged
    );
    let version: i64 = legacy
        .connection
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(version, 2, "legacy binaries only accept schema 1");
    assert_eq!(legacy.lists().unwrap()[0].name, "Legacy");
    assert_eq!(legacy.load(None, None).unwrap().word(0), "haus");
    drop(legacy);
    let mut upgraded = EncryptedStore::open_with_key(&path, &key).unwrap();
    assert_eq!(
        upgraded.verify_integrity().unwrap(),
        IntegrityStatus::Unchanged
    );
    upgraded
        .connection
        .pragma_update(None, "user_version", 1)
        .unwrap();
    assert_eq!(
        upgraded.verify_integrity().unwrap(),
        IntegrityStatus::Changed
    );
    assert!(matches!(
        upgraded.accept_current_integrity(),
        Err(StoreError::IntegrityWarning(IntegrityStatus::Changed))
    ));
}

#[test]
fn wrong_manifest_key_fails_authentication() {
    let (_dir, mut store) = store();
    store.integrity_key[31] ^= 1;
    assert_eq!(store.verify_integrity().unwrap(), IntegrityStatus::Changed);
}

#[test]
fn manifest_failure_rolls_back_the_entire_import() {
    let (_dir, mut store) = store();
    let before = store.lists().unwrap();
    store.connection.execute_batch("CREATE TRIGGER reject_manifest BEFORE INSERT ON integrity_manifest WHEN (SELECT count(*) FROM lists) > 1 BEGIN SELECT RAISE(ABORT, 'test'); END;").unwrap();
    // Authenticate the deliberate failure fixture; import must reach its commit.
    write_manifest(&store.connection, store.integrity_key.as_slice()).unwrap();
    assert!(store
        .import_bytes(
            b"haus",
            "Rejected",
            Language::German,
            &ImportRules::default()
        )
        .is_err());
    assert_eq!(store.lists().unwrap(), before);
    assert_eq!(
        store.verify_integrity().unwrap(),
        IntegrityStatus::Unchanged
    );
}

#[test]
fn original_source_changes_are_advisory_and_unavailable_is_distinct() {
    let (dir, mut store) = store();
    let input = dir.path().join("input.txt");
    std::fs::write(&input, b"haus").unwrap();
    store
        .import_file(&input, "File", Language::German, &ImportRules::default())
        .unwrap();
    assert_eq!(
        store
            .verify_original_sources()
            .unwrap()
            .iter()
            .map(|c| c.status.clone())
            .collect::<Vec<_>>(),
        vec![SourceStatus::NoSourcePath, SourceStatus::Unchanged]
    );
    std::fs::write(&input, b"boot").unwrap();
    assert_eq!(
        store.verify_original_sources().unwrap()[1].status,
        SourceStatus::Changed
    );
    std::fs::remove_file(&input).unwrap();
    assert_eq!(
        store.verify_original_sources().unwrap()[1].status,
        SourceStatus::Unavailable(std::io::ErrorKind::NotFound)
    );
    assert_eq!(
        store.verify_integrity().unwrap(),
        IntegrityStatus::Unchanged
    );
}

#[test]
#[cfg(windows)]
fn manifest_reopens_with_the_same_dpapi_protected_key() {
    let dir = tempfile::tempdir().unwrap();
    let paths = crate::StorePaths {
        database: dir.path().join("words.db"),
        key_file: dir.path().join("key.dpapi"),
    };
    let mut store = EncryptedStore::open(&paths).unwrap();
    store
        .import_bytes(b"haus", "File", Language::German, &ImportRules::default())
        .unwrap();
    assert_eq!(
        store.verify_integrity().unwrap(),
        IntegrityStatus::Unchanged
    );
    drop(store);
    assert_eq!(
        EncryptedStore::open(&paths)
            .unwrap()
            .verify_integrity()
            .unwrap(),
        IntegrityStatus::Unchanged
    );
}

#[test]
fn length_prefixes_disambiguate_fields() {
    let mut first = vec![];
    field(&mut first, b"ab");
    field(&mut first, b"c");
    let mut second = vec![];
    field(&mut second, b"a");
    field(&mut second, b"bc");
    assert_ne!(first, second);
}
