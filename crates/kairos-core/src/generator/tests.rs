use super::*;
use crate::{entropy_bits, profile_warnings, Language, ProfileId, ProfileWarning};
use rand::{rngs::StdRng, SeedableRng};

struct TestSource(Vec<String>);

impl TestSource {
    fn new(words: &[&str]) -> Self {
        Self(words.iter().map(|w| (*w).to_owned()).collect())
    }

    fn six_letters(len: usize) -> Self {
        Self(
            (0..len)
                .map(|mut index| {
                    let mut word = [b'a'; 6];
                    for c in &mut word {
                        *c += (index % 26) as u8;
                        index /= 26;
                    }
                    String::from_utf8(word.to_vec()).unwrap()
                })
                .collect(),
        )
    }
}

impl WordSource for TestSource {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn word(&self, index: usize) -> &str {
        &self.0[index]
    }
    fn language(&self, _: usize) -> Language {
        Language::German
    }
}

fn config(profile: u8) -> GeneratorConfig {
    GeneratorConfig {
        profile: ProfileId::new(profile).unwrap(),
        excluded_chars: Vec::new(),
    }
}

fn seeded(source: &dyn WordSource, config: &GeneratorConfig, seed: u64) -> GeneratedPassword {
    generate_with_rng(source, config, &mut StdRng::seed_from_u64(seed)).unwrap()
}

fn assert_error(source: &dyn WordSource, cfg: &GeneratorConfig, expected: GenerateError) {
    assert!(matches!(generate(source, cfg), Err(e) if e == expected));
    assert_eq!(entropy_bits_for(source, cfg), Err(expected));
}

#[test]
fn concept_table_matches_model_and_real_six_letter_sources() {
    let expected = [
        51.1, 57.4, 64.4, 72.0, 74.1, 75.8, 77.1, 78.0, 82.0, 102.0, 122.0, 131.5, 153.1,
    ];
    for (profile, expected) in expected.into_iter().enumerate() {
        let len = if profile == 0 { 7_000 } else { 21_000 };
        let source = TestSource::six_letters(len);
        let cfg = config(profile as u8);
        let model = entropy_bits(len, &cfg);
        let actual = entropy_bits_for(&source, &cfg).unwrap();
        assert!((model - expected).abs() < 0.1, "profile {profile}: {model}");
        assert!((model - actual).abs() < 1e-10);
        assert!((seeded(&source, &cfg, 17).entropy_bits - actual).abs() < 1e-10);
    }
}

#[test]
fn cumulative_profile_rules_and_output_shapes() {
    let source = TestSource::new(&["abcdef"]);
    let pool_lengths = [1, 1, 5, 5, 8, 12, 16, 20, 20, 20, 20, 20, 20];
    for profile in 0..=12 {
        let cfg = config(profile);
        let rules = profile_rules(cfg.profile);
        assert_eq!(
            rules.separator_pool.chars().count(),
            pool_lengths[profile as usize]
        );
        let count = match profile {
            0..=8 => 4,
            9 => 5,
            _ => 6,
        };
        assert_eq!(rules.word_count, count);
        for seed in 0..20 {
            let result = seeded(&source, &cfg, seed);
            let text = result.secret.as_str();
            let gaps = if profile == 12 { 2 } else { 1 };
            assert_eq!(
                text.chars().filter(|c| !c.is_alphanumeric()).count(),
                (count - 1) * gaps
            );
            assert_eq!(
                text.chars().filter(char::is_ascii_digit).count(),
                usize::from(profile >= 3)
            );
            assert_eq!(
                text.chars().filter(|c| c.is_alphabetic()).count(),
                count * 6
            );
            let words: Vec<_> = text
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| !w.is_empty())
                .collect();
            assert_eq!(words.len(), count);
            for word in words {
                assert!(word.starts_with(|c: char| c.is_alphabetic()));
                assert!(word.ends_with(|c: char| c.is_alphabetic()));
                let uppercase_count = word.chars().filter(|c| c.is_uppercase()).count();
                match profile {
                    0..=7 => assert_eq!(uppercase_count, 0),
                    8..=10 => {
                        assert!(uppercase_count <= 1);
                        assert!(word.chars().skip(1).all(|c| !c.is_uppercase()));
                    }
                    _ => assert_eq!(uppercase_count, 1),
                }
            }
        }
    }
}

