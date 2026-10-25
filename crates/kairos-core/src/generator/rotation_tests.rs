use super::*;
use crate::{Language, ProfileId};
use rand::{rngs::StdRng, SeedableRng};

struct Source<'a>(&'a [&'a str]);

impl WordSource for Source<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn word(&self, index: usize) -> &str {
        self.0[index]
    }

    fn language(&self, _: usize) -> Language {
        Language::German
    }
}

fn profile(words: usize, pool: &str) -> CustomProfile {
    CustomProfile::new(words, pool, 1, false, CaseRule::Lowercase).unwrap()
}

fn entropy(rotation: &ProfileRotation, excluded: &[char]) -> RotationEntropy {
    entropy_bits_for_rotation(
        &Source(&["ab", "cd"]),
        rotation,
        excluded,
        ProfileKnowledge::Unknown,
    )
    .unwrap()
}

#[test]
fn validates_custom_rules_at_construction_and_deserialization() {
    for (words, pool, gaps) in [
        (0, "-", 1),
        (65, "-", 1),
        (4, "-", 0),
        (4, "-", 9),
        (4, "", 1),
        (4, "--", 1),
        (4, "-a", 1),
        (4, "-0", 1),
        (4, " ", 1),
        (4, "\n", 1),
        (4, "é", 1),
    ] {
        assert_eq!(
            CustomProfile::new(words, pool, gaps, false, CaseRule::Lowercase),
            Err(GenerateError::InvalidConfig)
        );
        let mut raw = profile_rules(ProfileId::default()).clone();
        raw.word_count = words;
        raw.separator_pool = Cow::Owned(pool.to_owned());
        raw.separators_per_gap = gaps;
        assert!(CustomProfile::try_from(raw).is_err());
        let mut value = serde_json::to_value(profile(4, "-")).unwrap();
        value["word_count"] = words.into();
        value["separator_pool"] = pool.into();
        value["separators_per_gap"] = gaps.into();
        assert!(serde_json::from_value::<CustomProfile>(value).is_err());
    }
    assert!(CustomProfile::new(64, "-+", 8, true, CaseRule::OneRandomUppercase).is_ok());
    let mut value = serde_json::to_value(profile(4, "-")).unwrap();
    value["unknown_rule"] = true.into();
    assert!(serde_json::from_value::<CustomProfile>(value).is_err());
}

#[test]
fn json_roundtrip_preserves_all_modes_and_rotation() {
    let profiles: Vec<_> = [
        CaseRule::Lowercase,
        CaseRule::RandomInitial,
        CaseRule::OneRandomUppercase,
    ]
    .into_iter()
    .map(|case| CustomProfile::new(3, "-+\"", 2, true, case).unwrap())
    .collect();
    for profile in &profiles {
        let json = serde_json::to_string(profile).unwrap();
        let restored: CustomProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(&restored, profile);
        assert!(generate_custom(&Source(&["ab", "äö"]), &restored, &[]).is_ok());
    }
    let rotation = ProfileRotation::new(profiles).unwrap();
    let json = serde_json::to_string(&rotation).unwrap();
    assert_eq!(
        serde_json::from_str::<ProfileRotation>(&json).unwrap(),
        rotation
    );
}

#[test]
fn validates_rotation_size_including_json_boundary() {
    assert_eq!(
        ProfileRotation::new(vec![]),
        Err(GenerateError::InvalidConfig)
    );
    assert!(serde_json::from_str::<ProfileRotation>("[]").is_err());
    let profiles = vec![profile(2, "-"); 17];
    assert!(ProfileRotation::new(profiles.clone()).is_err());
    let json = serde_json::to_string(&profiles).unwrap();
    assert!(serde_json::from_str::<ProfileRotation>(&json).is_err());
    assert!(ProfileRotation::new(profiles[..16].to_vec()).is_ok());
}

