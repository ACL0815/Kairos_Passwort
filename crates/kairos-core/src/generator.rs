//! OS randomness, source validation, and direct construction of the secret.
//!
//! Modelled after: rand <https://github.com/rust-random/rand/blob/0.9.5/src/distr/uniform_int.rs>.
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp>.
//! Modelled after: zeroize <https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/src/lib.rs>.

use std::borrow::Cow;
use std::collections::HashSet;

use rand::distr::{Distribution, Uniform};
use rand::{rngs::OsRng, RngCore, TryRngCore};
use zeroize::Zeroizing;

use crate::profiles::separators;
use crate::{
    profile_rules, CaseRule, GenerateError, GeneratedPassword, GeneratorConfig, ProfileRules,
    Segment, SegmentKind, WordSource,
};

struct PreparedSource<'a> {
    words: Vec<Cow<'a, str>>,
    min_inner: usize,
    min_initial_choices: usize,
    min_uppercase: usize,
    max_word_bytes: usize,
}

impl<'a> PreparedSource<'a> {
    fn new(source: &'a dyn WordSource, rules: &ProfileRules) -> Result<Self, GenerateError> {
        if source.is_empty() {
            return Err(GenerateError::EmptySource);
        }
        // These are public dictionary entries, never selected password copies.
        let mut words = Vec::with_capacity(source.len());
        let mut min_inner = usize::MAX;
        let mut min_initial_choices = 2;
        let mut min_uppercase = usize::MAX;
        let mut max_word_bytes = 0;
        for index in 0..source.len() {
            let original = source.word(index);
            let word = if original
                .chars()
                .flat_map(char::to_lowercase)
                .eq(original.chars())
            {
                Cow::Borrowed(original)
            } else {
                Cow::Owned(original.to_lowercase())
            };
            if word.is_empty() || !word.chars().all(|c| c.is_alphabetic() && !c.is_uppercase()) {
                return Err(GenerateError::InvalidConfig);
            }
            let len = word.chars().count();
            min_inner = min_inner.min(len.saturating_sub(1));
            let uppercase_count = word.chars().filter(|&c| uppercase(c).is_some()).count();
            min_uppercase = min_uppercase.min(uppercase_count);
            if word.chars().next().and_then(uppercase).is_none() {
                min_initial_choices = 1;
            }
            // At most one letter is uppercased per word; UTF-8 width may grow.
            let extra = word
                .chars()
                .filter_map(|c| uppercase(c).map(|u| u.len_utf8().saturating_sub(c.len_utf8())))
                .max()
                .unwrap_or(0);
            max_word_bytes = max_word_bytes.max(
                word.len()
                    .checked_add(extra)
                    .ok_or(GenerateError::InvalidConfig)?,
            );
            words.push(word);
        }
        let unique: HashSet<&str> = words.iter().map(|w| w.as_ref()).collect();
        if unique.len() != words.len()
            || (rules.insert_digit && min_inner == 0)
            || (rules.case == CaseRule::OneRandomUppercase && min_uppercase == 0)
        {
            return Err(GenerateError::InvalidConfig);
        }
        Ok(Self {
            words,
            min_inner,
            min_initial_choices,
            min_uppercase,
            max_word_bytes,
        })
    }

    fn entropy(&self, rules: &ProfileRules, pool_len: usize) -> f64 {
        entropy_bound(
            self.words.len(),
            rules,
            pool_len,
            self.min_inner,
            self.min_initial_choices,
            self.min_uppercase,
        )
    }
}

// Reversible one-scalar mappings keep distinct words and casing choices distinct.
// Unicode's default ß -> SS expansion is replaced by the single capital ẞ.
fn uppercase(c: char) -> Option<char> {
    if c == 'ß' {
        return Some('ẞ');
    }
    let mut upper = c.to_uppercase();
    let u = upper.next()?;
    if upper.next().is_some() || u == c || !u.is_uppercase() {
        return None;
    }
    let mut lower = u.to_lowercase();
    (lower.next() == Some(c) && lower.next().is_none()).then_some(u)
}

/// Returns a conservative output min-entropy bound from the real dictionary.
///
/// Words are lowercased and must remain distinct, nonempty, and alphabetic,
/// without uppercase scalars that have no lowercase mapping.
/// Otherwise returns [`GenerateError::InvalidConfig`]. Digit profiles require
/// at least two Unicode scalars in every word. Profile 11/12 requires at least
/// one reversible uppercase position per word. Empty sources return
/// [`GenerateError::EmptySource`]. Excluding all separators is invalid.
///
/// Uses the minimum inner-position count and minimum casing-choice count across
/// all words, so revealing a particular word's length cannot inflate the bound.
/// For equal-length fully caseable words the bound is exact. Words contain no
/// digits or separators; removing those and reversing casing uniquely recovers
/// the lowercase word sequence. There is no profile-selection bonus.
///
/// Modelled after: xkcdpass <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
pub fn entropy_bits_for(
    source: &dyn WordSource,
    config: &GeneratorConfig,
) -> Result<f64, GenerateError> {
    let pool = separators(config)?;
    let rules = profile_rules(config.profile);
    Ok(PreparedSource::new(source, rules)?.entropy(rules, pool.len()))
}