#[test]
fn profile_one_uses_real_size_and_advises_on_short_lists() {
    assert_eq!(
        entropy_bits(7_000, &config(0)),
        entropy_bits(7_000, &config(1))
    );
    assert_eq!(
        profile_warnings(7_000, &config(1)),
        vec![ProfileWarning::SourceBelowRecommendation {
            actual: 7_000,
            recommended: 21_000,
        }]
    );
    assert!(profile_warnings(21_000, &config(1)).is_empty());
    assert!(profile_warnings(7_000, &config(0)).is_empty());
}

#[test]
fn excludes_separators_and_recalculates_entropy() {
    let source = TestSource::new(&["abcdef"]);
    let mut cfg = config(2);
    cfg.excluded_chars = vec!['+', '.', ',', '#', '#', 'a'];
    assert_eq!(entropy_bits(21_000, &cfg), entropy_bits(21_000, &config(1)));
    for seed in 0..50 {
        let generated = seeded(&source, &cfg, seed);
        assert_eq!(generated.secret.as_str(), "abcdef-abcdef-abcdef-abcdef");
        assert_eq!(generated.entropy_bits, 0.0);
    }
    let mut cfg = config(12);
    cfg.excluded_chars = vec!['-', '+'];
    let difference = entropy_bits(21_000, &config(12)) - entropy_bits(21_000, &cfg);
    assert!((difference - 10.0 * (20.0_f64 / 18.0).log2()).abs() < 1e-10);
}

#[test]
fn empty_pool_and_excluded_fixed_separator_are_invalid() {
    let source = TestSource::new(&["abcdef"]);
    for profile in 0..=12 {
        let mut cfg = config(profile);
        cfg.excluded_chars = profile_rules(cfg.profile).separator_pool.chars().collect();
        assert_error(&source, &cfg, GenerateError::InvalidConfig);
        assert_eq!(entropy_bits(21_000, &cfg), 0.0);
    }
}

#[test]
fn empty_source_has_no_entropy_and_returns_error() {
    assert_error(
        &TestSource::new(&[]),
        &config(0),
        GenerateError::EmptySource,
    );
    assert_eq!(entropy_bits(0, &config(12)), 0.0);
}

#[test]
fn rejects_empty_non_alphabetic_and_lowercase_collisions() {
    for words in [
        &[""][..],
        &["a-b"],
        &["a1b"],
        &["a b"],
        &["haus", "Haus"],
        &["haus", "haus"],
        &["İgel"],
        &["ϒ"],
    ] {
        assert_error(
            &TestSource::new(words),
            &config(0),
            GenerateError::InvalidConfig,
        );
    }
}

#[test]
fn short_words_only_fail_when_an_inner_digit_position_is_required() {
    let source = TestSource::new(&["a"]);
    assert!(generate(&source, &config(2)).is_ok());
    assert_error(&source, &config(3), GenerateError::InvalidConfig);
}

#[test]
fn variable_lengths_report_conservative_entropy() {
    let source = TestSource::new(&["ab", "abcdef"]);
    let cfg = config(11);
    let bits = entropy_bits_for(&source, &cfg).unwrap();
    let expected = 6.0 + 5.0 * 20.0_f64.log2() + 60.0_f64.log2() + 6.0;
    assert!((bits - expected).abs() < 1e-10);
    assert!(bits < entropy_bits(2, &cfg));
    for seed in 0..50 {
        assert_eq!(seeded(&source, &cfg, seed).entropy_bits, bits);
    }
}

#[test]
fn uncased_letters_do_not_gain_fictitious_entropy() {
    let source = TestSource::new(&["中文"]);
    assert_eq!(
        entropy_bits_for(&source, &config(8)),
        entropy_bits_for(&source, &config(7))
    );
    assert_error(&source, &config(11), GenerateError::InvalidConfig);
}

#[test]
fn normalizes_dictionary_case_before_generation() {
    let source = TestSource::new(&["Äpfel"]);
    assert_eq!(
        seeded(&source, &config(0), 1).secret.as_str(),
        "äpfel-äpfel-äpfel-äpfel"
    );
}

#[test]
fn unicode_casing_is_reversible_and_keeps_exactly_one_uppercase() {
    let source = TestSource::new(&["ßß", "äö", "ȿɀ"]);
    for profile in [11, 12] {
        for seed in 0..100 {
            let result = seeded(&source, &config(profile), seed);
            assert_eq!(
                result.secret.chars().filter(|c| c.is_uppercase()).count(),
                6
            );
            assert!(!result.secret.contains("SS"));
            assert_segments(&result);
        }
    }
    assert_eq!(uppercase('ß'), Some('ẞ'));
    assert_eq!(uppercase('ﬀ'), None);
    assert_eq!(uppercase('ς'), None);
}