#[test]
fn custom_profiles_use_effective_pools_and_real_sources() {
    let source = Source(&["ab", "cdef"]);
    let custom = CustomProfile::new(3, "-+", 2, true, CaseRule::OneRandomUppercase).unwrap();
    let bits = entropy_bits_for_custom(&source, &custom, &['+']).unwrap();
    let expected = 3.0 + 30.0_f64.log2() + 3.0;
    assert!((bits - expected).abs() < 1e-10);
    let generated = generate_custom(&source, &custom, &['+']).unwrap();
    assert_eq!(generated.entropy_bits, bits);
    assert!(!generated.secret.contains('+'));
    assert_eq!(generated.secret.matches("--").count(), 2);
    assert_eq!(
        generated
            .secret
            .chars()
            .filter(char::is_ascii_digit)
            .count(),
        1
    );
    assert_eq!(
        generated
            .secret
            .chars()
            .filter(|c| c.is_uppercase())
            .count(),
        3
    );
    assert!(matches!(
        generate_custom(&source, &custom, &['-', '+']),
        Err(GenerateError::InvalidConfig)
    ));
    assert!(matches!(
        generate_custom(&Source(&["a"]), &custom, &[]),
        Err(GenerateError::InvalidSource)
    ));
    let one = profile(1, "-");
    assert_eq!(
        generate_custom(&Source(&["ab"]), &one, &[])
            .unwrap()
            .secret
            .as_str(),
        "ab"
    );
}

#[test]
fn disjoint_proofs_cover_word_counts_pools_gap_widths_and_digits() {
    let pairs = [
        vec![profile(2, "-"), profile(3, "-")],
        vec![profile(2, "-"), profile(2, "+")],
        vec![
            profile(2, "-"),
            CustomProfile::new(2, "-", 2, false, CaseRule::Lowercase).unwrap(),
        ],
        vec![
            profile(2, "-"),
            CustomProfile::new(2, "-", 1, true, CaseRule::Lowercase).unwrap(),
        ],
    ];
    for (profiles, (bonus, bits)) in
        pairs
            .into_iter()
            .zip([(0.0, 2.0), (1.0, 3.0), (1.0, 3.0), (0.0, 2.0)])
    {
        let result = entropy(&ProfileRotation::new(profiles).unwrap(), &[]);
        assert_eq!(result.rotation_bonus_bits, bonus);
        assert_eq!(result.entropy_bits, bits);
    }
}

#[test]
fn sixteen_equal_disjoint_profiles_get_exactly_four_bits() {
    let profiles = "-+.,#*!?_:;/()\"'"
        .chars()
        .map(|c| profile(2, &c.to_string()))
        .collect();
    let rotation = ProfileRotation::new(profiles).unwrap();
    let source = Source(&["ab"]);
    let result =
        entropy_bits_for_rotation(&source, &rotation, &[], ProfileKnowledge::Unknown).unwrap();
    assert_eq!(result.entropy_bits, 4.0);
    assert_eq!(result.rotation_bonus_bits, 4.0);
    let known =
        entropy_bits_for_rotation(&source, &rotation, &[], ProfileKnowledge::Known).unwrap();
    assert_eq!(known.entropy_bits, 0.0);
    assert_eq!(known.rotation_bonus_bits, 0.0);
    let generated = generate_rotating(&source, &rotation, &[], ProfileKnowledge::Unknown).unwrap();
    assert_eq!(generated.password.entropy_bits, 4.0);
    assert_eq!(generated.rotation_bonus_bits, 4.0);
    let generated = generate_rotating(&source, &rotation, &[], ProfileKnowledge::Known).unwrap();
    assert_eq!(generated.password.entropy_bits, 0.0);
    assert_eq!(generated.rotation_bonus_bits, 0.0);
}

#[test]
fn unequal_strengths_never_credit_bonus_to_the_selected_stronger_profile() {
    let rotation = ProfileRotation::new(vec![profile(1, "-"), profile(8, "-")]).unwrap();
    let result = entropy(&rotation, &[]);
    assert_eq!(result.entropy_bits, 1.0);
    assert_eq!(result.rotation_bonus_bits, 0.0);
    let known = entropy_bits_for_rotation(
        &Source(&["ab", "cd"]),
        &rotation,
        &[],
        ProfileKnowledge::Known,
    )
    .unwrap();
    assert_eq!(known.entropy_bits, 1.0);
    assert_eq!(known.rotation_bonus_bits, 0.0);
    let mut rng = StdRng::seed_from_u64(8);
    let mut seen = [false; 2];
    for _ in 0..100 {
        let generated = generate_rotating_with_rng(
            &Source(&["ab", "cd"]),
            &rotation,
            &[],
            ProfileKnowledge::Unknown,
            &mut rng,
        )
        .unwrap();
        seen[usize::from(generated.password.secret.contains('-'))] = true;
        assert_eq!(generated.password.entropy_bits, 1.0);
        assert_eq!(generated.rotation_bonus_bits, 0.0);
    }
    assert_eq!(seen, [true; 2]);
}

