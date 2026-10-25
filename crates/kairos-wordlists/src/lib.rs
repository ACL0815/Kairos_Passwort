//! Embedded word sources implementing [`kairos_core::WordSource`].
//!
//! Word data is compiled into the executable; no runtime file or network access
//! is needed. The German source contains distinct ASCII words of 3–9 letters.
//!
//! Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist>.
//! Modelled after: passalong-core <https://docs.rs/passalong-core/0.3.0/src/passalong_core/crypto/words.rs.html>.
//! Only the separation of embedded data from password generation is modelled;
//! no implementation code was copied. See the crate README for data licenses.

use std::sync::OnceLock;

use kairos_core::{Language, WordSource};

/// An immutable, indexed word list backed by text embedded in the executable.
///
/// Constructed through [`german`]; entries borrow the embedded text rather than
/// allocating a separate string for each word.
pub struct EmbeddedWordList {
    words: Vec<&'static str>,
}

impl WordSource for EmbeddedWordList {
    fn len(&self) -> usize {
        self.words.len()
    }

    fn word(&self, index: usize) -> &str {
        self.words[index]
    }

    fn language(&self, index: usize) -> Language {
        // Apply the same index bounds as `word`, even for a single-language list.
        let _ = self.words[index];
        Language::German
    }
}

/// Returns the shared, embedded German word source.
///
/// A thread-safe index is initialized once. No external dictionary is needed.
/// Use [`WordSource::len`] for the actual vocabulary size when calculating
/// entropy; the list is smaller than the concept's 21,000-word example.
///
/// Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist>.
/// Modelled after: passalong-core <https://docs.rs/passalong-core/0.3.0/src/passalong_core/crypto/words.rs.html>.
///
/// ```
/// use kairos_core::{Language, WordSource};
/// let words = kairos_wordlists::german();
/// assert!(words.len() >= 7_000);
/// assert_eq!(words.language(0), Language::German);
/// ```
pub fn german() -> &'static EmbeddedWordList {
    static GERMAN: OnceLock<EmbeddedWordList> = OnceLock::new();
    GERMAN.get_or_init(|| EmbeddedWordList {
        words: include_str!("../data/german.txt").lines().collect(),
    })
}
