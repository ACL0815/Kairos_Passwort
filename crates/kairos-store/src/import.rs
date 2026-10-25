//! Text word-list parsing with explicit rejection counts.
//!
//! Modelled after: xkcdpass
//! <https://github.com/redacted/XKCD-password-generator/blob/9db439542d9a156f8a9db56323b459670175c132/xkcdpass/xkcd_password.py>.
//! Modelled after: diceware
//! <https://github.com/ulif/diceware/blob/98fa09afd7ab28b6075d771b2c44b0e00479e72f/diceware/wordlist.py>.

use std::collections::HashSet;

use zeroize::Zeroizing;

use crate::StoreError;

/// Rules applied after Unicode lowercase conversion, without transliteration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportRules {
    pub min_length: usize,
    pub max_length: usize,
    /// Each character in a normalized word must occur in this string.
    pub allowed_characters: String,
}

impl Default for ImportRules {
    fn default() -> Self {
        Self {
            min_length: 3,
            max_length: 9,
            allowed_characters: "abcdefghijklmnopqrstuvwxyz".into(),
        }
    }
}

/// Every input line has exactly one outcome; no rejected word is retained.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImportReport {
    pub accepted: usize,
    pub blank: usize,
    pub comment: usize,
    pub invalid_format: usize,
    pub invalid_characters: usize,
    pub too_short: usize,
    pub too_long: usize,
    pub duplicate: usize,
}

impl ImportReport {
    pub fn rejected(&self) -> usize {
        self.blank
            + self.comment
            + self.invalid_format
            + self.invalid_characters
            + self.too_short
            + self.too_long
            + self.duplicate
    }
}

pub(crate) fn parse(
    source: &[u8],
    rules: &ImportRules,
) -> Result<(Zeroizing<Vec<String>>, ImportReport), StoreError> {
    if rules.min_length == 0
        || rules.min_length > rules.max_length
        || rules.allowed_characters.is_empty()
        || rules
            .allowed_characters
            .chars()
            .any(|c| c.is_whitespace() || c.is_control())
    {
        return Err(StoreError::InvalidRules);
    }
    let text = std::str::from_utf8(source).map_err(|_| StoreError::InvalidEncoding)?;
    let mut report = ImportReport::default();
    let mut unique = HashSet::new();
    for line in text.trim_start_matches('\u{feff}').lines() {
        let line = line.trim();
        if line.is_empty() {
            report.blank += 1;
            continue;
        }
        if line.starts_with('#') {
            report.comment += 1;
            continue;
        }
        let mut fields = line.split_whitespace();
        let first = fields.next().expect("nonempty trimmed line");
        let candidate = match (fields.next(), fields.next()) {
            (None, None) => first,
            (Some(word), None)
                if (4..=6).contains(&first.len())
                    && first.bytes().all(|digit| (b'1'..=b'6').contains(&digit)) =>
            {
                word
            }
            _ => {
                report.invalid_format += 1;
                continue;
            }
        };
        let word = candidate.to_lowercase();
        if !word.chars().all(|c| rules.allowed_characters.contains(c)) {
            report.invalid_characters += 1;
        } else if word.chars().count() < rules.min_length {
            report.too_short += 1;
        } else if word.chars().count() > rules.max_length {
            report.too_long += 1;
        } else if !unique.insert(word) {
            report.duplicate += 1;
        } else {
            report.accepted += 1;
        }
    }
    Ok((Zeroizing::new(unique.into_iter().collect()), report))
}
