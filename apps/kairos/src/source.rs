//! A borrowed embedded source fits the same app boundary as an owned store snapshot.
//!
//! Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist>.

use kairos_core::{Language, WordSource};

struct BorrowedSource(&'static dyn WordSource);

impl WordSource for BorrowedSource {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn word(&self, index: usize) -> &str {
        self.0.word(index)
    }
    fn language(&self, index: usize) -> Language {
        self.0.language(index)
    }
}

/// Default source. `KairosApp::new` also accepts any owned `WordSource` snapshot.
pub fn german_source() -> Box<dyn WordSource> {
    Box::new(BorrowedSource(kairos_wordlists::german()))
}
