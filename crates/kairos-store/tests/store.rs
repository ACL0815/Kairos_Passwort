use std::{collections::BTreeSet, fs};

use kairos_core::{Language, WordSource};
use kairos_store::{DatabaseKey, EncryptedStore, ImportRules, StoreError};
use sha2::{Digest, Sha256};
use tempfile::tempdir;
use zeroize::Zeroizing;

fn words(source: &dyn WordSource) -> BTreeSet<&str> {
    (0..source.len()).map(|i| source.word(i)).collect()
}

#[test]
fn encrypted_file_roundtrip_preserves_provenance_and_language() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("words.db");
    let input = dir.path().join("input.txt");
    let bytes = b"\xef\xbb\xbf11111\tApfel\r\n11112 Birne\r\n";
    fs::write(&input, bytes).unwrap();
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&path, &key).unwrap();
    let outcome = store
        .import_file(&input, "Obst", Language::German, &ImportRules::default())
        .unwrap();
    assert_eq!(outcome.report.accepted, 2);
    assert_eq!(
        outcome.list.source_sha256,
        <[u8; 32]>::from(Sha256::digest(bytes))
    );
    assert!(outcome.list.imported_at_unix_seconds > 0);
    assert_eq!(
        outcome.list.source_path,
        Some(input.canonicalize().unwrap())
    );
    drop(store);
    let store = EncryptedStore::open_with_key(&path, &key).unwrap();
    assert_eq!(store.lists().unwrap(), vec![outcome.list.clone()]);
    let source = store.load(Some(outcome.list.id), None).unwrap();
    assert_eq!(words(&source), BTreeSet::from(["apfel", "birne"]));
    assert!((0..source.len()).all(|i| source.language(i) == Language::German));
}

#[test]
fn wrong_key_and_missing_key_cannot_read_database() {
    let dir = tempdir().unwrap();
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
    drop(store);
    let before = fs::read(&path).unwrap();
    assert!(matches!(
        EncryptedStore::open_with_key(&path, &DatabaseKey::generate().unwrap()),
        Err(StoreError::AuthenticationFailed)
    ));
    let plain = rusqlite::Connection::open(&path).unwrap();
    assert!(plain.prepare("SELECT word FROM words").is_err());
    drop(plain);
    assert_eq!(before, fs::read(&path).unwrap());
    assert!(!before.starts_with(b"SQLite format 3"));
    for word in [b"apfel".as_slice(), b"birne", b"Test"] {
        assert!(!before.windows(word.len()).any(|window| window == word));
    }
}

#[test]
fn all_key_bytes_including_nuls_are_significant() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("words.db");
    let mut bytes = Zeroizing::new([7; 32]);
    bytes[0] = 0;
    let first = DatabaseKey::from_bytes(bytes.clone());
    drop(EncryptedStore::open_with_key(&path, &first).unwrap());
    bytes[31] = 8;
    let second = DatabaseKey::from_bytes(bytes);
    assert!(matches!(
        EncryptedStore::open_with_key(&path, &second),
        Err(StoreError::AuthenticationFailed)
    ));
    assert!(EncryptedStore::open_with_key(&path, &first).is_ok());
}

#[test]
fn validation_counts_each_rejected_line_once() {
    let dir = tempdir().unwrap();
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&dir.path().join("words.db"), &key).unwrap();
    let input = "\u{feff}# Quelle\r\n\r\n APFEL \r\n11111 apfel\n11112\tBirne\n1111 haus\n111111 boot\n77777 wort\nzwei woerter\n11113 wort extra\nÖl\nßass\nabc!\nab\nabcdefghij\n";
    let result = store
        .import_bytes(
            input.as_bytes(),
            "Test",
            Language::German,
            &ImportRules::default(),
        )
        .unwrap();
    let report = result.report;
    assert!(result.list.source_path.is_none());
    assert_eq!(report.accepted, 4);
    assert_eq!(report.blank, 1);
    assert_eq!(report.comment, 1);
    assert_eq!(report.duplicate, 1);
    assert_eq!(report.invalid_format, 3);
    assert_eq!(report.invalid_characters, 3);
    assert_eq!(report.too_short, 1);
    assert_eq!(report.too_long, 1);
    assert_eq!(report.accepted + report.rejected(), input.lines().count());
}

