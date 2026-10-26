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
    // The original concept allowed all-identical tuples: profile 2's separator
    // term changes from 3*log2(5) = 6.965784 to log2(125 - 5) = 6.906891.
    let expected = [
        51.092557, 57.432407, 64.339297, 71.983154, 74.053543, 75.821097, 77.070616, 78.038436,
        82.038436, 102.043825, 121.987060, 131.496835, 153.106485,
    ];
    for (profile, expected) in expected.into_iter().enumerate() {
        let len = if profile == 0 { 7_000 } else { 21_000 };
        let source = TestSource::six_letters(len);
        let cfg = config(profile as u8);
        let model = entropy_bits(len, &cfg);
        let actual = entropy_bits_for(&source, &cfg).unwrap();
        assert!(
            (model - expected).abs() < 1e-6,
            "profile {profile}: {model}"
        );
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
    let expected = (20.0_f64.powi(10) - 20.0).log2() - (18.0_f64.powi(10) - 18.0).log2();
    assert!((difference - expected).abs() < 1e-10);
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
            GenerateError::InvalidSource,
        );
    }
}

#[test]
fn short_words_only_fail_when_an_inner_digit_position_is_required() {
    let source = TestSource::new(&["a"]);
    assert!(generate(&source, &config(2)).is_ok());
    assert_error(&source, &config(3), GenerateError::InvalidSource);
}

#[test]
fn variable_lengths_report_conservative_entropy() {
    let source = TestSource::new(&["ab", "abcdef"]);
    let cfg = config(11);
    let bits = entropy_bits_for(&source, &cfg).unwrap();
    let expected = 6.0 + (20.0_f64.powi(5) - 20.0).log2() + 60.0_f64.log2() + 6.0;
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
    assert_error(&source, &config(11), GenerateError::InvalidSource);
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
    let source = TestSource::new(&["aß", "äö", "ȿɀ"]);
    for profile in [11, 12] {
        for seed in 0..100 {
            let result = seeded(&source, &config(profile), seed);
            assert_eq!(
                result.secret.chars().filter(|c| c.is_uppercase()).count(),
                6
            );
            assert!(!result.secret.contains("SS"));
            assert!(!result.secret.contains('ẞ'));
            assert_segments(&result);
        }
    }
    assert_eq!(uppercase('ß'), None);
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
    let source = TestSource::new(&["ȿɀ", "aß", "äö"]);
    let prepared = PreparedSource::new(&source, profile_rules(config(12).profile)).unwrap();
    let expected = prepared.max_word_bytes * 6 + 10 + 1;
    for seed in 0..100 {
        let result = seeded(&source, &config(12), seed);
        assert!(result.secret.len() <= expected);
        assert!(result.secret.capacity() >= expected);
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
    let bounds = vec![
        1,
        2,
        3,
        5,
        20,
        21_000,
        (1 << 31) + 1,
        u32::MAX as usize,
        usize::MAX,
    ];
    #[cfg(target_pointer_width = "64")]
    let bounds = {
        let mut all = bounds;
        all.extend([1 << 32, (1 << 63) + 1]);
        all
    };
    for bound in bounds {
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
    // Chi-square critical values at p = 0.001 for each bin count (df = bins - 1).
    // Source: https://www.itl.nist.gov/div898/handbook/eda/section3/eda3674.htm
    for (counts, threshold) in [
        (&words[..], 13.82),
        (&separators[..], 43.82),
        (&digit_words[..], 16.27),
        (&digit_positions[..], 18.47),
        (&digits[..], 27.88),
        (&initial_pairs[..], 16.27),
    ] {
        let statistic = chi_square(counts);
        assert!(
            statistic < threshold,
            "chi-square {statistic}, {} bins",
            counts.len()
        );
        assert!(counts.iter().all(|&n| n > 0));
    }
}

#[test]
fn doubled_separators_allow_equal_pairs_with_a_globally_distinct_tuple() {
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
        let tuple: Vec<_> = result
            .secret
            .chars()
            .filter(|c| !c.is_alphanumeric())
            .collect();
        assert!(tuple.iter().any(|&c| c != tuple[0]));
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
    // Conditioning the ten-position tuple slightly reduces equal-pair counts:
    // 255 completions for --/++, 256 for -+/+-, out of 1022 allowed tuples.
    let total = pairs.iter().sum::<usize>() as f64;
    let statistic: f64 = pairs
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let completions = if i == 0 || i == 3 { 255.0 } else { 256.0 };
            let expected = total * completions / 1022.0;
            (count as f64 - expected).powi(2) / expected
        })
        .sum();
    assert!(statistic < 30.0);
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
    assert_eq!(
        GenerateError::InvalidSource.to_string(),
        "the word source is invalid for this profile"
    );
}

#[test]
fn mixed_initials_use_the_smallest_actual_choice_count() {
    let source = TestSource::new(&["中文", "ab"]);
    assert_eq!(
        entropy_bits_for(&source, &config(8)),
        entropy_bits_for(&source, &config(7))
    );
    for seed in 0..50 {
        assert_segments(&seeded(&source, &config(8), seed));
    }
}

