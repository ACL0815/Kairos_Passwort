//! Shared contract for word sources, generation profiles, and generated passwords.
//!
//! Generation and entropy calculations are reserved for PR-1.

use std::fmt;
use std::ops::Range;

use zeroize::Zeroizing;

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    pub kind: SegmentKind,
    pub range: Range<usize>,
}

/// A generated secret and the metadata needed to display it.
///
/// This type deliberately has no `Debug` or `Clone` implementation. Zeroizing
/// the owned string cannot erase copies made by the UI or clipboard.
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
    RandomSourceUnavailable,
}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NotImplemented => "password generation is not implemented yet",
            Self::EmptySource => "the word source is empty",
            Self::InvalidConfig => "the generation configuration is invalid",
            Self::RandomSourceUnavailable => "the system random source is unavailable",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for GenerateError {}

/// Generates a password from an indexed word source and validated settings.
///
/// The scaffold always returns `GenerateError::NotImplemented`.
pub fn generate(
    _source: &dyn WordSource,
    _config: &GeneratorConfig,
) -> Result<GeneratedPassword, GenerateError> {
    Err(GenerateError::NotImplemented)
}

/// Estimates entropy from the actual source size and effective profile choices.
///
/// # Panics
///
/// This scaffold stub always panics until PR-1 implements the calculation.
pub fn entropy_bits(_source_len: usize, _config: &GeneratorConfig) -> f64 {
    todo!("implement profile-aware entropy in PR-1")
}
