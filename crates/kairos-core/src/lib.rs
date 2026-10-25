//! Word sources, data-driven profiles, OS-backed generation, and entropy.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp>.
//! Modelled after: zeroize <https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/src/lib.rs>.
//!
//! See the crate README for source validation, Unicode casing, and entropy bounds.

use std::fmt;
use std::ops::Range;

use zeroize::Zeroizing;

mod generator;
mod profiles;

pub use generator::{
    entropy_bits_for, entropy_bits_for_custom, entropy_bits_for_rotation, generate,
    generate_custom, generate_rotating,
};
pub use profiles::{
    profile_rules, profile_warnings, CaseRule, CustomProfile, ProfileKnowledge, ProfileRotation,
    ProfileRules, ProfileWarning, RotatedPassword, RotationEntropy,
};

/// Language metadata attached to an individual source word.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Language {
    German,
    English,
    Latin,
    /// An additional language, preferably identified by a BCP 47 tag.
    Other(String),
}

/// An indexed word source with stable contents for the duration of generation.
///
/// Entries must be nonempty, distinct words. Valid indices are `0..self.len()`.
/// Implementations may panic when queried with an out-of-bounds index.
pub trait WordSource {
    /// Returns the number of available words.
    fn len(&self) -> usize;

    /// Returns the word at the given index.
    fn word(&self, index: usize) -> &str;

    /// Returns the language of the word at the given index.
    fn language(&self, index: usize) -> Language;

    /// Returns whether this source contains no words.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A validated built-in generation profile identifier in `0..=12`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ProfileId(u8);

impl ProfileId {
    /// Validates a built-in profile identifier.
    pub const fn new(value: u8) -> Result<Self, InvalidProfileId> {
        if value <= 12 {
            Ok(Self(value))
        } else {
            Err(InvalidProfileId(value))
        }
    }

    /// Returns the validated numeric identifier.
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for ProfileId {
    type Error = InvalidProfileId;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// An identifier outside the built-in profile range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidProfileId(pub u8);

impl fmt::Display for InvalidProfileId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "profile {} is outside 0..=12", self.0)
    }
}

impl std::error::Error for InvalidProfileId {}

/// Generation settings shared by the core, GUI, and word source consumers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GeneratorConfig {
    /// Built-in profile, defaulting to profile 0.
    pub profile: ProfileId,
    /// Separator characters to exclude; duplicates have no additional effect.
    pub excluded_chars: Vec<char>,
}

/// Semantic role of a highlighted password segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentKind {
    Word,
    Separator,
    Digit,
    Uppercase,
}

/// A highlighted UTF-8 byte range within the generated secret.
///
/// Ranges use valid character boundaries, are ordered, and do not overlap.
/// Uppercase letters and inserted digits split surrounding word segments.
/// Even without secret bytes, logging segments discloses lengths and positions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    pub kind: SegmentKind,
    pub range: Range<usize>,
}

/// A generated secret and the metadata needed to display it.
///
/// This type deliberately has no `Debug` or `Clone` implementation. Zeroizing
/// the owned string cannot erase copies made by the UI or clipboard.
/// Its public `secret` field must never be logged: `Zeroizing<String>` itself
/// implements `Debug` and does not redact its contents.
///
/// ```compile_fail
/// fn log_secret(password: &kairos_core::GeneratedPassword) {
///     println!("{password:?}");
/// }
/// ```
///
/// ```compile_fail
/// fn copy_secret(password: kairos_core::GeneratedPassword) {
///     let _copy = password.clone();
/// }
/// ```
pub struct GeneratedPassword {
    pub secret: Zeroizing<String>,
    pub segments: Vec<Segment>,
    pub entropy_bits: f64,
}

/// A generation failure without password contents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GenerateError {
    NotImplemented,
    EmptySource,
    InvalidConfig,
    /// The dictionary cannot produce unambiguous output under this profile.
    InvalidSource,
    RandomSourceUnavailable,
}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NotImplemented => "password generation is not implemented yet",
            Self::EmptySource => "the word source is empty",
            Self::InvalidConfig => "the generation configuration is invalid",
            Self::InvalidSource => "the word source is invalid for this profile",
            Self::RandomSourceUnavailable => "the system random source is unavailable",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for GenerateError {}

/// Estimates entropy from the actual source size and effective profile choices.
///
/// Assumes distinct, lowercase six-letter words with reversible single-letter
/// uppercase mappings. A digit has five inner insertion positions per word.
/// This model is not a lower bound for arbitrary words. Use [`entropy_bits_for`]
/// for an honest, source-dependent bound, as [`generate`] does.
/// This concept model must not be used for a password strength display; display
/// [`GeneratedPassword::entropy_bits`] or the result of [`entropy_bits_for`].
/// Empty sources or invalid separator exclusions return `0.0`.
///
/// Modelled after: xkcdpass <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
pub fn entropy_bits(source_len: usize, config: &GeneratorConfig) -> f64 {
    if source_len == 0 {
        return 0.0;
    }
    let Ok(separators) = profiles::separators(config) else {
        return 0.0;
    };
    generator::entropy_bound(
        source_len,
        profile_rules(config.profile),
        separators.len(),
        5,
        2,
        6,
    )
}
