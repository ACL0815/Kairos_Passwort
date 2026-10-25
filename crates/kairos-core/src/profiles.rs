//! Built-in rules, validated custom profiles, and opt-in rotation settings.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp>.
//! Modelled after: pwgen-go <https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/config/config.go>.
//! Modelled after: Bitwarden <https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/libs/tools/generator/core/src/data/default-passphrase-generation-options.ts>.
//! Modelled after: Serde <https://github.com/serde-rs/serde/tree/v1.0.229>.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{GenerateError, GeneratorConfig, ProfileId};

/// Letter casing applied independently to each word.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CaseRule {
    Lowercase,
    RandomInitial,
    OneRandomUppercase,
}

/// Rule data shared by generation, entropy, and future custom profiles.
///
/// Raw, editable rule data. Use [`CustomProfile`] for validated generation and
/// versioned persistence; this internal rule shape is not the storage schema.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ProfileRules {
    pub word_count: usize,
    /// Built-ins borrow static data; custom rules may own a runtime pool.
    pub separator_pool: Cow<'static, str>,
    pub separators_per_gap: usize,
    pub insert_digit: bool,
    pub case: CaseRule,
    pub recommended_source_len: usize,
}

const ALL_SEPARATORS: &str = "-+.,#*!?_:;/()\"'$%&=";

const BASE: ProfileRules = ProfileRules {
    word_count: 4,
    separator_pool: Cow::Borrowed("-"),
    separators_per_gap: 1,
    insert_digit: false,
    case: CaseRule::Lowercase,
    recommended_source_len: 7_000,
};

const LARGE: ProfileRules = ProfileRules {
    recommended_source_len: 21_000,
    ..BASE
};
const DIGIT: ProfileRules = ProfileRules {
    insert_digit: true,
    ..LARGE
};
const INITIAL: ProfileRules = ProfileRules {
    separator_pool: Cow::Borrowed(ALL_SEPARATORS),
    case: CaseRule::RandomInitial,
    ..DIGIT
};
const SIX: ProfileRules = ProfileRules {
    word_count: 6,
    ..INITIAL
};

static RULES: [ProfileRules; 13] = [
    BASE,
    LARGE,
    ProfileRules {
        separator_pool: Cow::Borrowed("-+.,#"),
        ..LARGE
    },
    ProfileRules {
        separator_pool: Cow::Borrowed("-+.,#"),
        ..DIGIT
    },
    ProfileRules {
        separator_pool: Cow::Borrowed("-+.,#*!?"),
        ..DIGIT
    },
    ProfileRules {
        separator_pool: Cow::Borrowed("-+.,#*!?_:;/"),
        ..DIGIT
    },
    ProfileRules {
        separator_pool: Cow::Borrowed("-+.,#*!?_:;/()\"'"),
        ..DIGIT
    },
    ProfileRules {
        separator_pool: Cow::Borrowed(ALL_SEPARATORS),
        ..DIGIT
    },
    INITIAL,
    ProfileRules {
        word_count: 5,
        ..INITIAL
    },
    SIX,
    ProfileRules {
        case: CaseRule::OneRandomUppercase,
        ..SIX
    },
    ProfileRules {
        separators_per_gap: 2,
        case: CaseRule::OneRandomUppercase,
        ..SIX
    },
];

/// Returns the cumulative built-in rules without changing the shared config.
pub const fn profile_rules(profile: ProfileId) -> &'static ProfileRules {
    &RULES[profile.get() as usize]
}

/// Advisory metadata; a small source remains usable and contributes its real size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProfileWarning {
    SourceBelowRecommendation { actual: usize, recommended: usize },
}

/// Reports a source below the profile's recommendation, including profile 1.
pub fn profile_warnings(source_len: usize, config: &GeneratorConfig) -> Vec<ProfileWarning> {
    let recommended = profile_rules(config.profile).recommended_source_len;
    warnings_for(source_len, recommended)
}