fn assert_segments(result: &GeneratedPassword) {
    let mut end = 0;
    for segment in &result.segments {
        assert_eq!(segment.range.start, end);
        assert!(segment.range.end > segment.range.start);
        assert!(result.secret.is_char_boundary(segment.range.start));
        assert!(result.secret.is_char_boundary(segment.range.end));
        let text = &result.secret[segment.range.clone()];
        assert!(text.chars().all(|c| match segment.kind {
            SegmentKind::Word => c.is_alphabetic() && !c.is_uppercase(),
            SegmentKind::Separator => !c.is_alphanumeric(),
            SegmentKind::Digit => c.is_ascii_digit(),
            SegmentKind::Uppercase => c.is_uppercase(),
        }));
        end = segment.range.end;
    }
    assert_eq!(end, result.secret.len());
}

#[test]
fn segments_cover_utf8_bytes_and_split_on_digit_and_uppercase() {
    let source = TestSource::new(&["äöüßéȿ"]);
    for profile in 0..=12 {
        for seed in 0..25 {
            assert_segments(&seeded(&source, &config(profile), seed));
        }
    }
    let source = TestSource::new(&["ß"]);
    let result = seeded(&source, &config(0), 0);
    assert_eq!(
        result.segments[0],
        Segment {
            kind: SegmentKind::Word,
            range: 0..2
        }
    );
    assert_eq!(
        result.segments[1],
        Segment {
            kind: SegmentKind::Separator,
            range: 2..3
        }
    );
}

#[test]
fn buffer_capacity_is_sufficient_even_when_uppercase_utf8_grows() {
    let source = TestSource::new(&["ȿɀ", "ßß", "äö"]);
    let prepared = PreparedSource::new(&source, profile_rules(config(12).profile)).unwrap();
    let expected = prepared.max_word_bytes * 6 + 10 + 1;
    for seed in 0..100 {
        let result = seeded(&source, &config(12), seed);
        assert!(result.secret.len() <= expected);
        assert_eq!(result.secret.capacity(), expected);
    }
}

struct FailAfter {
    remaining: usize,
    first_zero: bool,
}

impl TryRngCore for FailAfter {
    type Error = &'static str;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        if self.remaining == 0 {
            return Err("injected failure, never surfaced");
        }
        self.remaining -= 1;
        Ok(if self.first_zero { 0 } else { u32::MAX })
    }
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.try_next_u32().map(u64::from)
    }
    fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
        Err("injected failure")
    }
}

#[test]
fn random_source_errors_abort_before_and_after_secret_construction() {
    let source = TestSource::new(&["abcdef", "ghijkl", "mnopqr"]);
    for remaining in [0, 1, 4, 8] {
        let mut rng = FailAfter {
            remaining,
            first_zero: false,
        };
        assert!(matches!(
            generate_with_rng(&source, &config(12), &mut rng),
            Err(GenerateError::RandomSourceUnavailable)
        ));
    }
}

#[test]
fn failure_during_rejection_terminates_and_is_not_a_fallback() {
    let mut rng = FailAfter {
        remaining: 1,
        first_zero: true,
    };
    assert_eq!(
        draw(&mut rng, 3),
        Err(GenerateError::RandomSourceUnavailable)
    );
    for bound in [1, 2, 3, 5, 20, 21_000, usize::MAX] {
        let mut rng = FailAfter {
            remaining: 0,
            first_zero: false,
        };
        assert_eq!(
            draw(&mut rng, bound),
            Err(GenerateError::RandomSourceUnavailable)
        );
    }
}

#[test]
fn invalid_configuration_never_draws_randomness() {
    let mut rng = FailAfter {
        remaining: 0,
        first_zero: false,
    };
    let mut cfg = config(0);
    cfg.excluded_chars = vec!['-'];
    assert!(matches!(
        generate_with_rng(&TestSource::new(&["ab"]), &cfg, &mut rng),
        Err(GenerateError::InvalidConfig)
    ));
}

#[test]
fn system_rng_generates_a_valid_password() {
    let result = generate(&TestSource::new(&["ab", "cd"]), &config(12)).unwrap();
    assert_segments(&result);
    assert!(result.entropy_bits.is_finite());
}

fn chi_square(counts: &[usize]) -> f64 {
    let expected = counts.iter().sum::<usize>() as f64 / counts.len() as f64;
    counts
        .iter()
        .map(|&n| (n as f64 - expected).powi(2) / expected)
        .sum()
}