#[test]
fn decomposed_latin_accents_are_rejected_as_invalid_source() {
    for word in ["a\u{0308}pfel", "cafe\u{0301}", "i\u{0307}gel"] {
        assert_error(
            &TestSource::new(&[word]),
            &config(0),
            GenerateError::InvalidSource,
        );
    }
}

#[test]
fn built_in_pools_are_unique_non_alphanumeric_ascii_and_rules_are_usable() {
    for profile in 0..=12 {
        let rules = profile_rules(config(profile).profile);
        let pool: HashSet<_> = rules.separator_pool.chars().collect();
        assert_eq!(pool.len(), rules.separator_pool.len());
        assert!(pool.iter().all(|c| c.is_ascii_punctuation()));
        assert!(!pool.is_empty());
        assert!(rules.word_count > 0);
        assert!(rules.separators_per_gap > 0);
    }
}

struct ScriptedRng(std::collections::VecDeque<u32>);

impl TryRngCore for ScriptedRng {
    type Error = &'static str;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.0.pop_front().ok_or("script exhausted")
    }
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Err("unexpected 64-bit draw")
    }
    fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
        Err("unexpected byte draw")
    }
}

fn midpoint(choices: usize, choice: usize) -> u32 {
    (((2 * choice + 1) as u64 * (1_u64 << 32)) / (2 * choices) as u64) as u32
}

fn assert_unique_with_entropy(mut outputs: Vec<GeneratedPassword>, expected_count: usize) {
    assert_eq!(outputs.len(), expected_count);
    outputs.sort_unstable_by(|a, b| a.secret.as_str().cmp(b.secret.as_str()));
    assert!(outputs
        .windows(2)
        .all(|pair| pair[0].secret.as_str() != pair[1].secret.as_str()));
    let exact = (expected_count as f64).log2();
    for result in &outputs {
        assert!(result.entropy_bits <= exact + 1e-10);
        assert!((result.entropy_bits - exact).abs() < 1e-10);
        assert_segments(result);
    }
}

#[test]
fn exhausts_variable_length_word_sequences_and_separator_choices() {
    let source = TestSource::new(&["ab", "cde"]);
    let mut cfg = config(2);
    cfg.excluded_chars = vec!['.', ',', '#'];
    let mut outputs = Vec::new();
    for word_mask in 0..16 {
        for separator_mask in 1..7 {
            let mut script = std::collections::VecDeque::new();
            for index in 0..4 {
                if index > 0 {
                    script.push_back(midpoint(2, (separator_mask >> (index - 1)) & 1));
                }
                script.push_back(midpoint(2, (word_mask >> index) & 1));
            }
            let mut rng = ScriptedRng(script);
            outputs.push(generate_with_rng(&source, &cfg, &mut rng).unwrap());
            assert!(rng.0.is_empty());
        }
    }
    assert_unique_with_entropy(outputs, 96);
}

#[test]
fn allowed_separator_tuples_are_uniform_with_two_choices_and_three_positions() {
    let source = TestSource::new(&["ab"]);
    let mut cfg = config(2);
    cfg.excluded_chars = vec!['.', ',', '#'];
    let mut rng = StdRng::seed_from_u64(0x5345_5053);
    let mut counts = [0; 8];
    for _ in 0..24_000 {
        let result = generate_with_rng(&source, &cfg, &mut rng).unwrap();
        let code = result
            .secret
            .chars()
            .filter(|c| !c.is_alphanumeric())
            .fold(0, |code, c| code * 2 + usize::from(c == '+'));
        counts[code] += 1;
        assert!((result.entropy_bits - 6.0_f64.log2()).abs() < 1e-10);
    }
    assert_eq!(counts[0], 0);
    assert_eq!(counts[7], 0);
    assert!(counts[1..7].iter().all(|&n| n > 0));
    assert!(chi_square(&counts[1..7]) < 20.52); // df=5, p=0.001, fixed seed.
}

#[test]
fn repeated_tuple_rejection_preserves_utf8_words_and_segments_and_propagates_errors() {
    let source = TestSource::new(&["äöü"]);
    let mut cfg = config(2);
    cfg.excluded_chars = vec!['.', ',', '#'];
    let initial = std::collections::VecDeque::from([
        midpoint(1, 0),
        midpoint(2, 0),
        midpoint(1, 0),
        midpoint(2, 0),
        midpoint(1, 0),
        midpoint(2, 0),
        midpoint(1, 0),
    ]);
    let mut retry = initial.clone();
    retry.extend([midpoint(2, 1); 3]); // Reject +++ as well as the initial ---.
    retry.extend([midpoint(2, 0), midpoint(2, 1), midpoint(2, 0)]);
    let mut rng = ScriptedRng(retry);
    let result = generate_with_rng(&source, &cfg, &mut rng).unwrap();
    assert!(rng.0.is_empty());
    assert_eq!(result.secret.as_str(), "äöü-äöü+äöü-äöü");
    let fixed = seeded(&source, &config(0), 0);
    assert_eq!(result.segments, fixed.segments);
    assert_segments(&result);
    for supplied_retry_draws in 0..6 {
        let mut script = initial.clone();
        script.extend(std::iter::repeat_n(midpoint(2, 1), supplied_retry_draws));
        assert!(matches!(
            generate_with_rng(&source, &cfg, &mut ScriptedRng(script)),
            Err(GenerateError::RandomSourceUnavailable)
        ));
    }
}

