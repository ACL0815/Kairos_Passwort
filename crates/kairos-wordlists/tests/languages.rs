#[path = "../tools/languages.rs"]
mod pipeline;

use std::collections::{BTreeMap, BTreeSet};

use kairos_core::{Language, WordSource};
use kairos_wordlists::{combined, english, german, latin, EmbeddedWordList};

const SOURCES: [fn() -> &'static EmbeddedWordList; 4] = [german, english, latin, combined];

#[test]
fn single_language_sources_have_equal_sizes_and_valid_spelling() {
    for (source, language, text, max_mean) in [
        (
            english(),
            Language::English,
            include_str!("../data/languages/english.txt"),
            6.5,
        ),
        (
            latin(),
            Language::Latin,
            include_str!("../data/languages/latin.txt"),
            6.7,
        ),
    ] {
        assert_eq!(source.len(), 7_134);
        assert!(!source.is_empty());
        let mut seen = BTreeSet::new();
        for (index, word) in text.lines().enumerate() {
            assert!(pipeline::eligible(word), "invalid spelling: {word}");
            assert!(seen.insert(word), "duplicate: {word}");
            assert_eq!(source.word(index), word);
            assert_eq!(source.language(index), language);
        }
        assert_eq!(seen.len(), source.len());
        assert!(text.ends_with('\n'));
        assert!(!text.contains('\r'));
        assert!(!text.contains("\n\n"));
        assert!(text.lines().zip(text.lines().skip(1)).all(|(a, b)| a < b));
        let mean = seen.iter().map(|word| word.len()).sum::<usize>() as f64 / seen.len() as f64;
        assert!((5.5..=max_mean).contains(&mean), "mean: {mean}");
    }
}

#[test]
fn all_spelling_sets_are_pairwise_disjoint_and_union_keeps_every_flag() {
    let mut expected = BTreeMap::new();
    for source in [german(), english(), latin()] {
        for index in 0..source.len() {
            assert!(
                expected
                    .insert(source.word(index), source.language(index))
                    .is_none(),
                "cross-language overlap: {}",
                source.word(index)
            );
        }
    }
    let union: &dyn WordSource = combined();
    assert_eq!(union.len(), 21_402);
    let mut seen = BTreeSet::new();
    let mut counts = [0; 3];
    for index in 0..union.len() {
        let word = union.word(index);
        assert!(seen.insert(word));
        assert_eq!(expected.get(word), Some(&union.language(index)));
        match union.language(index) {
            Language::German => counts[0] += 1,
            Language::English => counts[1] += 1,
            Language::Latin => counts[2] += 1,
            other => panic!("unexpected language: {other:?}"),
        }
    }
    assert_eq!(counts, [7_134; 3]);
    assert_eq!(seen.len(), expected.len());
}

#[test]
fn combined_order_is_interleaved_without_language_blocks() {
    let lists = [german(), english(), latin()];
    for index in 0..combined().len() {
        assert_eq!(combined().word(index), lists[index % 3].word(index / 3));
        assert_eq!(
            combined().language(index),
            lists[index % 3].language(index / 3)
        );
    }
    // Equal, disjoint vocabularies support the concept's factor 3^4 = 81.
    let gain = 4.0 * (combined().len() as f64 / german().len() as f64).log2();
    assert!((gain - 81_f64.log2()).abs() < 1e-12);
}

#[test]
fn pipeline_checks_every_source_notice_output_and_report_offline() {
    let build = pipeline::build().unwrap();
    pipeline::verify_output("english", &build.english).unwrap();
    pipeline::verify_output("latin", &build.latin).unwrap();
    assert_eq!(build.report, include_str!("../data/languages/report.txt"));
    assert_eq!(
        build.latin_glosses,
        include_str!("../data/languages/latin-glosses.tsv")
    );
}

#[test]
fn editorial_decisions_are_valid_and_cannot_silently_restore_regressions() {
    for (text, source) in [
        (
            include_str!("../data/languages/english-rejected.tsv"),
            english(),
        ),
        (
            include_str!("../data/languages/latin-rejected.tsv"),
            latin(),
        ),
    ] {
        assert!(text.ends_with('\n'));
        let words: BTreeSet<_> = (0..source.len()).map(|index| source.word(index)).collect();
        for row in text.lines() {
            assert!(!words.contains(row.split_once('\t').unwrap().0));
        }
    }
    // These fixtures are independent of the versioned editorial decisions.
    for word in [
        "douche", "badass", "cancer", "suicide", "war", "wine", "nintendo", "xbox",
    ] {
        assert!(
            !(0..english().len()).any(|i| english().word(i) == word),
            "English regression: {word}"
        );
    }
    for word in [
        "cum", "anus", "sex", "cancer", "merda", "mors", "vinum", "abnueo", "adnililo",
    ] {
        assert!(
            !(0..latin().len()).any(|i| latin().word(i) == word),
            "Latin regression: {word}"
        );
    }
    for (source, neutral) in [
        (english(), "faith"),
        (english(), "marxism"),
        (english(), "islamist"),
        (latin(), "amo"),
    ] {
        assert!((0..source.len()).any(|i| source.word(i) == neutral));
    }
}

#[test]
fn latin_decoder_recovers_known_lemmas_and_rejects_unsupported_forms() {
    let mut raw = flate2::read::GzDecoder::new(
        &include_bytes!("../data/languages/upstream/whitaker/DICTLINE.GEN.gz")[..],
    );
    use std::io::Read;
    let mut bytes = Vec::new();
    raw.read_to_end(&mut bytes).unwrap();
    let text: String = bytes.into_iter().map(char::from).collect();
    for (stem, grammar, expected) in [
        ("ros", "N      1 1", "rosa"),
        ("bon", "ADJ    1 1", "bonus"),
        ("civis", "N      3 3", "civis"),
        ("fort", "ADJ    3 2", "fortis"),
        ("am", "V      1 1", "amo"),
        ("vid", "V      2 1", "video"),
        ("audi", "V      3 4", "audio"),
    ] {
        let line = text
            .lines()
            .find(|line| {
                line.len() >= 110 && line[..19].trim() == stem && line[76..100].starts_with(grammar)
            })
            .unwrap();
        assert_eq!(
            pipeline::latin_lemma(&line[..110]).as_deref(),
            Some(expected)
        );
    }
    for invalid in ["", "aqua", "ä", &" ".repeat(110)] {
        assert!(pipeline::latin_lemma(invalid).is_none());
    }
    for word in ["a-b", "élan", "ABC", "ab", "abcdefghij", ""] {
        assert!(!pipeline::eligible(word));
    }
}

#[test]
fn invalid_editorial_decisions_fail_instead_of_weakening_filters() {
    let candidates = BTreeSet::from(["alpha".to_owned(), "beta".to_owned()]);
    for invalid in [
        "alpha",
        "alpha\tother\n",
        "missing\tbelastend\n",
        "alpha\tbelastend\nalpha\tbelastend\n",
        "beta\tEigenname\nalpha\tbelastend\n",
    ] {
        assert!(pipeline::rejections(invalid, &candidates).is_err());
    }
    assert!(pipeline::rejections("alpha\tSchreibbarkeit\nbeta\tbelastend\n", &candidates).is_ok());
}

#[test]
fn all_sources_reject_invalid_indices_for_both_methods() {
    for constructor in SOURCES {
        let source = constructor();
        assert!(std::panic::catch_unwind(|| source.word(source.len())).is_err());
        assert!(std::panic::catch_unwind(|| source.language(source.len())).is_err());
    }
}

#[test]
fn all_sources_are_shared_across_threads() {
    for constructor in SOURCES {
        let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(constructor)).collect();
        for handle in handles {
            assert!(std::ptr::eq(constructor(), handle.join().unwrap()));
        }
    }
}