#[test]
fn seeded_chi_square_checks_words_separators_digit_positions_and_initials() {
    // Fixed cryptographic PRNG seed makes this regression test reproducible.
    // This detects selection mistakes, not the quality of the OS entropy source.
    let source = TestSource::new(&["aaaaaa", "bbbbbb", "cccccc"]);
    let mut rng = StdRng::seed_from_u64(0x4b61_6972_6f73);
    let mut words = [0; 3];
    let mut separators = [0; 20];
    let mut digit_words = [0; 4];
    let mut digit_positions = [0; 5];
    let mut digits = [0; 10];
    let mut initial_pairs = [0; 4];
    let pool: Vec<_> = profile_rules(config(8).profile)
        .separator_pool
        .chars()
        .collect();
    for _ in 0..20_000 {
        let result = generate_with_rng(&source, &config(8), &mut rng).unwrap();
        let tokens: Vec<_> = result.secret.split(|c| pool.contains(&c)).collect();
        let mut initials = [0; 4];
        for (i, word) in tokens.iter().enumerate() {
            initials[i] = usize::from(word.chars().next().unwrap().is_uppercase());
            words[(word.as_bytes()[0].to_ascii_lowercase() - b'a') as usize] += 1;
            if let Some((position, digit)) =
                word.chars().enumerate().find(|(_, c)| c.is_ascii_digit())
            {
                digit_words[i] += 1;
                digit_positions[position - 1] += 1;
                digits[digit.to_digit(10).unwrap() as usize] += 1;
            }
        }
        initial_pairs[initials[0] * 2 + initials[1]] += 1;
        for c in result.secret.chars().filter(|c| pool.contains(c)) {
            separators[pool.iter().position(|&p| p == c).unwrap()] += 1;
        }
    }
    for counts in [
        &words[..],
        &separators,
        &digit_words,
        &digit_positions,
        &digits,
        &initial_pairs,
    ] {
        let statistic = chi_square(counts);
        assert!(
            statistic < 60.0,
            "chi-square {statistic}, {} bins",
            counts.len()
        );
        assert!(counts.iter().all(|&n| n > 0));
    }
}

#[test]
fn doubled_separators_are_independently_drawn() {
    let source = TestSource::new(&["ab"]);
    let mut cfg = config(12);
    cfg.excluded_chars = profile_rules(cfg.profile)
        .separator_pool
        .chars()
        .filter(|&c| c != '-' && c != '+')
        .collect();
    let mut rng = StdRng::seed_from_u64(901);
    let mut pairs = [0; 4];
    for _ in 0..4_000 {
        let result = generate_with_rng(&source, &cfg, &mut rng).unwrap();
        for segment in result
            .segments
            .iter()
            .filter(|s| s.kind == SegmentKind::Separator)
        {
            let pair = &result.secret[segment.range.clone()];
            assert_eq!(pair.len(), 2);
            pairs[usize::from(pair.as_bytes()[0] == b'+') * 2
                + usize::from(pair.as_bytes()[1] == b'+')] += 1;
        }
    }
    assert!(chi_square(&pairs) < 30.0);
}

#[test]
fn random_uppercase_positions_are_uniform() {
    let source = TestSource::new(&["abcdef"]);
    let mut rng = StdRng::seed_from_u64(1337);
    let mut positions = [0; 6];
    for _ in 0..4_000 {
        let result = generate_with_rng(&source, &config(11), &mut rng).unwrap();
        for word in result.secret.split(|c: char| !c.is_alphanumeric()) {
            let letters = word.chars().filter(|c| c.is_alphabetic());
            positions[letters
                .enumerate()
                .find(|(_, c)| c.is_uppercase())
                .unwrap()
                .0] += 1;
        }
    }
    assert!(chi_square(&positions) < 35.0);
}

#[test]
fn identifiers_and_default_configuration_preserve_contract() {
    assert_eq!(GeneratorConfig::default(), config(0));
    for id in 0..=12 {
        assert_eq!(ProfileId::try_from(id).unwrap().get(), id);
    }
    assert!(ProfileId::new(13).is_err());
    assert!(ProfileId::new(u8::MAX).is_err());
}

#[test]
fn error_texts_never_include_source_or_rng_error_details() {
    assert_eq!(
        GenerateError::InvalidConfig.to_string(),
        "the generation configuration is invalid"
    );
    assert_eq!(
        GenerateError::RandomSourceUnavailable.to_string(),
        "the system random source is unavailable"
    );
}
