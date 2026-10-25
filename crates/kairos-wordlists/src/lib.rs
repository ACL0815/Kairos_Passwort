//! Embedded word sources implementing [`kairos_core::WordSource`].
//!
//! Word data is compiled into the executable; no runtime file or network access
//! is needed. All sources contain distinct ASCII words of 3–9 letters.
//! [`combined`] exposes the disjoint union for independent language choice at
//! each uniformly sampled word index. Sampling belongs to `kairos-core`.
//!
//! Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist>.
//! Modelled after: passalong-core <https://docs.rs/passalong-core/0.3.0/src/passalong_core/crypto/words.rs.html>.
//! Only the separation of embedded data from password generation is modelled;
//! no implementation code was copied. See the crate README for data licenses.

use std::sync::OnceLock;

use kairos_core::{Language, WordSource};

/// An immutable, indexed word list backed by text embedded in the executable.
///
/// Constructed through [`german`], [`english`], [`latin`], or [`combined`]; entries
/// borrow the embedded text rather than allocating a string for each word.
pub struct EmbeddedWordList {
    words: Vec<(&'static str, Language)>,
}

impl WordSource for EmbeddedWordList {
    fn len(&self) -> usize {
        self.words.len()
    }

    fn word(&self, index: usize) -> &str {
        self.words[index].0
    }

    fn language(&self, index: usize) -> Language {
        self.words[index].1.clone()
    }
}

/// Returns the shared, embedded German word source.
///
/// A thread-safe index is initialized once. No external dictionary is needed.
/// Use [`WordSource::len`] for the actual vocabulary size when calculating
/// entropy. This single-language source retains the PR-2 vocabulary unchanged.
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
        words: index(include_str!("../data/german.txt"), Language::German),
    })
}

fn index(text: &'static str, language: Language) -> Vec<(&'static str, Language)> {
    text.lines().map(|word| (word, language.clone())).collect()
}

/// Returns the shared, embedded English word source, disjoint from German.
///
/// Modelled after: EFF <https://www.eff.org/dice> (curated passphrase vocabulary).
/// Data attribution, modifications, and source checksums are in the crate README.
pub fn english() -> &'static EmbeddedWordList {
    static ENGLISH: OnceLock<EmbeddedWordList> = OnceLock::new();
    ENGLISH.get_or_init(|| EmbeddedWordList {
        words: index(
            include_str!("../data/languages/english.txt"),
            Language::English,
        ),
    })
}

/// Returns the shared, embedded Latin word source, disjoint from German and English.
///
/// Modelled after: Whitaker's WORDS <https://github.com/mk270/whitakers-words/tree/1f2f0fb0867a896d7b9284a03d615ed635d6f992>.
/// See the crate README for the pinned data source and editorial selection.
pub fn latin() -> &'static EmbeddedWordList {
    static LATIN: OnceLock<EmbeddedWordList> = OnceLock::new();
    LATIN.get_or_init(|| EmbeddedWordList {
        words: index(include_str!("../data/languages/latin.txt"), Language::Latin),
    })
}

/// Returns the disjoint union of all three embedded languages.
///
/// Pass this source to the generator for independent language choice per word:
/// uniformly sampling `0..len()` samples every distinct spelling once. Do not
/// choose one language for an entire password or add a separate language bonus
/// to `log2(len())`. Equal list sizes give each language probability 1/3.
///
/// Entries are deterministically interleaved, retaining their language flags.
/// This order is public, carries no entropy, and needs no initialization RNG.
///
/// Modelled after: xkcdpass <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
/// Only union-before-sampling is modelled; no implementation code was copied.
///
/// ```
/// use kairos_core::WordSource;
/// let source: &dyn WordSource = kairos_wordlists::combined();
/// assert_eq!(source.len(), 21_402);
/// assert_eq!(source.word(1), kairos_wordlists::english().word(0));
/// ```
pub fn combined() -> &'static EmbeddedWordList {
    static COMBINED: OnceLock<EmbeddedWordList> = OnceLock::new();
    COMBINED.get_or_init(|| {
        let lists = [german(), english(), latin()];
        let mut words = Vec::with_capacity(lists.iter().map(|list| list.len()).sum());
        for index in 0..lists.iter().map(|list| list.len()).max().unwrap_or(0) {
            for list in lists {
                if let Some(entry) = list.words.get(index) {
                    words.push(entry.clone());
                }
            }
        }
        EmbeddedWordList { words }
    })
}