#[test]
fn unicode_rules_are_explicit_and_count_characters() {
    let dir = tempdir().unwrap();
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&dir.path().join("words.db"), &key).unwrap();
    let rules = ImportRules {
        min_length: 2,
        max_length: 3,
        allowed_characters: "abcdefghijklmnopqrstuvwxyzäöüß".into(),
    };
    let result = store
        .import_bytes(
            "ÖL\nGRÜN\nMaß".as_bytes(),
            "Unicode",
            Language::German,
            &rules,
        )
        .unwrap();
    assert_eq!(result.report.accepted, 2);
    assert_eq!(result.report.too_long, 1);
    assert_eq!(
        words(&store.load(None, None).unwrap()),
        BTreeSet::from(["öl", "maß"])
    );
}

#[test]
fn invalid_inputs_do_not_create_lists_or_words() {
    let dir = tempdir().unwrap();
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&dir.path().join("words.db"), &key).unwrap();
    assert!(matches!(
        store.import_bytes(&[0xff], "Bad", Language::German, &ImportRules::default()),
        Err(StoreError::InvalidEncoding)
    ));
    assert!(
        matches!(store.import_bytes(b"123", "Empty", Language::German, &ImportRules::default()), Err(StoreError::EmptyImport(report)) if report.accepted == 0 && report.invalid_characters == 1)
    );
    for rules in [
        ImportRules {
            min_length: 0,
            ..ImportRules::default()
        },
        ImportRules {
            min_length: 10,
            max_length: 3,
            ..ImportRules::default()
        },
        ImportRules {
            allowed_characters: "".into(),
            ..ImportRules::default()
        },
        ImportRules {
            allowed_characters: "abc ".into(),
            ..ImportRules::default()
        },
        ImportRules {
            allowed_characters: "ABC".into(),
            ..ImportRules::default()
        },
        ImportRules {
            allowed_characters: "abc\0".into(),
            ..ImportRules::default()
        },
    ] {
        assert!(matches!(
            store.import_bytes(b"apfel", "Bad", Language::German, &rules),
            Err(StoreError::InvalidRules)
        ));
    }
    assert!(matches!(
        store.import_bytes(b"apfel", " ", Language::German, &ImportRules::default()),
        Err(StoreError::InvalidListName)
    ));
    assert!(matches!(
        store.import_bytes(
            b"apfel",
            "Bad",
            Language::Other("".into()),
            &ImportRules::default()
        ),
        Err(StoreError::InvalidLanguage)
    ));
    assert!(store.lists().unwrap().is_empty());
    assert!(store.load(None, None).unwrap().is_empty());
}

#[test]
fn list_and_language_filters_remove_cross_list_duplicates() {
    let dir = tempdir().unwrap();
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&dir.path().join("words.db"), &key).unwrap();
    let rules = ImportRules::default();
    let german = store
        .import_bytes(b"haus\nboot", "Deutsch", Language::German, &rules)
        .unwrap();
    store
        .import_bytes(b"boot\napple", "English", Language::English, &rules)
        .unwrap();
    store
        .import_bytes(b"domus", "Latin", Language::Latin, &rules)
        .unwrap();
    store
        .import_bytes(b"maison", "French", Language::Other("fr".into()), &rules)
        .unwrap();
    store
        .import_bytes(b"anders", "Other de", Language::Other("DE".into()), &rules)
        .unwrap();
    let source = store.load(None, None).unwrap();
    assert_eq!(source.len(), 6);
    let boot = (0..source.len())
        .find(|i| source.word(*i) == "boot")
        .unwrap();
    assert_eq!(source.language(boot), Language::German);
    assert_eq!(
        words(&store.load(Some(german.list.id), None).unwrap()),
        BTreeSet::from(["haus", "boot"])
    );
    let english = store.load(None, Some(&Language::English)).unwrap();
    assert_eq!(words(&english), BTreeSet::from(["boot", "apple"]));
    assert!((0..english.len()).all(|i| english.language(i) == Language::English));
    assert!(store
        .load(Some(german.list.id), Some(&Language::English))
        .unwrap()
        .is_empty());
    assert_eq!(
        store
            .load(None, Some(&Language::Other("de".into())))
            .unwrap()
            .len(),
        3
    );
    assert!(store.load(Some(i64::MAX), None).unwrap().is_empty());
}