#[test]
fn overlap_duplicates_partial_overlap_and_unused_pools_get_no_bonus() {
    for profiles in [
        vec![profile(2, "-"), profile(2, "-")],
        vec![profile(2, "-+"), profile(2, "+.")],
        vec![profile(2, "-"), profile(2, "+"), profile(2, "-")],
        vec![profile(1, "-"), profile(1, "+")],
        vec![
            profile(2, "-"),
            CustomProfile::new(2, "-", 1, false, CaseRule::OneRandomUppercase).unwrap(),
        ],
        vec![
            CustomProfile::from(ProfileId::new(0).unwrap()),
            CustomProfile::from(ProfileId::new(1).unwrap()),
        ],
    ] {
        assert_eq!(
            entropy(&ProfileRotation::new(profiles).unwrap(), &[]).rotation_bonus_bits,
            0.0
        );
    }
    let rotation = ProfileRotation::new(vec![profile(2, "-+"), profile(2, "+.")]).unwrap();
    assert_eq!(entropy(&rotation, &['+']).rotation_bonus_bits, 1.0);
    assert_eq!(
        entropy(&rotation, &['+', '+', 'a']).rotation_bonus_bits,
        1.0
    );
}

struct FailingRng;

impl TryRngCore for FailingRng {
    type Error = &'static str;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Err("injected failure")
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Err("injected failure")
    }

    fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
        Err("injected failure")
    }
}

#[test]
fn every_entry_is_validated_before_selection_and_rng_failures_abort() {
    let rotation = ProfileRotation::new(vec![profile(2, "-"), profile(3, "+")]).unwrap();
    let source = Source(&["ab"]);
    assert!(matches!(
        generate_rotating_with_rng(
            &source,
            &rotation,
            &['+'],
            ProfileKnowledge::Unknown,
            &mut FailingRng,
        ),
        Err(GenerateError::InvalidConfig)
    ));
    let digit = CustomProfile::new(2, "-", 1, true, CaseRule::Lowercase).unwrap();
    let invalid = ProfileRotation::new(vec![profile(2, "-"), digit]).unwrap();
    assert!(matches!(
        generate_rotating_with_rng(
            &Source(&["a"]),
            &invalid,
            &[],
            ProfileKnowledge::Unknown,
            &mut FailingRng,
        ),
        Err(GenerateError::InvalidSource)
    ));
    assert!(matches!(
        generate_rotating_with_rng(
            &source,
            &rotation,
            &[],
            ProfileKnowledge::Unknown,
            &mut FailingRng,
        ),
        Err(GenerateError::RandomSourceUnavailable)
    ));
    assert_eq!(
        entropy_bits_for_rotation(&Source(&[]), &rotation, &[], ProfileKnowledge::Unknown),
        Err(GenerateError::EmptySource)
    );
}

#[test]
fn profile_draws_are_uniform_and_independent_with_repetition() {
    for (count, threshold) in [(3, 13.82), (7, 22.46), (16, 37.70)] {
        let rotation =
            ProfileRotation::new((1..=count).map(|n| profile(n, "-")).collect()).unwrap();
        let mut rng = StdRng::seed_from_u64(0x524f_5441_5445);
        let mut counts = vec![0_usize; count];
        let mut pairs = [0_usize; 9];
        let mut previous = None;
        for _ in 0..32_000 {
            let generated = generate_rotating_with_rng(
                &Source(&["ab"]),
                &rotation,
                &[],
                ProfileKnowledge::Unknown,
                &mut rng,
            )
            .unwrap();
            let index = generated.password.secret.matches('-').count();
            counts[index] += 1;
            if count == 3 {
                if let Some(last) = previous {
                    pairs[last * 3 + index] += 1;
                }
                previous = Some(index);
            }
        }
        let chi_square = |counts: &[usize]| {
            let expected = counts.iter().sum::<usize>() as f64 / counts.len() as f64;
            counts
                .iter()
                .map(|&n| (n as f64 - expected).powi(2) / expected)
                .sum::<f64>()
        };
        assert!(
            chi_square(&counts) < threshold,
            "{count} profiles: {counts:?}"
        );
        if count == 3 {
            assert!(chi_square(&pairs) < 26.13, "{pairs:?}");
        }
    }
}

