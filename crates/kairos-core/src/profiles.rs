//! Built-in rule data and effective separator pools.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp>.

use std::borrow::Cow;

use crate::{GenerateError, GeneratorConfig, ProfileId};

/// Letter casing applied independently to each word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CaseRule {
    Lowercase,
    RandomInitial,
    OneRandomUppercase,
}

/// Rule data shared by generation, entropy, and future custom profiles.
///
/// Built-ins satisfy: positive word/gap counts, and a nonempty, unique ASCII
/// punctuation pool. Future custom-rule constructors must validate these
/// invariants before passing rules to generation or entropy calculations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ProfileRules {
    pub word_count: usize,
    /// Built-ins borrow static data; future custom rules may own a runtime pool.
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
    let pool: Vec<_> = profile_rules(config.profile)
        .separator_pool
        .chars()
        .filter(|c| !config.excluded_chars.contains(c))
        .collect();
    if pool.is_empty() {
        Err(GenerateError::InvalidConfig)
    } else {
        Ok(pool)
    }
}