#[test]
fn stored_order_is_stable_but_not_alphabetical_and_snapshot_is_independent() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("words.db");
    let key = DatabaseKey::generate().unwrap();
    let mut store = EncryptedStore::open_with_key(&path, &key).unwrap();
    let input: String = (b'a'..=b'z')
        .map(|c| format!("wort{}\n", char::from(c)))
        .collect();
    store
        .import_bytes(
            input.as_bytes(),
            "Alphabet",
            Language::German,
            &ImportRules::default(),
        )
        .unwrap();
    let snapshot = store.load(None, None).unwrap();
    let order: Vec<_> = (0..snapshot.len()).map(|i| snapshot.word(i)).collect();
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_ne!(order, sorted); // Probability of a legitimate sorted shuffle is 1/26!.
    assert_eq!(words(&snapshot).len(), 26);
    store
        .import_bytes(b"apfel", "More", Language::German, &ImportRules::default())
        .unwrap();
    drop(store);
    let store = EncryptedStore::open_with_key(&path, &key).unwrap();
    let source = store.load(Some(1), None).unwrap();
    assert_eq!(
        order,
        (0..source.len())
            .map(|i| source.word(i))
            .collect::<Vec<_>>()
    );
    assert_eq!(snapshot.len(), 26);
}

#[test]
fn plaintext_database_is_not_silently_adopted() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("plain.db");
    let plain = rusqlite::Connection::open(&path).unwrap();
    plain
        .execute_batch("CREATE TABLE existing(value TEXT); INSERT INTO existing VALUES ('keep');")
        .unwrap();
    drop(plain);
    let before = fs::read(&path).unwrap();
    assert!(EncryptedStore::open_with_key(&path, &DatabaseKey::generate().unwrap()).is_err());
    assert_eq!(before, fs::read(&path).unwrap());
}

#[cfg(windows)]
mod windows {
    use super::*;
    use kairos_store::StorePaths;

    fn paths(root: &std::path::Path) -> StorePaths {
        StorePaths {
            database: root.join("db/words.db"),
            key_file: root.join("keys/database-key.dpapi"),
        }
    }

    #[test]
    fn dpapi_reopens_and_key_file_has_no_plaintext_secret() {
        let dir = tempdir().unwrap();
        let paths = paths(dir.path());
        let mut store = EncryptedStore::open(&paths).unwrap();
        store
            .import_bytes(b"apfel", "Test", Language::German, &ImportRules::default())
            .unwrap();
        let protected = fs::read(&paths.key_file).unwrap();
        assert!(protected.len() > 32);
        drop(store);
        assert_eq!(
            EncryptedStore::open(&paths)
                .unwrap()
                .load(None, None)
                .unwrap()
                .word(0),
            "apfel"
        );
        assert_eq!(protected, fs::read(&paths.key_file).unwrap());
    }

    #[test]
    fn missing_key_for_existing_database_is_not_recreated() {
        let dir = tempdir().unwrap();
        let paths = paths(dir.path());
        drop(EncryptedStore::open(&paths).unwrap());
        fs::remove_file(&paths.key_file).unwrap();
        let before = fs::read(&paths.database).unwrap();
        assert!(matches!(
            EncryptedStore::open(&paths),
            Err(StoreError::MissingKey)
        ));
        assert!(!paths.key_file.exists());
        assert_eq!(before, fs::read(&paths.database).unwrap());
    }

    #[test]
    fn corrupt_key_is_not_replaced_even_without_a_database() {
        let dir = tempdir().unwrap();
        let paths = paths(dir.path());
        fs::create_dir_all(paths.key_file.parent().unwrap()).unwrap();
        fs::write(&paths.key_file, b"corrupt DPAPI blob").unwrap();
        assert!(matches!(
            EncryptedStore::open(&paths),
            Err(StoreError::KeyProtection)
        ));
        assert_eq!(fs::read(&paths.key_file).unwrap(), b"corrupt DPAPI blob");
        assert!(!paths.database.exists());
    }

    #[test]
    fn concurrent_initialization_uses_one_atomically_published_key() {
        let dir = tempdir().unwrap();
        let paths = paths(dir.path());
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
        let threads: Vec<_> = (0..4)
            .map(|_| {
                let paths = paths.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    drop(EncryptedStore::open(&paths).unwrap());
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
        assert!(EncryptedStore::open(&paths)
            .unwrap()
            .lists()
            .unwrap()
            .is_empty());
        assert_eq!(
            fs::read_dir(paths.key_file.parent().unwrap())
                .unwrap()
                .count(),
            1
        );
    }
}