fn warnings_for(source_len: usize, recommended: usize) -> Vec<ProfileWarning> {
    if source_len < recommended {
        vec![ProfileWarning::SourceBelowRecommendation {
            actual: source_len,
            recommended,
        }]
    } else {
        Vec::new()
    }
}

pub(crate) fn separators(config: &GeneratorConfig) -> Result<Vec<char>, GenerateError> {
    separators_for(profile_rules(config.profile), &config.excluded_chars)
}

pub(crate) fn separators_for(
    rules: &ProfileRules,
    excluded_chars: &[char],
) -> Result<Vec<char>, GenerateError> {
    let pool: Vec<_> = rules
        .separator_pool
        .chars()
        .filter(|c| !excluded_chars.contains(c))
        .collect();
    if pool.is_empty() {
        Err(GenerateError::InvalidConfig)
    } else {
        Ok(pool)
    }
}

/// Validated custom rules, serializable without generated secrets.
///
/// Limits are 1..=64 words, 1..=8 separators per gap, and a nonempty pool of
/// distinct ASCII punctuation. Rules are validated after deserialization;
/// callers must bound the input document size before parsing.
/// A one-word profile still requires a valid pool, even though it emits no gaps.
/// Source-dependent requirements are checked by [`crate::generate_custom`].
///
/// ```
/// use kairos_core::{CaseRule, CustomProfile, ProfileId, ProfileRotation};
/// let own = CustomProfile::new(5, "-+.,", 1, true, CaseRule::RandomInitial)?;
/// let rotation = ProfileRotation::new(vec![ProfileId::new(0)?.into(), own])?;
/// let json = serde_json::to_string(&rotation)?;
/// let restored: ProfileRotation = serde_json::from_str(&json)?;
/// assert_eq!(rotation, restored);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "CustomProfileV1", into = "CustomProfileV1")]
pub struct CustomProfile(ProfileRules);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CustomProfileV1 {
    version: u8,
    word_count: usize,
    separator_pool: String,
    separators_per_gap: usize,
    insert_digit: bool,
    case: CaseRule,
    #[serde(default = "default_recommendation")]
    recommended_source_len: usize,
}

fn default_recommendation() -> usize {
    1
}

impl TryFrom<CustomProfileV1> for CustomProfile {
    type Error = GenerateError;

    fn try_from(data: CustomProfileV1) -> Result<Self, Self::Error> {
        if data.version != 1 {
            return Err(GenerateError::InvalidConfig);
        }
        Self::try_from(ProfileRules {
            word_count: data.word_count,
            separator_pool: Cow::Owned(data.separator_pool),
            separators_per_gap: data.separators_per_gap,
            insert_digit: data.insert_digit,
            case: data.case,
            recommended_source_len: data.recommended_source_len,
        })
    }
}

impl From<CustomProfile> for CustomProfileV1 {
    fn from(profile: CustomProfile) -> Self {
        let rules = profile.0;
        Self {
            version: 1,
            word_count: rules.word_count,
            separator_pool: rules.separator_pool.into_owned(),
            separators_per_gap: rules.separators_per_gap,
            insert_digit: rules.insert_digit,
            case: rules.case,
            recommended_source_len: rules.recommended_source_len,
        }
    }
}

impl CustomProfile {
    /// Builds validated rules with no minimum dictionary-size recommendation.
    pub fn new(
        word_count: usize,
        separator_pool: impl Into<String>,
        separators_per_gap: usize,
        insert_digit: bool,
        case: CaseRule,
    ) -> Result<Self, GenerateError> {
        Self::try_from(ProfileRules {
            word_count,
            separator_pool: Cow::Owned(separator_pool.into()),
            separators_per_gap,
            insert_digit,
            case,
            recommended_source_len: 1,
        })
    }

    /// Borrows validated rules. Edit a clone and revalidate to change a profile.
    pub fn rules(&self) -> &ProfileRules {
        &self.0
    }

