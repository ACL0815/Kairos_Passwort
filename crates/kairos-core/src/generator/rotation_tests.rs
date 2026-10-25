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
        let json = serde_json::to_string(&raw).unwrap();
        assert!(serde_json::from_str::<CustomProfile>(&json).is_err());
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
    let profiles = vec![profile(2, "-"); 257];
    assert!(ProfileRotation::new(profiles.clone()).is_err());
    let json = serde_json::to_string(&profiles).unwrap();
    assert!(serde_json::from_str::<ProfileRotation>(&json).is_err());
    assert!(ProfileRotation::new(profiles[..256].to_vec()).is_ok());
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
    for profiles in pairs {
        let minimum = profiles
            .iter()
            .map(|p| entropy_bits_for_custom(&Source(&["ab", "cd"]), p, &[]).unwrap())
            .fold(f64::INFINITY, f64::min);
        let result = entropy(&ProfileRotation::new(profiles).unwrap(), &[]);
        assert_eq!(result.rotation_bonus_bits, 1.0);
        assert_eq!(result.entropy_bits, minimum + 1.0);
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
}

#[test]
fn unequal_strengths_never_credit_bonus_to_the_selected_stronger_profile() {
    let rotation = ProfileRotation::new(vec![profile(1, "-"), profile(8, "-")]).unwrap();
    let result = entropy(&rotation, &[]);
    assert_eq!(result.entropy_bits, 2.0);
    assert_eq!(result.rotation_bonus_bits, 1.0);
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
        assert_eq!(generated.password.entropy_bits, 2.0);
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