#[test]
fn custom_entropy_handles_zero_one_two_and_many_separator_positions() {
    let source = TestSource::new(&["ab"]);
    for (words, width, pool, expected) in [
        (1, 1, "-+", 0.0),
        (2, 1, "-+", 1.0),
        (3, 1, "-+", 1.0),
        (4, 1, "-+", 6.0_f64.log2()),
        (4, 1, "-", 0.0),
        (2, 2, "-+", 1.0),
        (64, 8, "-+", 504.0),
        (64, 8, "-+.,#*!?_:;/()\"'$%&=", 504.0 * 20.0_f64.log2()),
    ] {
        let custom = CustomProfile::new(words, pool, width, false, CaseRule::Lowercase).unwrap();
        let bits = entropy_bits_for_custom(&source, &custom, &[]).unwrap();
        assert!((bits - expected).abs() < 1e-10);
        let result = generate_custom(&source, &custom, &[]).unwrap();
        assert_eq!(result.entropy_bits, bits);
        assert_segments(&result);
    }
    let custom = CustomProfile::new(4, "-+", 1, false, CaseRule::Lowercase).unwrap();
    let result = generate_custom(&source, &custom, &['+']).unwrap();
    assert_eq!(result.entropy_bits, 0.0);
    assert_eq!(result.secret.as_str(), "ab-ab-ab-ab");
}

#[test]
fn profile_twelve_accepts_all_1022_nonconstant_separator_tuples() {
    let source = TestSource::new(&["ab"]);
    let mut cfg = config(12);
    cfg.excluded_chars = profile_rules(cfg.profile)
        .separator_pool
        .chars()
        .filter(|&c| c != '-' && c != '+')
        .collect();
    let expected = 1022.0_f64.log2() + 60.0_f64.log2() + 6.0;
    let mut seen = HashSet::new();
    // Every pair may be equal (e.g. --, ++, --, --, --); only the entire
    // ten-position tuple must differ. Include both rejected initial tuples.
    for mask in 0..1024 {
        let mut script = std::collections::VecDeque::from([midpoint(6, 0)]);
        for word in 0..6 {
            if word > 0 {
                for offset in 0..2 {
                    script.push_back(midpoint(2, (mask >> (2 * (word - 1) + offset)) & 1));
                }
            }
            script.push_back(midpoint(1, 0));
            if word == 0 {
                script.push_back(midpoint(1, 0));
            }
            script.push_back(midpoint(2, 0));
            if word == 0 {
                script.push_back(midpoint(10, 0));
            }
        }
        if mask == 0 || mask == 1023 {
            script.extend((0..10).map(|i| midpoint(2, usize::from(i == 0))));
        }
        let mut rng = ScriptedRng(script);
        let result = generate_with_rng(&source, &cfg, &mut rng).unwrap();
        assert!(rng.0.is_empty());
        assert_segments(&result);
        assert!((result.entropy_bits - expected).abs() < 1e-10);
        let tuple: String = result
            .secret
            .chars()
            .filter(|c| !c.is_alphanumeric())
            .collect();
        assert!(tuple.contains('-') && tuple.contains('+'));
        seen.insert(tuple);
    }
    assert_eq!(seen.len(), 1022);
}

#[test]
fn exhausts_profile_twelve_digit_and_uppercase_choices_without_collisions() {
    let source = TestSource::new(&["ab"]);
    let mut cfg = config(12);
    cfg.excluded_chars = profile_rules(cfg.profile)
        .separator_pool
        .chars()
        .filter(|&c| c != '-')
        .collect();
    let mut outputs = Vec::new();
    for digit_word in 0..6 {
        for digit in 0..10 {
            for uppercase_mask in 0..64 {
                let mut script = std::collections::VecDeque::from([midpoint(6, digit_word)]);
                for index in 0..6 {
                    if index > 0 {
                        script.extend([midpoint(1, 0); 2]);
                    }
                    script.push_back(midpoint(1, 0));
                    if index == digit_word {
                        script.push_back(midpoint(1, 0));
                    }
                    script.push_back(midpoint(2, (uppercase_mask >> index) & 1));
                    if index == digit_word {
                        script.push_back(midpoint(10, digit));
                    }
                }
                let mut rng = ScriptedRng(script);
                outputs.push(generate_with_rng(&source, &cfg, &mut rng).unwrap());
                assert!(rng.0.is_empty());
            }
        }
    }
    assert_unique_with_entropy(outputs, 3_840);
}
