#[path = "../tools/languages.rs"]
mod pipeline;

use std::collections::{BTreeMap, BTreeSet};

use kairos_core::{Language, WordSource};
use kairos_wordlists::{combined, english, german, latin, EmbeddedWordList};

const SOURCES: [fn() -> &'static EmbeddedWordList; 4] = [german, english, latin, combined];

#[test]
fn final_lead_decision_excludes_the_seven_confirmed_words() {
    for (source, words) in [
        (german(), &["farbig", "pritsche"][..]),
        (english(), &["lunchbox", "pucker", "joyride"][..]),
        (latin(), &["lascivio", "punctim"][..]),
    ] {
        for word in words {
            assert!(
                !(0..source.len()).any(|index| source.word(index) == *word),
                "final lead exclusion returned: {word}"
            );
            assert!(
                !(0..combined().len()).any(|index| combined().word(index) == *word),
                "final lead exclusion returned in combined source: {word}"
            );
        }
    }
}

#[test]
fn final_lead_decision_preserves_ordinary_b_words() {
    for (source, words) in [
        (german(), &["nimmst", "klage", "kappe"][..]),
        (english(), &["squirrel"][..]),
        (latin(), &["scio"][..]),
    ] {
        for word in words {
            assert!(
                (0..source.len()).any(|index| source.word(index) == *word),
                "final lead B approval removed: {word}"
            );
        }
    }
}

#[test]
fn single_language_sources_have_equal_sizes_and_valid_spelling() {
    assert_eq!(english().len(), 6_895);
    assert_eq!(latin().len(), 6_895);
    assert_eq!(english().len(), latin().len());
    assert!(english().len() <= german().len());
    for (source, language, text, max_mean) in [
        (
            english(),
            Language::English,
            include_str!("../data/languages/english.txt"),
            6.7,
        ),
        (
            latin(),
            Language::Latin,
            include_str!("../data/languages/latin.txt"),
            7.0,
        ),
    ] {
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
    assert_eq!(
        union.len(),
        german().len() + english().len() + latin().len()
    );
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
    assert_eq!(counts, [german().len(), english().len(), latin().len()]);
    assert_eq!(seen.len(), expected.len());
}

#[test]
fn combined_order_is_interleaved_without_language_blocks() {
    let lists = [german(), english(), latin()];
    let interleaved = english().len() * 3;
    for index in 0..interleaved {
        assert_eq!(combined().word(index), lists[index % 3].word(index / 3));
        assert_eq!(
            combined().language(index),
            lists[index % 3].language(index / 3)
        );
    }
    for (offset, index) in (interleaved..combined().len()).enumerate() {
        assert_eq!(combined().language(index), Language::German);
        assert_eq!(
            combined().word(index),
            german().word(english().len() + offset)
        );
    }
    if english().len() == german().len() {
        // Only equal, disjoint vocabularies support the factor 3^4 = 81.
        let gain = 4.0 * (combined().len() as f64 / german().len() as f64).log2();
        assert!((gain - 81_f64.log2()).abs() < 1e-12);
    }
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
    for (source, neutral) in [(english(), "faith"), (latin(), "amo")] {
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

#[test]
fn missing_latin_stems_never_become_words() {
    for grammar in ["ADV X", "N 1 1 F T", "ADJ 1 1 POS", "V 1 1 TRANS"] {
        let row = format!("{:<19}{:<57}{grammar:<24}{:<10}", "zzz", "", "X X X D O");
        assert!(
            pipeline::latin_lemma(&row).is_none(),
            "placeholder accepted: {grammar}"
        );
    }
    for index in 0..latin().len() {
        assert!(!latin().word(index).starts_with("zzz"));
    }
}

#[test]
fn sensitive_latin_meanings_and_reviewed_spellings_are_excluded() {
    for gloss in [
        "buttocks; anus; (rude)",
        "URINATE",
        "commit sodomy with",
        "female pudenda",
        "brothel",
        "pimp",
        "procuress",
        "private parts",
        "genitalia",
        "ejaculate; (somewhat rude)",
        "enema",
        "sausage; (rude word)",
        "cleft; (sometimes rude)",
        "vulgar expression",
        "scrotum",
        "penis",
    ] {
        assert!(
            pipeline::sensitive_gloss(gloss),
            "sensitive gloss missed: {gloss}"
        );
    }
    for harmless in [
        "water; sea, lake",
        "peace; harmony",
        "procure goods; provide",
        "peninsula; promontory",
    ] {
        assert!(!pipeline::sensitive_gloss(harmless));
    }
    let words: BTreeSet<_> = (0..latin().len())
        .map(|index| latin().word(index))
        .collect();
    for word in [
        "culus", "cunnus", "pedico", "percido", "meio", "mingo", "mio", "lupanar", "leno", "lena",
        "natis", "ile", "ilum", "sulcus", "vagina", "semen", "genital", "cluster", "plo", "arcs",
        "apes", "scabies", "ulcero", "suppuro", "pusula", "putesco", "putidus", "tabidus",
    ] {
        assert!(!words.contains(word), "review regression: {word}");
    }
    for row in include_str!("../data/languages/latin-glosses.tsv").lines() {
        // The spelling is not an English gloss (Latin `dies` means day).
        let (_, records) = row.split_once('\t').unwrap();
        assert!(
            !pipeline::sensitive_gloss(records),
            "sensitive selected entry: {row}"
        );
    }
    for word in ["islamist", "marxism"] {
        assert!(!(0..english().len()).any(|index| english().word(index) == word));
    }
}

#[test]
fn disease_injury_decay_and_death_classes_cover_meanings_and_inflections() {
    for gloss in [
        "DISEASE; diseased",
        "sick; sickness; sickening",
        "plagues; plagued; plaguing",
        "pest; pestilence; pestilential",
        "ulceration; ulcerous",
        "sores; soreness",
        "wounds; wounded; wounding",
        "pus",
        "suppurate; suppuration; suppurating",
        "fester; festering",
        "itch; itching; itchy",
        "mange; mangy",
        "scab; scabies; scabby",
        "leprosy; leprous; lepers",
        "rot; rotten; rotting",
        "putrid; putrefying",
        "corpse; corpses; cadaver",
        "death; deaths; dead",
        "die; dies; died; dying",
        "kill; kills; killed; killing",
        "slain; perish",
        "decayed; infection; fever",
        "pustule; abscess; gangrene; necrotic",
        "illness; ailment; infirmity; lesion; purulent; putrescent",
        "hemorrhoids; inflammation; morbid; paralysis; diarrhea; dizzy",
        "tomb; coffin; burial; funerary; end of life; inflamed eyes",
        "prolapse; convulse; tetanus; catarrh; gout; wart; deafness; wasting away",
        "hamstring; hamstrung",
    ] {
        for term in gloss.split(';').map(str::trim) {
            assert!(pipeline::distressing_terms(term), "class missed: {term}");
            assert!(pipeline::sensitive_gloss(term), "gloss missed: {term}");
        }
    }
    for neutral in [
        "rotation; rotate; rotunda; carrot",
        "pestle; tapestry",
        "manger; management",
        "octopus; push; campus",
        "stitch; ditch; kitchen",
        "skill; skillet",
        "audience; diet; soldier",
        "undying love; antivirus; germless; germproof",
        "unwound thread; withered leaf; scratch, scrape",
        "made of wood; boil food; inflamed with love; grave, serious; by decree of fate",
    ] {
        assert!(
            !pipeline::distressing_terms(neutral),
            "fragment false positive: {neutral}"
        );
        assert!(
            !pipeline::sensitive_gloss(neutral),
            "gloss false positive: {neutral}"
        );
    }
}

#[test]
fn verifier_disease_findings_are_caught_in_original_dictionary_records() {
    use std::io::Read;
    let mut raw = flate2::read::GzDecoder::new(
        &include_bytes!("../data/languages/upstream/whitaker/DICTLINE.GEN.gz")[..],
    );
    let mut bytes = Vec::new();
    raw.read_to_end(&mut bytes).unwrap();
    let text: String = bytes.into_iter().map(char::from).collect();
    for word in ["scabies", "ulcero", "suppuro"] {
        let records: Vec<_> = text
            .lines()
            .filter(|line| line.get(..110).and_then(pipeline::latin_lemma).as_deref() == Some(word))
            .collect();
        assert!(!records.is_empty(), "source regression: {word}");
        assert!(
            records
                .iter()
                .any(|line| pipeline::sensitive_gloss(&line[110..])),
            "source meaning escaped guard: {word}"
        );
        assert!(!(0..latin().len()).any(|index| latin().word(index) == word));
    }
}

#[test]
fn english_selection_passes_the_same_disease_classes_as_latin_glosses() {
    let words: BTreeSet<_> = (0..english().len())
        .map(|index| english().word(index))
        .collect();
    for word in &words {
        assert!(
            !pipeline::distressing_terms(word),
            "English class regression: {word}"
        );
    }
    for word in [
        "rot", "scabbed", "scabby", "disease", "sick", "plague", "wound", "death", "kill",
    ] {
        assert!(!words.contains(word), "English review regression: {word}");
    }
    for word in [
        "rotunda",
        "carrot",
        "push",
        "manger",
        "skill",
        "undying",
        "antivirus",
        "unwound",
    ] {
        assert!(words.contains(word), "neutral English word removed: {word}");
    }
    assert!(!words.contains("hamstring"));
    for word in ["dies", "rota", "roto", "rotundus", "pax"] {
        assert!(
            (0..latin().len()).any(|index| latin().word(index) == word),
            "Latin spelling treated as an English gloss: {word}"
        );
    }
}

#[test]
fn identical_first_glosses_do_not_repeat_in_latin_selection() {
    let mut seen = BTreeMap::new();
    for row in include_str!("../data/languages/latin-glosses.tsv").lines() {
        let (word, records) = row.split_once('\t').unwrap();
        let first = records.split(" | ").next().unwrap();
        let meaning = first
            .splitn(3, ' ')
            .nth(2)
            .unwrap()
            .trim_start_matches('|')
            .trim();
        assert!(
            seen.insert(meaning, word).is_none(),
            "duplicate first gloss: {word}"
        );
    }
    let words: BTreeSet<_> = (0..latin().len())
        .map(|index| latin().word(index))
        .collect();
    for group in [
        &["paeniteo", "peniteo", "poeniteo"][..],
        &["neclego", "neglego", "negligo"],
        &["computo", "conputo"],
        &["absilio", "apsilio"],
        &["colostra", "colostrum", "colustra", "colustrum"],
        &["hisopum", "hisopus", "hysopum", "hysopus"],
        &["orfanus", "orphanus"],
        &["praesaepe", "praesepe"],
    ] {
        assert!(group.iter().filter(|word| words.contains(**word)).count() <= 1);
    }
}

#[test]
fn re_verifier_round_two_findings_have_semantic_blocks() {
    let decisions = include_str!("../data/languages/semantic-decisions.tsv");
    for (language, source, words) in [
        ("en", english(), &["dagger"][..]),
        (
            "la",
            latin(),
            &[
                "scrotum", "follis", "motio", "inustum", "tardipes", "viduitas", "hydromel",
                "potor", "merso", "bardus", "morio", "botulus", "hasta", "servula",
            ][..],
        ),
    ] {
        for word in words {
            assert!(
                !(0..source.len()).any(|index| source.word(index) == *word),
                "regression: {language}/{word}"
            );
            assert!(
                decisions
                    .lines()
                    .any(|row| row.starts_with(&format!("{word}\t{language}\tblock\t"))),
                "missing semantic block: {word}"
            );
        }
    }
}

#[test]
fn every_embedded_english_and_latin_word_has_a_source_bound_semantic_approval() {
    let english_sources = pipeline::english_sources().unwrap();
    let decisions: BTreeMap<_, _> = include_str!("../data/languages/semantic-decisions.tsv")
        .lines()
        .map(|row| {
            let fields: Vec<_> = row.split('\t').collect();
            ((fields[1], fields[0]), fields)
        })
        .collect();
    let glosses: BTreeMap<_, _> = include_str!("../data/languages/latin-glosses.tsv")
        .lines()
        .map(|row| row.split_once('\t').unwrap())
        .collect();
    for (language, source) in [("en", english()), ("la", latin())] {
        for index in 0..source.len() {
            let word = source.word(index);
            let decision = &decisions[&(language, word)];
            assert_eq!(
                &decision[2..4],
                &["allow", "neutral"],
                "unapproved {language}/{word}"
            );
            let meaning = if language == "en" {
                &english_sources[word]
            } else {
                glosses[word]
            };
            assert_eq!(
                decision[5],
                pipeline::checksum(meaning.as_bytes()),
                "stale review: {word}"
            );
        }
    }
}

#[test]
fn rubric_b_examples_remain_available_and_slavery_is_rejected() {
    for (source, words) in [
        (english(), &["pouncing", "bladder", "reptile"][..]),
        (latin(), &["absurde", "situs"][..]),
    ] {
        for word in words {
            assert!(
                (0..source.len()).any(|index| source.word(index) == *word),
                "rubric B regression: {word}"
            );
        }
    }
    for gloss in [
        "young female slave",
        "freedman; ex-slave",
        "slavery; bondage",
        "enslaved captive",
        "serf; serfdom",
    ] {
        assert!(pipeline::sensitive_gloss(gloss), "slavery escaped: {gloss}");
    }
    for gloss in ["bladder; knee", "pounce; leap", "absurd", "place; position"] {
        assert!(
            !pipeline::sensitive_gloss(gloss),
            "neutral B gloss: {gloss}"
        );
    }
}

#[test]
fn secondary_senses_distinguish_metaphors_from_sexual_vulgar_and_slur_senses() {
    for gloss in [
        "reptile; a groveling or despised person",
        "snake; a deceitful person",
        "pig; a greedy person",
        "rat; an untrustworthy person",
        "cow; an unpleasant person",
        "worm; a contemptible person",
        "weasel; an evasive person",
    ] {
        assert!(!pipeline::sensitive_gloss(gloss), "B metaphor: {gloss}");
    }
    for gloss in [
        "animal; sexual slang",
        "object; vulgar expression",
        "animal; obscene usage",
        "racial slur",
        "ethnic slur",
        "religious slur",
        "ableist slur",
        "sexist slur",
        "homophobic slur",
        "transphobic slur",
        "slur against people of a particular origin",
    ] {
        assert!(pipeline::sensitive_gloss(gloss), "A sense: {gloss}");
    }
    // Retain the strict A guard for primary meanings.
    for gloss in ["death", "wound", "slavery", "scrotum"] {
        assert!(pipeline::sensitive_gloss(gloss), "A primary sense: {gloss}");
    }
}

#[test]
fn english_meanings_include_secondary_senses_and_expose_coverage_gaps() {
    let sources = pipeline::english_sources().unwrap();
    assert_eq!(sources.len(), 9_558);
    assert!(sources["bladder"].contains("noun:"));
    assert!(sources["pouncing"].contains(":pounce:"));
    assert!(sources["dagger"].contains(":dagger: a short knife"));
    assert!(sources["cat"].contains("[lex=18:noun.person]"));
    assert!(sources["cyber"].contains("Zusatzbeleg 2026-10-25 Sexualitaet:"));
    assert!(sources["maroon"].contains("93472515"));
    assert!(sources.values().any(|source| source.ends_with("NO_ENTRY")));
    let decisions = include_str!("../data/languages/semantic-decisions.tsv");
    assert!(pipeline::semantic_blocks(decisions, "en", &sources).is_ok());
    let mut changed = sources;
    changed
        .get_mut("bladder")
        .unwrap()
        .push_str(" changed secondary meaning");
    assert!(pipeline::semantic_blocks(decisions, "en", &changed).is_err());
}

#[test]
fn existing_conservative_blocks_and_a_families_cannot_reenter_the_lists() {
    for (source, words) in [
        (
            english(),
            &[
                "angler",
                "blabber",
                "cat",
                "copper",
                "crawlers",
                "creature",
                "culprit",
                "deceiver",
                "doormat",
                "exile",
                "flop",
                "fossil",
                "gull",
                "jaywalker",
                "jinx",
                "junior",
                "lamb",
                "lizard",
                "maid",
                "maroon",
                "ostrich",
                "parrot",
                "pretender",
                "pseudo",
                "puppet",
                "pushover",
                "renegade",
                "shark",
                "simple",
                "skirt",
                "sponge",
                "stowaway",
                "trickster",
                "unicorn",
                "cyber",
                "dirtiness",
                "banana",
                "clam",
                "eggplant",
                "fluff",
                "kitty",
                "meat",
                "pearl",
                "salami",
                "starfish",
                "tap",
                "tapping",
                "taps",
                "laxative",
            ][..],
        ),
        (
            latin(),
            &[
                "hera",
                "era",
                "erus",
                "erilis",
                "ipsimus",
                "domina",
                "dominus",
                "famulor",
                "relegatio",
                "exsul",
                "exsulo",
                "extorris",
                "exul",
                "exulo",
                "extermino",
                "dolus",
                "reus",
            ][..],
        ),
    ] {
        for word in words {
            assert!(
                !(0..source.len()).any(|index| source.word(index) == *word),
                "stage-two cause regression: {word}"
            );
        }
    }
}

#[test]
fn person_review_and_evidence_gaps_fail_closed() {
    let meaning = "[lex=18:noun.person] noun:00000000:alpha: an ordinary participant";
    let source = format!("alpha | EFF-large=no | WordNet-3.0 {meaning}");
    let sources = BTreeMap::from([("alpha".to_owned(), source.clone())]);
    let digest = pipeline::checksum(source.as_bytes());
    let review = format!(
        "alpha\tN\tallow\tneutral\tAll person senses read.\t{digest}\tWordNet-3.0 {meaning}\n"
    );
    assert!(pipeline::verify_english_person_review(&sources, &BTreeSet::new(), &review).is_ok());
    assert!(pipeline::verify_english_person_review(&sources, &BTreeSet::new(), "").is_err());
    assert!(pipeline::verify_english_person_review(
        &sources,
        &BTreeSet::new(),
        &review.replace(&digest, &"0".repeat(64)),
    )
    .is_err());
    let mut expanded = sources.clone();
    expanded.insert("beta".to_owned(), source.replace("alpha", "beta"));
    assert!(pipeline::verify_english_person_review(&expanded, &BTreeSet::new(), &review).is_err());
    let gap = "beta | EFF-large=no | WordNet-3.0 NO_ENTRY";
    assert!(!pipeline::english_has_evidence(gap));
    let gap_sources = BTreeMap::from([("beta".to_owned(), gap.to_owned())]);
    assert!(pipeline::verify_english_person_review(&gap_sources, &BTreeSet::new(), "").is_err());
    assert!(pipeline::verify_english_person_review(
        &gap_sources,
        &BTreeSet::from(["beta".to_owned()]),
        ""
    )
    .is_ok());
    assert!(pipeline::english_has_evidence(
        "beta | EFF-large=yes | WordNet-3.0 NO_ENTRY"
    ));
}

#[test]
fn stage_two_exclusions_cover_embedded_words_and_reserves() {
    let decisions = include_str!("../data/languages/semantic-decisions.tsv");
    for row in include_str!("../data/languages/stage2-decisions.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 6);
        let source = match fields[1] {
            "en" => english(),
            "la" => latin(),
            other => panic!("unexpected language: {other}"),
        };
        assert!(["allow", "block"].contains(&fields[3]));
        if fields[3] == "block" {
            assert!(!(0..source.len()).any(|index| source.word(index) == fields[0]));
        } else {
            assert_eq!(fields[2], "B");
            assert_eq!(fields[4], "neutral");
        }
        let prefix = format!(
            "{}\t{}\t{}\t{}\t",
            fields[0], fields[1], fields[3], fields[4]
        );
        assert!(decisions.lines().any(|entry| entry.starts_with(&prefix)));
    }
}

#[test]
fn semantic_review_fails_closed_on_missing_unknown_changed_or_invalid_decisions() {
    let sources = BTreeMap::from([("alpha".to_owned(), "neutral meaning".to_owned())]);
    let digest = pipeline::checksum(b"neutral meaning");
    let valid = format!("alpha\ten\tallow\tneutral\tReviewed neutral meaning.\t{digest}\n");
    assert!(pipeline::semantic_blocks(&valid, "en", &sources)
        .unwrap()
        .is_empty());
    let blocked = valid.replace("allow\tneutral", "block\tZweifel");
    assert!(pipeline::semantic_blocks(&blocked, "en", &sources)
        .unwrap()
        .contains("alpha"));
    for invalid in [
        String::new(),
        valid.replace("alpha", "unknown"),
        valid.replace(&digest, &"0".repeat(64)),
        valid.replace("allow", "maybe"),
        valid.replace("neutral", "Zweifel"),
        valid.replace("Reviewed neutral meaning.", ""),
        format!("{valid}{valid}"),
        blocked.replace("Zweifel", "other"),
    ] {
        assert!(
            pipeline::semantic_blocks(&invalid, "en", &sources).is_err(),
            "accepted: {invalid}"
        );
    }
}
