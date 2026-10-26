//! Explicit word exclusions, never a password history.
//!
//! Modelled after: xkcdpass word filtering before sampling.
//! <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.

use std::{collections::BTreeSet, io, path::PathBuf};

#[derive(Default)]
pub struct BlockedWords {
    pub words: BTreeSet<String>,
    pub path: Option<PathBuf>,
    pub error: Option<&'static str>,
}

/// Embedded dictionaries retain only lowercase ASCII spellings of 3–9 letters.
pub fn normalize(word: &str) -> Option<String> {
    let word = word.trim().to_ascii_lowercase();
    ((3..=9).contains(&word.len()) && word.bytes().all(|byte| byte.is_ascii_lowercase()))
        .then_some(word)
}

impl BlockedWords {
    pub fn configured() -> Self {
        let path = std::env::var_os("KAIROS_BLOCKED_WORDS_FILE")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("APPDATA")
                    .map(|root| PathBuf::from(root).join("Kairos/blocked-words.txt"))
            });
        match path {
            Some(path) => Self::load(path),
            None => Self { error: Some("Kein Sperrlistenpfad verfügbar. APPDATA oder KAIROS_BLOCKED_WORDS_FILE setzen."), ..Self::default() },
        }
    }

    pub fn load(path: PathBuf) -> Self {
        let mut result = Self {
            path: Some(path.clone()),
            ..Self::default()
        };
        let read = (|| -> io::Result<String> {
            use std::io::Read;
            let mut text = String::new();
            std::fs::File::open(path)?
                .take(1_048_577)
                .read_to_string(&mut text)?;
            if text.len() > 1_048_576 {
                return Err(io::Error::other("blocked words file too large"));
            }
            Ok(text)
        })();
        match read {
            Ok(text) => {
                for line in text.lines().filter(|line| !line.trim().is_empty()) {
                    if let Some(word) = normalize(line) {
                        result.words.insert(word);
                    } else {
                        result.error = Some("Sperrliste enthält ungültige Zeilen. Generierung gesperrt; Datei korrigieren und neu laden.");
                    }
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(_) => result.error = Some("Sperrliste konnte nicht gelesen werden. Generierung gesperrt; Datei prüfen und neu laden."),
        }
        result
    }

    pub fn change(&mut self, word: &str, blocked: bool) -> io::Result<()> {
        if self.error.is_some() {
            return Err(io::Error::other("blocked words unavailable"));
        }
        let word = normalize(word).ok_or_else(|| io::Error::other("invalid word"))?;
        let mut words = self.words.clone();
        if blocked {
            words.insert(word);
        } else {
            words.remove(&word);
        }
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| io::Error::other("missing path"))?;
        let parent = path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new("."));
        std::fs::create_dir_all(parent)?;
        let mut file = tempfile::NamedTempFile::new_in(parent)?;
        use std::io::Write;
        for word in &words {
            writeln!(file, "{word}")?;
        }
        file.as_file().sync_all()?;
        file.persist(path).map_err(|error| error.error)?;
        self.words = words;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_words_persist_and_can_be_removed_without_passwords() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("Kairos/blocked-words.txt");
        let mut list = BlockedWords::load(path.clone());
        list.change(" AhOrN ", true).unwrap();
        list.change("ahorn", true).unwrap();
        list.change("WIESE", true).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "ahorn\nwiese\n");
        let mut loaded = BlockedWords::load(path.clone());
        assert!(loaded.error.is_none());
        assert_eq!(loaded.words, list.words);
        loaded.change("AHORN", false).unwrap();
        assert_eq!(std::fs::read_to_string(path).unwrap(), "wiese\n");
        for invalid in ["ahorn-wiese", "w1ese", "Öl", "a", "muchtoolong"] {
            assert!(loaded.change(invalid, true).is_err());
        }
    }

    #[test]
    fn unreadable_or_invalid_files_block_generation_and_cannot_be_overwritten() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("blocked.txt");
        std::fs::write(&path, "AHORN\ninvalid-password\n").unwrap();
        let mut list = BlockedWords::load(path.clone());
        assert!(list.words.contains("ahorn"));
        assert!(list.error.is_some());
        assert!(list.change("wiese", true).is_err());
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "AHORN\ninvalid-password\n"
        );
        assert!(BlockedWords::load(root.path().to_owned()).error.is_some());
        std::fs::write(&path, vec![b'a'; 1_048_577]).unwrap();
        assert!(BlockedWords::load(path).error.is_some());
    }

    #[test]
    fn failed_save_keeps_previous_filter() {
        let root = tempfile::tempdir().unwrap();
        let mut list = BlockedWords::load(root.path().join("blocked.txt"));
        list.change("ahorn", true).unwrap();
        list.path = Some(root.path().to_owned());
        assert!(list.change("wiese", true).is_err());
        assert_eq!(list.words, BTreeSet::from(["ahorn".into()]));
    }
}
