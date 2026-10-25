#[path = "../tools/pipeline.rs"]
mod pipeline;

use std::collections::HashSet;

use kairos_core::{Language, WordSource};
use sha2::{Digest, Sha256};

const TEXT: &str = include_str!("../data/german.txt");

#[test]
fn embedded_words_satisfy_source_contract_and_typing_rules() {
    let words: &dyn WordSource = kairos_wordlists::german();
    assert_eq!(words.len(), 7_299);
    assert!((7_000..=21_000).contains(&words.len()));
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
        assert!(!reason.is_empty());
        assert!(!words.contains(word), "rejected word returned: {word}");
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