#[test]
fn builtin_rules_and_seeded_outputs_remain_identical_for_custom_copies() {
    for id in 0..=12 {
        let id = ProfileId::new(id).unwrap();
        let custom = CustomProfile::from(id);
        assert_eq!(custom.rules(), profile_rules(id));
        let cfg = GeneratorConfig {
            profile: id,
            excluded_chars: vec![],
        };
        let source = Source(&["abcdef", "äpfeln"]);
        let prepared = PreparedSource::new(&source, custom.rules()).unwrap();
        let pool = separators_for(custom.rules(), &[]).unwrap();
        for seed in 0..20 {
            let original =
                generate_with_rng(&source, &cfg, &mut StdRng::seed_from_u64(seed)).unwrap();
            let copied = generate_prepared(
                &prepared,
                custom.rules(),
                &pool,
                &mut StdRng::seed_from_u64(seed),
            )
            .unwrap();
            assert_eq!(original.secret, copied.secret);
            assert_eq!(original.segments, copied.segments);
            assert_eq!(original.entropy_bits, copied.entropy_bits);
        }
    }
}

#[test]
fn versioned_json_keeps_v1_defaults_and_rejects_unsupported_rules() {
    let original = profile(2, "-");
    let mut value = serde_json::to_value(&original).unwrap();
    assert_eq!(value["version"], 1);
    let mut unversioned = value.clone();
    unversioned.as_object_mut().unwrap().remove("version");
    assert!(serde_json::from_value::<CustomProfile>(unversioned).is_err());
    value
        .as_object_mut()
        .unwrap()
        .remove("recommended_source_len");
    assert_eq!(
        serde_json::from_value::<CustomProfile>(value.clone()).unwrap(),
        original
    );
    for version in [
        serde_json::json!(0),
        serde_json::json!(2),
        serde_json::json!("1"),
    ] {
        value["version"] = version;
        assert!(serde_json::from_value::<CustomProfile>(value.clone()).is_err());
    }
    value["version"] = 1.into();
    value["case"] = "future_case".into();
    assert!(serde_json::from_value::<CustomProfile>(value.clone()).is_err());
    value["case"] = "lowercase".into();
    value.as_object_mut().unwrap().remove("word_count");
    assert!(serde_json::from_value::<CustomProfile>(value).is_err());
    for id in 0..=12 {
        let custom = CustomProfile::from(ProfileId::new(id).unwrap());
        let restored: CustomProfile =
            serde_json::from_str(&serde_json::to_string(&custom).unwrap()).unwrap();
        assert_eq!(custom, restored);
    }
}

#[test]
fn custom_recommendations_produce_advisories_without_changing_strength() {
    let mut rules = profile(2, "-").rules().clone();
    rules.recommended_source_len = 10;
    let custom = CustomProfile::try_from(rules).unwrap();
    assert_eq!(
        custom.warnings(2),
        vec![crate::ProfileWarning::SourceBelowRecommendation {
            actual: 2,
            recommended: 10
        }]
    );
    assert!(custom.warnings(10).is_empty());
    assert_eq!(
        entropy_bits_for_custom(&Source(&["ab", "cd"]), &custom, &[]).unwrap(),
        2.0
    );
}

struct ChoiceScript(std::collections::VecDeque<u32>);

