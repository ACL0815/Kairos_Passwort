#[path = "../tools/pipeline.rs"]
mod pipeline;

use std::collections::HashSet;

use kairos_core::{Language, WordSource};
use sha2::{Digest, Sha256};

const TEXT: &str = include_str!("../data/german.txt");

#[test]
fn embedded_words_satisfy_source_contract_and_typing_rules() {
    let words: &dyn WordSource = kairos_wordlists::german();
    assert_eq!(words.len(), 6_909);
    assert!((5_000..=21_000).contains(&words.len()));
    assert!(!words.is_empty());
    let mut seen = HashSet::new();
    for (index, line) in TEXT.lines().enumerate() {
        assert!(pipeline::eligible(line), "invalid word at {index}: {line}");
        assert!(seen.insert(line), "duplicate word: {line}");
        assert_eq!(words.word(index), line);
        assert_eq!(words.language(index), Language::German);
    }
    assert_eq!(seen.len(), words.len());
    assert!(TEXT.ends_with('\n'));
    assert!(!TEXT.contains('\r'));
    assert!(!TEXT.contains("\n\n"));
    assert!(TEXT.lines().zip(TEXT.lines().skip(1)).all(|(a, b)| a < b));
    let mean = seen.iter().map(|word| word.len()).sum::<usize>() as f64 / words.len() as f64;
    assert!((5.5..=6.6).contains(&mean), "mean length: {mean}");
}

#[test]
fn curated_rejections_are_absent() {
    let words: HashSet<_> = TEXT.lines().collect();
    for line in include_str!("../data/rejected.tsv").lines() {
        let (word, reason) = line.split_once('\t').unwrap();
        assert!(pipeline::REASONS.contains(&reason));
        assert!(!words.contains(word), "rejected word returned: {word}");
    }
}

#[test]
fn editorial_inputs_are_sorted_unique_and_categorized() {
    for text in [
        include_str!("../data/supplement.txt"),
        include_str!("../data/rejected.tsv"),
    ] {
        assert!(text.ends_with('\n'));
        assert!(!text.contains("\n\n"));
        let entries: Vec<_> = text
            .lines()
            .map(|line| line.split('\t').next().unwrap())
            .collect();
        assert!(entries.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(entries.iter().all(|word| pipeline::eligible(word)));
    }
    assert!(include_str!("../data/supplement.txt")
        .lines()
        .all(|word| word.len() <= 7));
}

#[test]
fn ending_variants_and_editorial_regressions_are_excluded() {
    let words: HashSet<_> = TEXT.lines().collect();
    for word in &words {
        for ending in ["e", "n", "s", "en", "er", "es", "em", "r"] {
            if let Some(stem) = word.strip_suffix(ending) {
                assert!(!words.contains(stem), "ending neighbors: {stem}/{word}");
            }
        }
    }
    // Independent regression fixtures: changing the curation input alone must
    // not silently undo the review's spelling, neutrality, and safety choices.
    for word in [
        "arsch",
        "asozial",
        "asthma",
        "armbrust",
        "angst",
        "abfeuern",
        "gefeuert",
        "bier",
        "wein",
        "rotwein",
        "portwein",
        "weinlaune",
        "ruine",
        "alpen",
        "abbuk",
        "abhub",
        "aalst",
        "tages",
        "menus",
        "menu",
        "uboot",
        "kupon",
        "fonem",
        "mieze",
        "zofe",
        "first",
        "range",
        "page",
        "type",
        "user",
        "token",
        "login",
        "reset",
        "setup",
        "story",
        "store",
    ] {
        assert!(!words.contains(word), "editorial regression: {word}");
    }
    for neutral in ["atheismus", "glaube", "engel"] {
        assert!(words.contains(neutral), "neutral word removed: {neutral}");
    }
}

#[test]
fn dictionary_validation_preserves_word_case_rules() {
    assert_eq!(pipeline::dictionary_word("Aal").as_deref(), Some("aal"));
    assert_eq!(pipeline::dictionary_word("apfel").as_deref(), Some("apfel"));
    for invalid in ["ROM", "U-Boot", "Menü", "123", "fooBAR", ""] {
        assert!(pipeline::dictionary_word(invalid).is_none());
    }
}

#[test]
fn intoxicant_review_excludes_ambiguous_drug_and_alcohol_terms() {
    let words: HashSet<_> = TEXT.lines().collect();
    let rejections = include_str!("../data/rejected.tsv");
    for word in ["drink", "bowle", "hanf", "prost"] {
        assert!(!words.contains(word), "intoxicant regression: {word}");
        assert!(rejections
            .lines()
            .any(|line| line == format!("{word}\tbelastend")));
    }
    // Ingredients, animals, smoke, instruments, and travel retain ordinary
    // meanings; matching a fragment alone is not an editorial rejection.
    for word in ["hopfen", "malz", "rauch", "trip"] {
        assert!(words.contains(word), "neutral word removed: {word}");
    }
}

#[test]
fn stage_two_german_triage_is_applied_before_embedding() {
    let words: HashSet<_> = TEXT.lines().collect();
    for text in [
        include_str!("../data/languages/stage2-german-triage.tsv"),
        include_str!("../data/languages/stage2-german-followup.tsv"),
    ] {
        for row in text.lines() {
            let fields: Vec<_> = row.split('\t').collect();
            assert_eq!(fields.len(), 5);
            assert!(["A", "B"].contains(&fields[1]));
            match fields[2] {
                "block" => assert!(
                    !words.contains(fields[0]),
                    "German A regression: {}",
                    fields[0]
                ),
                "allow" => assert!(
                    words.contains(fields[0]),
                    "German B regression: {}",
                    fields[0]
                ),
                other => panic!("invalid triage action: {other}"),
            }
        }
    }
}

#[test]
fn embedded_list_matches_verified_offline_pipeline_and_checksum() {
    let build = pipeline::build().unwrap();
    assert_eq!(build.text, TEXT);
    assert_eq!(build.stages.last().unwrap().1, TEXT.lines().count());
    let expected = include_str!("../data/german.sha256").trim();
    assert_eq!(format!("{:x}", Sha256::digest(TEXT.as_bytes())), expected);
}

#[test]
fn shared_source_is_initialized_once_across_threads() {
    let expected = kairos_wordlists::german();
    let handles: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(kairos_wordlists::german))
        .collect();
    for handle in handles {
        assert!(std::ptr::eq(expected, handle.join().unwrap()));
    }
}

#[test]
fn filters_reject_umlauts_sharp_s_punctuation_and_invalid_lengths() {
    for word in [
        "äpfel",
        "öl",
        "tür",
        "straße",
        "a-b",
        "ab",
        "abcdefghij",
        "",
        "Name",
    ] {
        assert!(!pipeline::eligible(word), "unexpectedly accepted {word}");
    }
    for word in ["apfel", "baum", "viereck"] {
        assert!(pipeline::eligible(word));
    }
}

#[test]
#[should_panic]
fn word_rejects_out_of_bounds_index() {
    let words = kairos_wordlists::german();
    words.word(words.len());
}

#[test]
#[should_panic]
fn language_rejects_out_of_bounds_index() {
    let words = kairos_wordlists::german();
    words.language(words.len());
}