pub(crate) fn entropy_bound(
    source_len: usize,
    rules: &ProfileRules,
    pool_len: usize,
    min_inner: usize,
    initial_choices: usize,
    uppercase_choices: usize,
) -> f64 {
    let words = rules.word_count as f64;
    let mut bits = words * (source_len as f64).log2()
        + ((rules.word_count - 1) * rules.separators_per_gap) as f64 * (pool_len as f64).log2();
    if rules.insert_digit {
        bits += (10.0 * words * min_inner as f64).log2();
    }
    bits += words
        * match rules.case {
            CaseRule::Lowercase => 0.0,
            CaseRule::RandomInitial => (initial_choices as f64).log2(),
            CaseRule::OneRandomUppercase => (uppercase_choices as f64).log2(),
        };
    bits
}

/// Generates directly into one preallocated, zeroized secret buffer.
///
/// Uses [`OsRng`] for every draw and rand's unbiased [`Uniform`] distribution.
/// An OS error aborts with [`GenerateError::RandomSourceUnavailable`], including
/// errors after part of the secret was written. No alternative RNG is used.
/// Source validity and entropy follow [`entropy_bits_for`].
///
/// Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp>.
pub fn generate(
    source: &dyn WordSource,
    config: &GeneratorConfig,
) -> Result<GeneratedPassword, GenerateError> {
    generate_with_rng(source, config, &mut OsRng)
}

fn generate_with_rng<R: TryRngCore + ?Sized>(
    source: &dyn WordSource,
    config: &GeneratorConfig,
    rng: &mut R,
) -> Result<GeneratedPassword, GenerateError> {
    let pool = separators(config)?;
    let rules = profile_rules(config.profile);
    let prepared = PreparedSource::new(source, rules)?;
    let gaps = (rules.word_count - 1) * rules.separators_per_gap;
    let capacity = prepared
        .max_word_bytes
        .checked_mul(rules.word_count)
        .and_then(|n| n.checked_add(gaps + usize::from(rules.insert_digit)))
        .ok_or(GenerateError::InvalidConfig)?;
    let mut secret = Zeroizing::new(String::with_capacity(capacity));
    let mut segments = Vec::with_capacity(rules.word_count * 5 + gaps);
    let digit_word = if rules.insert_digit {
        Some(draw(rng, rules.word_count)?)
    } else {
        None
    };
    for word_index in 0..rules.word_count {
        if word_index > 0 {
            for _ in 0..rules.separators_per_gap {
                append(
                    &mut secret,
                    &mut segments,
                    pool[draw(rng, pool.len())?],
                    SegmentKind::Separator,
                );
            }
        }
        let word = &prepared.words[draw(rng, prepared.words.len())?];
        let digit_position = if digit_word == Some(word_index) {
            Some(1 + draw(rng, word.chars().count() - 1)?)
        } else {
            None
        };
        let upper_position = match rules.case {
            CaseRule::Lowercase => None,
            CaseRule::RandomInitial => (draw(rng, 2)? == 1).then_some(0),
            CaseRule::OneRandomUppercase => {
                let choice = draw(
                    rng,
                    word.chars().filter(|&c| uppercase(c).is_some()).count(),
                )?;
                word.chars()
                    .enumerate()
                    .filter(|(_, c)| uppercase(*c).is_some())
                    .nth(choice)
                    .map(|(i, _)| i)
            }
        };
        for (position, c) in word.chars().enumerate() {
            if digit_position == Some(position) {
                let digit = char::from(b'0' + draw(rng, 10)? as u8);
                append(&mut secret, &mut segments, digit, SegmentKind::Digit);
            }
            if let Some(u) = (upper_position == Some(position))
                .then(|| uppercase(c))
                .flatten()
            {
                append(&mut secret, &mut segments, u, SegmentKind::Uppercase);
            } else {
                append(&mut secret, &mut segments, c, SegmentKind::Word);
            }
        }
    }
    debug_assert!(secret.len() <= capacity);
    Ok(GeneratedPassword {
        secret,
        segments,
        entropy_bits: prepared.entropy(rules, pool.len()),
    })
}

fn append(secret: &mut String, segments: &mut Vec<Segment>, c: char, kind: SegmentKind) {
    let start = secret.len();
    secret.push(c);
    if let Some(last) = segments.last_mut().filter(|last| last.kind == kind) {
        last.range.end = secret.len();
    } else {
        segments.push(Segment {
            kind,
            range: start..secret.len(),
        });
    }
}

// rand's Uniform API is infallible. MAX makes its Lemire rejection loop finish
// after a source error: for range n, low(MAX * n) = 2^w - n >= 2^w % n.
// The sentinel result is always discarded. This adapter is private to draw().
struct FallibleUniform<'a, R: TryRngCore + ?Sized> {
    rng: &'a mut R,
    failed: bool,
}

impl<R: TryRngCore + ?Sized> RngCore for FallibleUniform<'_, R> {
    fn next_u32(&mut self) -> u32 {
        if self.failed {
            return u32::MAX;
        }
        self.rng.try_next_u32().unwrap_or_else(|_| {
            self.failed = true;
            u32::MAX
        })
    }

    fn next_u64(&mut self) -> u64 {
        if self.failed {
            return u64::MAX;
        }
        self.rng.try_next_u64().unwrap_or_else(|_| {
            self.failed = true;
            u64::MAX
        })
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if self.failed || self.rng.try_fill_bytes(dest).is_err() {
            self.failed = true;
            dest.fill(u8::MAX);
        }
    }
}

fn draw<R: TryRngCore + ?Sized>(rng: &mut R, upper: usize) -> Result<usize, GenerateError> {
    let distribution = Uniform::new(0, upper).map_err(|_| GenerateError::InvalidConfig)?;
    let mut adapter = FallibleUniform { rng, failed: false };
    let value = distribution.sample(&mut adapter);
    if adapter.failed {
        Err(GenerateError::RandomSourceUnavailable)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests;
