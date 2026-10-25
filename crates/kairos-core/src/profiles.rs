//! Built-in rule data and effective separator pools.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp>.

use crate::{GenerateError, GeneratorConfig, ProfileId};

/// Letter casing applied independently to each word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaseRule {
    Lowercase,
    RandomInitial,
    OneRandomUppercase,
}

/// Rule data shared by generation, entropy, and future custom profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProfileRules {
    pub word_count: usize,
    pub separator_pool: &'static str,
    pub separators_per_gap: usize,
    pub insert_digit: bool,
    pub case: CaseRule,
    pub recommended_source_len: usize,
}

const ALL_SEPARATORS: &str = "-+.,#*!?_:;/()\"'$%&=";

const BASE: ProfileRules = ProfileRules {
    word_count: 4,
    separator_pool: "-",
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
    separator_pool: ALL_SEPARATORS,
    case: CaseRule::RandomInitial,
    ..DIGIT
};
const SIX: ProfileRules = ProfileRules {
    word_count: 6,
    ..INITIAL
};

const RULES: [ProfileRules; 13] = [
    BASE,
    LARGE,
    ProfileRules {
        separator_pool: "-+.,#",
        ..LARGE
    },
    ProfileRules {
        separator_pool: "-+.,#",
        ..DIGIT
    },
    ProfileRules {
        separator_pool: "-+.,#*!?",
        ..DIGIT
    },
    ProfileRules {
        separator_pool: "-+.,#*!?_:;/",
        ..DIGIT
    },
    ProfileRules {
        separator_pool: "-+.,#*!?_:;/()\"'",
        ..DIGIT
    },
    ProfileRules {
        separator_pool: ALL_SEPARATORS,
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