impl TryRngCore for ChoiceScript {
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

fn enumerate_events(
    events: &[(usize, Option<usize>)],
    script: &mut Vec<u32>,
    probability: f64,
    visit: &mut impl FnMut(&[u32], f64),
) {
    let Some((&(upper, fixed), tail)) = events.split_first() else {
        visit(script, probability);
        return;
    };
    let choices = fixed.map_or(0..upper, |value| value..value + 1);
    for choice in choices {
        let midpoint = (((2 * choice + 1) as u64 * (1_u64 << 32)) / (2 * upper) as u64) as u32;
        script.push(midpoint);
        enumerate_events(tail, script, probability / upper as f64, visit);
        script.pop();
    }
}

fn exhaust_mixture(source: &dyn WordSource, profiles: Vec<CustomProfile>, expect_disjoint: bool) {
    let rotation = ProfileRotation::new(profiles).unwrap();
    let mut mixture = std::collections::HashMap::<String, f64>::new();
    let mut supports = Vec::new();
    let mut reported = None;
    let mut bonus = 0.0;
    for (index, profile) in rotation.profiles().iter().enumerate() {
        let rules = profile.rules();
        let prepared = PreparedSource::new(source, rules).unwrap();
        let pool = separators_for(rules, &[]).unwrap();
        let mut support = HashSet::new();
        for mut word_code in 0..prepared.words.len().pow(rules.word_count as u32) {
            let words: Vec<_> = (0..rules.word_count)
                .map(|_| {
                    let word = word_code % prepared.words.len();
                    word_code /= prepared.words.len();
                    word
                })
                .collect();
            for digit_word in 0..if rules.insert_digit {
                rules.word_count
            } else {
                1
            } {
                let mut events = vec![(rotation.profiles().len(), Some(index))];
                if rules.insert_digit {
                    events.push((rules.word_count, Some(digit_word)));
                }
                for (word_index, &choice) in words.iter().enumerate() {
                    if word_index > 0 {
                        events.extend(std::iter::repeat_n(
                            (pool.len(), None),
                            rules.separators_per_gap,
                        ));
                    }
                    events.push((prepared.words.len(), Some(choice)));
                    let word = &prepared.words[choice];
                    let digit_here = rules.insert_digit && word_index == digit_word;
                    if digit_here {
                        events.push((word.chars().count() - 1, None));
                    }
                    match rules.case {
                        CaseRule::Lowercase => (),
                        CaseRule::RandomInitial => events.push((2, None)),
                        CaseRule::OneRandomUppercase => events.push((
                            word.chars().filter(|&c| uppercase(c).is_some()).count(),
                            None,
                        )),
                    }
                    if digit_here {
                        events.push((10, None));
                    }
                }
                enumerate_events(&events, &mut Vec::new(), 1.0, &mut |script, probability| {
                    let mut rng = ChoiceScript(script.iter().copied().collect());
                    let result = generate_rotating_with_rng(
                        source,
                        &rotation,
                        &[],
                        ProfileKnowledge::Unknown,
                        &mut rng,
                    )
                    .unwrap();
                    assert!(rng.0.is_empty());
                    if let Some(previous) = reported {
                        assert_eq!(result.password.entropy_bits, previous);
                    }
                    reported = Some(result.password.entropy_bits);
                    bonus = result.rotation_bonus_bits;
                    let output = result.password.secret.to_string();
                    support.insert(output.clone());
                    *mixture.entry(output).or_default() += probability;
                });
            }
        }
        supports.push(support);
    }
    assert!((mixture.values().sum::<f64>() - 1.0).abs() < 1e-10);
    let actual_bits = -mixture.values().copied().fold(0.0_f64, f64::max).log2();
    assert!(
        actual_bits + 1e-10 >= reported.unwrap(),
        "actual {actual_bits}, reported {reported:?}"
    );
    let disjoint = supports
        .iter()
        .enumerate()
        .all(|(i, a)| supports[..i].iter().all(|b| a.is_disjoint(b)));
    assert_eq!(disjoint, expect_disjoint);
    if bonus > 0.0 {
        assert!(disjoint);
    }
}

#[test]
fn exhausts_actual_mixture_probabilities_and_disjointness() {
    let make = |w, s: &str, k, d, c| CustomProfile::new(w, s, k, d, c).unwrap();
    exhaust_mixture(
        &Source(&["ab", "cd"]),
        vec![
            make(2, "-", 1, false, CaseRule::Lowercase),
            make(2, "-", 2, false, CaseRule::Lowercase),
        ],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "cde"]),
        vec![
            make(2, "-", 2, true, CaseRule::Lowercase),
            make(3, "-", 1, true, CaseRule::Lowercase),
        ],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "cd"]),
        vec![
            make(2, "-", 1, false, CaseRule::RandomInitial),
            make(2, "-", 1, true, CaseRule::OneRandomUppercase),
        ],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "中文"]),
        vec![
            make(2, "-", 1, false, CaseRule::Lowercase),
            make(2, "+", 1, false, CaseRule::RandomInitial),
        ],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "cde"]),
        vec![profile(2, "-+"), profile(3, "-")],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "cd"]),
        vec![
            make(1, "-", 1, true, CaseRule::RandomInitial),
            make(2, "-", 1, true, CaseRule::Lowercase),
        ],
        true,
    );
    exhaust_mixture(
        &Source(&["ab", "cde"]),
        vec![profile(2, "-+"), profile(2, "+.")],
        false,
    );
    exhaust_mixture(
        &Source(&["ab", "中文"]),
        vec![
            profile(2, "-"),
            make(2, "-", 1, false, CaseRule::RandomInitial),
        ],
        false,
    );
}
