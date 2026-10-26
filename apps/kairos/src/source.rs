//! A borrowed embedded source fits the same app boundary as an owned store snapshot.
//!
//! Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist>.

use kairos_core::{Language, WordSource};
use std::collections::BTreeSet;

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

pub fn combined_source() -> Box<dyn WordSource> {
    Box::new(BorrowedSource(kairos_wordlists::combined()))
}

pub fn uses_german(profile: kairos_core::ProfileId, german_only: bool) -> bool {
    german_only || profile.get() == 0
}

/// Stable index view; filtering changes both sampling and the entropy input.
///
/// Modelled after: xkcdpass filtering before vocabulary-size calculation.
/// <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
pub struct FilteredSource<'a> {
    source: &'a dyn WordSource,
    indices: Vec<usize>,
}

impl<'a> FilteredSource<'a> {
    pub fn new(source: &'a dyn WordSource, blocked: &BTreeSet<String>) -> Self {
        Self {
            source,
            indices: (0..source.len())
                .filter(|&index| {
                    let word = source.word(index);
                    if word.bytes().any(|byte| byte.is_ascii_uppercase()) {
                        !blocked.contains(&word.to_ascii_lowercase())
                    } else {
                        !blocked.contains(word)
                    }
                })
                .collect(),
        }
    }
}

impl WordSource for FilteredSource<'_> {
    fn len(&self) -> usize {
        self.indices.len()
    }
    fn word(&self, index: usize) -> &str {
        self.source.word(self.indices[index])
    }
    fn language(&self, index: usize) -> Language {
        self.source.language(self.indices[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kairos_core::{GeneratorConfig, ProfileId};

    #[test]
    fn profile_zero_is_german_and_later_profiles_expand_unless_overridden() {
        for id in 0..=12 {
            let profile = ProfileId::new(id).unwrap();
            assert_eq!(uses_german(profile, false), id == 0);
            assert!(uses_german(profile, true));
        }
        assert_eq!(german_source().len(), 6907);
        assert_eq!(combined_source().len(), 20697);
    }

    #[test]
    fn filtered_indices_preserve_languages_and_entropy_uses_retained_size() {
        let base = combined_source();
        let blocked = (0..base.len() - 2)
            .map(|i| base.word(i).to_owned())
            .collect();
        let filtered = FilteredSource::new(base.as_ref(), &blocked);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered.word(0), base.word(base.len() - 2));
        assert_eq!(filtered.language(1), base.language(base.len() - 1));
        let config = GeneratorConfig::default();
        for _ in 0..8 {
            let result = kairos_core::generate(&filtered, &config).unwrap();
            assert_eq!(result.entropy_bits, 4.0);
            assert!(result.secret.split('-').all(|word| !blocked.contains(word)));
        }
        let all = (0..base.len()).map(|i| base.word(i).to_owned()).collect();
        assert!(matches!(
            kairos_core::generate(&FilteredSource::new(base.as_ref(), &all), &config),
            Err(kairos_core::GenerateError::EmptySource)
        ));
    }
}