    /// Reports an unmet dictionary-size recommendation without changing entropy.
    pub fn warnings(&self, source_len: usize) -> Vec<ProfileWarning> {
        warnings_for(source_len, self.0.recommended_source_len)
    }
}

impl TryFrom<ProfileRules> for CustomProfile {
    type Error = GenerateError;

    fn try_from(rules: ProfileRules) -> Result<Self, Self::Error> {
        let pool = rules.separator_pool.as_bytes();
        if !(1..=64).contains(&rules.word_count)
            || !(1..=8).contains(&rules.separators_per_gap)
            || pool.is_empty()
            || !pool.iter().all(u8::is_ascii_punctuation)
            || pool.iter().enumerate().any(|(i, c)| pool[..i].contains(c))
        {
            return Err(GenerateError::InvalidConfig);
        }
        Ok(Self(rules))
    }
}

impl From<ProfileId> for CustomProfile {
    fn from(id: ProfileId) -> Self {
        Self(profile_rules(id).clone())
    }
}

impl From<CustomProfile> for ProfileRules {
    fn from(profile: CustomProfile) -> Self {
        profile.0
    }
}

/// An opt-in list of 1..=16 equally likely profile entries.
///
/// Each generation draws afresh with the system CSPRNG. Duplicate or overlapping
/// entries are allowed, but cannot receive the disjoint-profile bonus.
/// JSON represents this type as an array of validated custom profiles.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Vec<CustomProfile>", into = "Vec<CustomProfile>")]
pub struct ProfileRotation(Vec<CustomProfile>);

impl ProfileRotation {
    /// Validates the number of entries. Built-ins can be added with `.into()`.
    pub fn new(profiles: Vec<CustomProfile>) -> Result<Self, GenerateError> {
        if profiles.is_empty() || profiles.len() > 16 {
            return Err(GenerateError::InvalidConfig);
        }
        Ok(Self(profiles))
    }

    /// Returns configuration entries, never the profile selected for a password.
    pub fn profiles(&self) -> &[CustomProfile] {
        &self.0
    }
}

impl TryFrom<Vec<CustomProfile>> for ProfileRotation {
    type Error = GenerateError;

    fn try_from(profiles: Vec<CustomProfile>) -> Result<Self, Self::Error> {
        Self::new(profiles)
    }
}

impl From<ProfileRotation> for Vec<CustomProfile> {
    fn from(rotation: ProfileRotation) -> Self {
        rotation.0
    }
}

/// Whether the attacker knows the profile selected for the current password.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProfileKnowledge {
    /// Only the configured list and algorithm are known, not the current draw.
    Unknown,
    /// The current profile is exposed; no profile-selection bonus is claimed.
    Known,
}

/// Conservative mixture min-entropy, not average profile or Shannon entropy.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct RotationEntropy {
    pub entropy_bits: f64,
    /// GUI checkbox value; zero unless the disjointness proof succeeds and the
    /// current profile is unknown, with equal conservative profile strengths.
    /// Never exceeds `log2(number of entries)` or the concept's four-bit cap.
    pub rotation_bonus_bits: f64,
}

/// A rotated password without the chosen profile identifier or rule data.
///
/// No `Debug`, `Clone`, or serialization implementation exposes the secret.
/// Segments and the password itself still reveal structure; do not log either.
///
/// ```compile_fail
/// fn log_password(password: &kairos_core::RotatedPassword) {
///     println!("{password:?}");
/// }
/// ```
///
/// ```compile_fail
/// fn copy_password(password: kairos_core::RotatedPassword) {
///     let _copy = password.clone();
/// }
/// ```
#[non_exhaustive]
pub struct RotatedPassword {
    /// Its entropy is the whole rotation's conservative bound, independent of
    /// the particular profile drawn for this password.
    pub password: crate::GeneratedPassword,
    pub rotation_bonus_bits: f64,
}
