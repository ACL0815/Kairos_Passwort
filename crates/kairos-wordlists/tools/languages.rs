//! Offline, deterministic curation of the English and Latin data.
//!
//! Modelled after: xkcdpass <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
//! Only filtering and set union before sampling are modelled; no code was copied.
//! Modelled after: Whitaker's WORDS <https://github.com/mk270/whitakers-words/tree/1f2f0fb0867a896d7b9284a03d615ed635d6f992>.
//! Only dictionary data is used, with a deliberately limited lemma decoder.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Build {
    pub english: String,
    pub latin: String,
    pub latin_glosses: String,
    pub report: String,
}

pub fn eligible(word: &str) -> bool {
    (3..=9).contains(&word.len()) && word.bytes().all(|byte| byte.is_ascii_lowercase())
}

pub fn checksum(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read_verified(root: &Path, name: &str, manifest: &str) -> Result<Vec<u8>> {
    let bytes = fs::read(root.join(name))?;
    let raw = if name.ends_with(".gz") {
        let mut raw = Vec::new();
        GzDecoder::new(&bytes[..]).read_to_end(&mut raw)?;
        raw
    } else {
        bytes
    };
    let entry = format!("{}  {}", checksum(&raw), name.trim_end_matches(".gz"));
    if !manifest.lines().any(|line| line == entry) {
        return Err(format!("source checksum mismatch: {name}").into());
    }
    Ok(raw)
}

/// Decodes only regular, unambiguous lemma patterns, never bare word stems.
///
/// WORDS uses 19-byte stem fields and a 24-byte grammatical field. Noun and
/// adjective nominatives and verb first-person present forms are reconstructed
/// from the first stem. Unsupported patterns, proper names, impersonal verbs,
/// capitalized stems, and non-ASCII spellings are deliberately omitted.
pub fn latin_lemma(line: &str) -> Option<String> {
    if !line.is_ascii() || line.len() < 110 {
        return None;
    }
    let stem = line[..19].trim();
    if stem.is_empty() || stem == "zzz" || !stem.bytes().all(|byte| byte.is_ascii_lowercase()) {
        return None;
    }
    let grammar: Vec<_> = line[76..100].split_whitespace().collect();
    let suffix = match grammar.as_slice() {
        ["N", "1", "1", _, kind] if *kind != "N" => "a",
        ["N", "2", "1", _, kind] | ["N", "4", "1", _, kind] if *kind != "N" => "us",
        ["N", "2", "2", _, kind] if *kind != "N" => "um",
        ["N", "3", "1" | "2" | "3" | "4", _, kind] if *kind != "N" => "",
        ["N", "5", "1", _, kind] if *kind != "N" => "es",
        ["ADJ", "1", "1", _] => "us",
        ["ADJ", "3", "1" | "3", _] => "",
        ["ADJ", "3", "2", _] => "is",
        ["V", "1" | "3", "1" | "4", "DEP"] => "or",
        ["V", "2", "1", "DEP"] => "eor",
        ["V", "1" | "3", "1" | "4", "X" | "TRANS" | "INTRANS"] => "o",
        ["V", "2", "1", "X" | "TRANS" | "INTRANS"] => "eo",
        ["ADV", "POS" | "X"] | ["CONJ"] | ["PREP", _] | ["INTERJ"] => "",
        _ => return None,
    };
    let word = format!("{stem}{suffix}");
    eligible(&word).then_some(word)
}

pub fn rejections(text: &str, candidates: &BTreeSet<String>) -> Result<BTreeSet<String>> {
    let mut rejected = BTreeSet::new();
    let mut previous = "";
    for line in text.lines() {
        let (word, reason) = line.split_once('\t').ok_or("invalid rejection entry")?;
        if ![
            "vulgär",
            "beleidigend",
            "belastend",
            "Eigenname",
            "Schreibbarkeit",
        ]
        .contains(&reason)
            || !candidates.contains(word)
            || word <= previous
        {
            return Err(format!("invalid, unordered, or duplicate rejection: {word}").into());
        }
        rejected.insert(word.to_owned());
        previous = word;
    }
    Ok(rejected)
}

/// Exact, source-bound semantic decisions for the entire eligible candidate pool.
/// Unknown words, missing reviews and changed secondary meanings fail closed.
pub fn semantic_blocks(
    text: &str,
    language: &str,
    sources: &BTreeMap<String, String>,
) -> Result<BTreeSet<String>> {
    let mut seen = BTreeSet::new();
    let mut blocked = BTreeSet::new();
    let mut previous = None;
    for row in text.lines() {
        let fields: Vec<_> = row.split('\t').collect();
        if fields.len() != 6 {
            return Err("semantic decision needs six fields".into());
        }
        let [word, lang, decision, class, reason, fingerprint] = fields[..] else {
            unreachable!()
        };
        let key = (lang, word);
        if previous.is_some_and(|prev| prev >= key)
            || !["en", "la"].contains(&lang)
            || !["allow", "block"].contains(&decision)
            || reason.trim().is_empty()
            || fingerprint.len() != 64
            || !fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
            || (decision == "allow" && class != "neutral")
            || (decision == "block"
                && !class.split(',').all(|c| {
                    [
                        "Krankheit",
                        "Verletzung",
                        "Tod-Trauer",
                        "Gewalt-Waffen",
                        "Sexualitaet",
                        "Ausscheidung",
                        "Drogen-Alkohol",
                        "Abwertung",
                        "Vulgaer",
                        "Slur",
                        "Sklaverei-Unterdrueckung",
                        "Verbrechen",
                        "Herabsetzung-Extremismus",
                        "Zweifel",
                    ]
                    .contains(&c)
                }))
        {
            return Err(format!("invalid semantic decision: {lang}/{word}").into());
        }
        previous = Some(key);
        if lang != language {
            continue;
        }
        if sources
            .get(word)
            .map(|source| checksum(source.as_bytes()))
            .as_deref()
            != Some(fingerprint)
        {
            return Err(format!("missing or changed semantic source: {lang}/{word}").into());
        }
        seen.insert(word.to_owned());
        if decision == "block" {
            blocked.insert(word.to_owned());
        }
    }
    if seen.len() != sources.len() {
        return Err(format!(
            "incomplete semantic review: {language}: {}/{}",
            seen.len(),
            sources.len()
        )
        .into());
    }
    Ok(blocked)
}

/// Rejects explicit sexual/vulgar/slur senses and conservative sensitive markers.
/// Ordinary animal/object metaphors alone do not force A under the lead rubric.
/// Existing stricter blocks remain; this is not a general semantic classifier.
pub fn sensitive_gloss(gloss: &str) -> bool {
    let gloss = gloss.to_ascii_lowercase();
    distressing_terms(&gloss)
        || gloss
            .split(|ch: char| !ch.is_ascii_alphabetic())
            .any(|token| {
                [
                    "scrotum",
                    "testes",
                    "penis",
                    "phallus",
                    "slave",
                    "slaves",
                    "slavery",
                    "enslave",
                    "enslaved",
                    "enslaving",
                    "bondslave",
                    "bondman",
                    "bondwoman",
                    "serf",
                    "serfdom",
                ]
                .contains(&token)
            })
        || [
            "(rude)",
            "(somewhat rude)",
            "rude word",
            "rude term",
            "sometimes rude",
            "rude expression",
            "vulgar",
            "obscene",
            "racial slur",
            "ethnic slur",
            "religious slur",
            "ableist slur",
            "sexist slur",
            "homophobic slur",
            "transphobic slur",
            "slur against",
            "genital",
            "urinat",
            "sodomy",
            "anal intercourse",
            "brothel",
            "pimp",
            "procuress",
            "procurer",
            "private parts",
            "buttocks",
            "pudend",
            "ejaculat",
            "sexual",
            "coitus",
            "enema",
            "excrement",
            "prostitut",
            "testicle",
        ]
        .iter()
        .any(|marker| gloss.contains(marker))
}

/// Shared disease, injury, decay, and death classes for English spellings and
/// English dictionary glosses. Match complete tokens or deliberate inflection
/// families, never fragments such as `rot` in `rotation` or `pus` in `push`.
pub fn distressing_terms(text: &str) -> bool {
    const TERMS: &[&str] = &[
        "sick",
        "sicks",
        "sickly",
        "sickness",
        "sicknesses",
        "plague",
        "plagues",
        "plagued",
        "plaguing",
        "pest",
        "pests",
        "pestilence",
        "pestilences",
        "pestilent",
        "pestilential",
        "pestiferous",
        "sore",
        "sores",
        "soreness",
        "wound",
        "wounds",
        "wounded",
        "wounding",
        "pus",
        "mange",
        "mangy",
        "scab",
        "scabs",
        "scabbed",
        "scabbing",
        "scabby",
        "scabies",
        "leprosy",
        "leprous",
        "leper",
        "lepers",
        "rot",
        "rots",
        "rotted",
        "rotting",
        "rotten",
        "corpse",
        "corpses",
        "cadaver",
        "cadavers",
        "cadaverous",
        "carcass",
        "carcasses",
        "carcase",
        "carcases",
        "death",
        "deaths",
        "dead",
        "deadly",
        "die",
        "dies",
        "died",
        "dying",
        "kill",
        "kills",
        "killed",
        "killing",
        "killer",
        "killers",
        "slay",
        "slays",
        "slaying",
        "slain",
        "slew",
        "perish",
        "perishes",
        "perished",
        "perishing",
        "decay",
        "decays",
        "decayed",
        "decaying",
        "infect",
        "infects",
        "infected",
        "infecting",
        "infection",
        "infections",
        "infectious",
        "fever",
        "fevers",
        "feverish",
        "febrile",
        "cancer",
        "cancers",
        "cancerous",
        "tumor",
        "tumors",
        "tumour",
        "tumours",
        "pox",
        "smallpox",
        "abscess",
        "abscesses",
        "gangrene",
        "gangrenous",
        "necrosis",
        "necrotic",
        "ill",
        "illness",
        "illnesses",
        "ail",
        "ails",
        "ailed",
        "ailing",
        "infirmity",
        "infirmities",
        "debility",
        "lameness",
        "lesion",
        "lesions",
        "carbuncle",
        "carbuncles",
        "blain",
        "blains",
        "scurf",
        "ringworm",
        "eczema",
        "inflammation",
        "inflammations",
        "contagion",
        "contagious",
        "epidemic",
        "epidemics",
        "cholera",
        "rabies",
        "rabid",
        "palsy",
        "apoplexy",
        "insane",
        "insanely",
        "insanity",
        "demented",
        "dementia",
        "madness",
        "injury",
        "injuries",
        "injure",
        "injures",
        "injured",
        "injuring",
        "hurt",
        "hurts",
        "hurting",
        "bruise",
        "bruises",
        "bruised",
        "bruising",
        "abrasion",
        "abrasions",
        "fracture",
        "fractures",
        "fractured",
        "trauma",
        "traumatic",
        "contusion",
        "contusions",
        "blood",
        "bleeding",
        "cripple",
        "crippled",
        "cripples",
        "maim",
        "maimed",
        "maiming",
        "spasm",
        "spasms",
        "convulsion",
        "convulsions",
        "seizure",
        "seizures",
        "dizzy",
        "dizziness",
        "giddiness",
        "numbness",
        "mildew",
        "blight",
        "blighted",
        "mortal",
        "mortality",
        "mortally",
        "mortuary",
        "fatal",
        "fatality",
        "fatalities",
        "fatally",
        "expire",
        "expires",
        "expired",
        "expiring",
        "bury",
        "buries",
        "buried",
        "burying",
        "burial",
        "burials",
        "tomb",
        "tombs",
        "tombstone",
        "tombstones",
        "coffin",
        "coffins",
        "bier",
        "funeral",
        "funerals",
        "funerary",
        "funereal",
        "hamstring",
        "hamstrings",
        "hamstringing",
        "hamstrung",
        "gout",
        "catarrh",
        "tetanus",
        "wart",
        "warts",
        "bilious",
        "prolapse",
        "prolapsed",
        "convulse",
        "convulsed",
        "convulsing",
        "numb",
        "blindness",
        "deafness",
        "swollen",
        "swelling",
    ];
    const FAMILIES: &[&str] = &[
        "diseas",
        "sicken",
        "ulcer",
        "suppurat",
        "fester",
        "itch",
        "putrid",
        "putref",
        "pustul",
        "ailment",
        "malady",
        "maladies",
        "purulen",
        "putresc",
        "decompos",
        "carrion",
        "morbid",
        "deceas",
        "slaughter",
        "assassin",
        "murder",
        "hemorr",
        "haemorr",
        "typh",
        "dysent",
        "diarr",
        "tubercul",
        "paraly",
        "vomit",
        "nausea",
        "emaciat",
        "sepulcr",
        "sepulchr",
        "entomb",
    ];
    let text = text.to_ascii_lowercase();
    text.split(|ch: char| !ch.is_ascii_alphabetic())
        .any(|token| TERMS.contains(&token) || FAMILIES.iter().any(|stem| token.starts_with(stem)))
        || ["inflamed eyes", "pass away", "end of life", "wasting away"]
            .iter()
            .any(|phrase| text.contains(phrase))
}
// The records are produced locally as "source-line frequency meaning".
fn meaning(record: &str) -> &str {
    record
        .splitn(3, ' ')
        .nth(2)
        .unwrap_or("")
        .trim_start_matches('|')
        .trim()
}

fn rank_words(words: &mut [(u8, String)]) {
    words.sort_by(|(rank_a, a), (rank_b, b)| (rank_a, a.len(), a).cmp(&(rank_b, b.len(), b)));
}

fn output(words: impl IntoIterator<Item = String>) -> String {
    let words: BTreeSet<_> = words.into_iter().collect();
    words.into_iter().map(|word| word + "\n").collect()
}

/// Pinned WordNet definitions and explicit EFF Large membership, including gaps.
/// The extract is independently reproducible with prepare-english-senses.mjs.
pub fn english_sources() -> Result<BTreeMap<String, String>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/languages/upstream");
    let manifest = fs::read_to_string(root.join("SHA256SUMS"))?;
    for name in ["wordnet/LICENSE", "wordnet/WordNet-3.0.tar.gz"] {
        read_verified(&root, name, &manifest)?;
    }
    let raw = String::from_utf8(read_verified(
        &root,
        "wordnet/candidate-senses.tsv.gz",
        &manifest,
    )?)?;
    let mut sources = BTreeMap::new();
    let mut previous = "";
    for row in raw.lines() {
        let (word, meanings) = row.split_once('\t').ok_or("invalid English source row")?;
        if !eligible(word)
            || word <= previous
            || !meanings.starts_with(&format!("{word} | EFF-large="))
            || !meanings.contains(" | WordNet-3.0 ")
            || meanings.contains('\t')
        {
            return Err(format!("invalid or unordered English source: {word}").into());
        }
        sources.insert(word.to_owned(), meanings.to_owned());
        previous = word;
    }
    let evidence = fs::read_to_string(root.join("../english-extra-evidence.tsv"))?;
    let mut extra_words = BTreeSet::new();
    for row in evidence.lines() {
        let fields: Vec<_> = row.split('\t').collect();
        if fields.len() != 5 || !extra_words.insert(fields[0]) {
            return Err("invalid or duplicate additional English evidence".into());
        }
        let expected = format!(
            " | Zusatzbeleg {} {}: {} ({})",
            fields[1], fields[2], fields[3], fields[4]
        );
        if !sources
            .get(fields[0])
            .is_some_and(|source| source.ends_with(&expected))
        {
            return Err(format!("additional English evidence is stale: {}", fields[0]).into());
        }
    }
    for (word, source) in &sources {
        if source.contains(" | Zusatzbeleg ") && !extra_words.contains(word.as_str()) {
            return Err(format!("unaccounted additional English evidence: {word}").into());
        }
    }
    Ok(sources)
}

/// EFF membership is a positive signal, not an A/B verdict.
pub fn english_has_evidence(source: &str) -> bool {
    !source.contains("WordNet-3.0 NO_ENTRY") || source.contains(" | EFF-large=yes | ")
}

/// Every allowed noun.person association needs a source-bound explicit reading.
pub fn verify_english_person_review(
    sources: &BTreeMap<String, String>,
    blocked: &BTreeSet<String>,
    review: &str,
) -> Result<()> {
    let mut seen = BTreeSet::new();
    let mut previous = "";
    for row in review.lines() {
        let fields: Vec<_> = row.split('\t').collect();
        if fields.len() != 7 {
            return Err("invalid English person review row".into());
        }
        let word = fields[0];
        let source = sources
            .get(word)
            .ok_or("unknown English person review word")?;
        let meanings = source
            .split(" | ")
            .filter(|meaning| meaning.contains(":noun.person]"))
            .collect::<Vec<_>>()
            .join(" | ");
        if word <= previous
            || !seen.insert(word)
            || meanings.is_empty()
            || fields[6] != meanings
            || fields[5] != checksum(source.as_bytes())
            || fields[4].trim().is_empty()
            || !matches!(
                (fields[1], fields[2]),
                ("A", "block") | ("B", "block" | "allow") | ("N", "allow")
            )
            || (fields[2] == "block") != blocked.contains(word)
        {
            return Err(format!("invalid or stale English person review: {word}").into());
        }
        previous = word;
    }
    for (word, source) in sources {
        if !blocked.contains(word)
            && source.contains(":noun.person]")
            && !seen.contains(word.as_str())
        {
            return Err(format!("unreviewed English person meaning: {word}").into());
        }
        if !blocked.contains(word) && !english_has_evidence(source) {
            return Err(
                format!("English word has neither WordNet nor EFF Large evidence: {word}").into(),
            );
        }
    }
    Ok(())
}

fn select(
    mut words: Vec<(u8, String)>,
    excluded: &BTreeSet<String>,
    size: usize,
) -> Result<String> {
    words.retain(|(_, word)| !excluded.contains(word));
    // Common Latin lemmas first; English ranks are equal. Prefer short words
    // within each frequency band, with an ordinal tie break for reproducibility.
    rank_words(&mut words);
    if words.len() < size {
        return Err(format!("only {} curated words, need {size}", words.len()).into());
    }
    Ok(output(words.into_iter().take(size).map(|(_, word)| word)))
}

pub fn build() -> Result<Build> {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let root = data.join("languages");
    let upstream = root.join("upstream");
    let manifest = fs::read_to_string(upstream.join("SHA256SUMS"))?;
    // Verify notices as well as dictionary bytes, offline.
    for name in [
        "eff/LICENSE.txt",
        "eff/COPYRIGHT-POLICY.txt",
        "whitaker/LICENCE.txt",
        "whitaker/README.md",
        "wordnik/LICENSE",
        "wordnik/README.md",
    ] {
        read_verified(&upstream, name, &manifest)?;
    }
    let german = fs::read_to_string(data.join("german.txt"))?;
    if checksum(german.as_bytes()) != fs::read_to_string(data.join("german.sha256"))?.trim() {
        return Err("German checksum mismatch; rebuild and review PR-2 data first".into());
    }
    let german: BTreeSet<String> = german.lines().map(str::to_owned).collect();
    let german_size = german.len();
    let mut report = format!("German retained: {german_size}\n");
    report.push_str(&format!(
        "Acceptance rubric A/B sha256: {}\n",
        checksum(&fs::read(data.join("curation-rubric.md"))?)
    ));
    let decisions = fs::read_to_string(root.join("semantic-decisions.tsv"))?;
    report.push_str(&format!(
        "Semantic decisions sha256: {}\n",
        checksum(decisions.as_bytes())
    ));

    let mut english = BTreeSet::new();
    let mut raw_count = 0;
    for (name, digits, count) in [
        ("eff/large.txt", 5, 7_776),
        ("eff/short1.txt", 4, 1_296),
        ("eff/short2.txt", 4, 1_296),
    ] {
        let raw = String::from_utf8(read_verified(&upstream, name, &manifest)?)?;
        if raw.lines().count() != count {
            return Err(format!("unexpected source size: {name}").into());
        }
        for line in raw.lines() {
            let (roll, word) = line.split_once('\t').ok_or("invalid EFF row")?;
            if roll.len() != digits || !roll.bytes().all(|byte| (b'1'..=b'6').contains(&byte)) {
                return Err("invalid dice prefix".into());
            }
            raw_count += 1;
            if eligible(word) {
                english.insert(word.to_owned());
            }
        }
    }
    report.push_str(&format!(
        "English raw rows: {raw_count}\nEnglish eligible distinct: {}\n",
        english.len()
    ));
    let dictionary = String::from_utf8(read_verified(
        &upstream,
        "wordnik/wordlist-20210729.txt.gz",
        &manifest,
    )?)?;
    let dictionary: BTreeSet<_> = dictionary
        .lines()
        .filter_map(|line| line.strip_prefix('"')?.strip_suffix('"'))
        .collect();
    let supplement = fs::read_to_string(root.join("english-supplement.txt"))?;
    let mut previous = "";
    for word in supplement.lines() {
        if !eligible(word)
            || word.len() > 7
            || !dictionary.contains(word)
            || word <= previous
            || !english.insert(word.to_owned())
        {
            return Err(
                format!("invalid, duplicate, or unordered English supplement: {word}").into(),
            );
        }
        previous = word;
    }
    report.push_str(&format!(
        "English Wordnik selected additions: {}\n",
        supplement.lines().count()
    ));
    let sources = english_sources()?;
    if sources.keys().cloned().collect::<BTreeSet<_>>() != english {
        return Err("English meaning extract does not cover the exact candidate pool".into());
    }
    report.push_str(&format!(
        "English WordNet covered: {}; explicit no-entry: {}; EFF Large positive signals: {}\n",
        sources
            .values()
            .filter(|source| !source.contains("WordNet-3.0 NO_ENTRY"))
            .count(),
        sources
            .values()
            .filter(|source| source.contains("WordNet-3.0 NO_ENTRY"))
            .count(),
        sources
            .values()
            .filter(|source| source.contains(" | EFF-large=yes | "))
            .count()
    ));
    let semantic = semantic_blocks(&decisions, "en", &sources)?;
    let person_review = fs::read_to_string(root.join("stage2-person-senses.tsv"))?;
    verify_english_person_review(&sources, &semantic, &person_review)?;
    report.push_str(&format!(
        "English person meanings review: {} candidates; sha256: {}\n",
        person_review.lines().count(),
        checksum(person_review.as_bytes())
    ));
    report.push_str(&format!(
        "English semantic decisions: {}; blocked: {}\n",
        english.len(),
        semantic.len()
    ));
    let rejected = rejections(
        &fs::read_to_string(root.join("english-rejected.tsv"))?,
        &english,
    )?;
    report.push_str(&format!(
        "English editorial rejections: {}\n",
        rejected.len()
    ));
    english.retain(|word| !rejected.contains(word));
    let before_semantic = english.len();
    english.retain(|word| !semantic.contains(word));
    report.push_str(&format!(
        "English additional semantic blocks: {}\n",
        before_semantic - english.len()
    ));
    let before_guard = english.len();
    english.retain(|word| !distressing_terms(word));
    report.push_str(&format!(
        "English disease/injury/decay/death guard removed: {}\n",
        before_guard - english.len()
    ));
    let overlap = english.intersection(&german).count();
    report.push_str(&format!("English overlap removed (German): {overlap}\nEnglish available after curation and overlap removal: {}\n", english.len() - overlap));
    let size = german_size.min(english.len() - overlap);
    let english = select(
        english.into_iter().map(|word| (0, word)).collect(),
        &german,
        size,
    )?;

    let raw = read_verified(&upstream, "whitaker/DICTLINE.GEN.gz", &manifest)?;
    // The vendored dictionary is Latin-1. The fixed-width lemma and frequency
    // fields are ASCII; meanings are not used by the decoder.
    let raw: String = raw.into_iter().map(char::from).collect();
    let mut latin = BTreeMap::new();
    let mut glosses: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (index, line) in raw.lines().enumerate() {
        // Non-ASCII gloss characters do not invalidate ASCII grammatical fields.
        let Some(prefix) = line.get(..110) else {
            continue;
        };
        if let Some(word) = latin_lemma(prefix) {
            let frequency = prefix.as_bytes()[106];
            // A rare secondary sense still belongs to the same spelling.
            glosses.entry(word.clone()).or_default().push(format!(
                "{} {} {}",
                index + 1,
                char::from(frequency),
                &line[110..]
            ));
            if (b'A'..=b'D').contains(&frequency) {
                latin
                    .entry(word)
                    .and_modify(|rank: &mut u8| *rank = (*rank).min(frequency))
                    .or_insert(frequency);
            }
        }
    }
    report.push_str(&format!("Latin raw rows: {}\nLatin eligible distinct (frequency A-D, supported lemma patterns): {}\n", raw.lines().count(), latin.len()));
    let semantic = semantic_blocks(
        &decisions,
        "la",
        &latin
            .keys()
            .map(|word| (word.clone(), glosses[word].join(" | ")))
            .collect(),
    )?;
    report.push_str(&format!(
        "Latin semantic decisions: {}; blocked: {}\n",
        latin.len(),
        semantic.len()
    ));
    let rejected = rejections(
        &fs::read_to_string(root.join("latin-rejected.tsv"))?,
        &latin.keys().cloned().collect(),
    )?;
    report.push_str(&format!("Latin editorial rejections: {}\n", rejected.len()));
    latin.retain(|word, _| !rejected.contains(word));
    let before_semantic = latin.len();
    latin.retain(|word, _| !semantic.contains(word));
    report.push_str(&format!(
        "Latin additional semantic blocks: {}\n",
        before_semantic - latin.len()
    ));
    let before_guard = latin.len();
    latin.retain(|word, _| {
        !["vagina", "semen", "genital"].contains(&word.as_str())
            && !glosses[word]
                .iter()
                .any(|record| sensitive_gloss(meaning(record)))
    });
    report.push_str(&format!(
        "Latin sensitive-gloss/spelling guard removed: {}\n",
        before_guard - latin.len()
    ));
    let english_set: BTreeSet<_> = english.lines().map(str::to_owned).collect();
    let de_overlap = latin.keys().filter(|word| german.contains(*word)).count();
    let en_overlap = latin
        .keys()
        .filter(|word| english_set.contains(*word))
        .count();
    let excluded: BTreeSet<_> = german.union(&english_set).cloned().collect();
    report.push_str(&format!("Latin overlap removed (German): {de_overlap}\nLatin overlap removed (English): {en_overlap}\nLatin available after curation and overlap removal: {}\n", latin.len() - de_overlap - en_overlap));
    let mut latin: Vec<_> = latin
        .into_iter()
        .filter(|(word, _)| !excluded.contains(word))
        .map(|(word, rank)| (rank, word))
        .collect();
    rank_words(&mut latin);
    // Identical complete first glosses conservatively identify spelling variants.
    // Keep the highest-frequency, then shortest, then ordinal spelling. This can
    // also discard genuine synonyms; no linguistic equivalence is claimed.
    let before_variants = latin.len();
    let mut seen_glosses = BTreeSet::new();
    latin.retain(|(_, word)| seen_glosses.insert(meaning(&glosses[word][0])));
    report.push_str(&format!(
        "Latin identical-gloss variants removed: {}\nLatin available after variant filter: {}\n",
        before_variants - latin.len(),
        latin.len()
    ));
    let final_size = size.min(latin.len());
    if final_size == 0 {
        return Err("semantic curation left an empty language pool".into());
    }
    // Keep the overlap exclusions made against the larger English preselection:
    // reducing the size never restores a spelling that was already excluded.
    let english = select(
        english.lines().map(|word| (0, word.to_owned())).collect(),
        &BTreeSet::new(),
        final_size,
    )?;
    if final_size < german_size {
        report.push_str(&format!("Equal EN/LA size reduced to {final_size}: minimum of German target, reviewed English pool and Latin pool after overlap/variant filtering. German unchanged; Latin overlap exclusions retain the English preselection of {size}.\n"));
    }
    let size = final_size;
    let latin = select(latin, &BTreeSet::new(), size)?;
    let latin_glosses = latin
        .lines()
        .map(|word| format!("{word}\t{}\n", glosses[word].join(" | ")))
        .collect();
    for (label, text) in [("English", &english), ("Latin", &latin)] {
        let total: usize = text.lines().map(str::len).sum();
        report.push_str(&format!(
            "{label} final: {size}; mean length: {:.3}; sha256: {}\n",
            total as f64 / size as f64,
            checksum(text.as_bytes())
        ));
    }
    report.push_str(&format!("Combined distinct: {}\n", german_size + size * 2));
    Ok(Build {
        english,
        latin,
        latin_glosses,
        report,
    })
}

pub fn verify_output(name: &str, actual: &str) -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/languages");
    if fs::read_to_string(root.join(format!("{name}.txt")))? != actual
        || fs::read_to_string(root.join(format!("{name}.sha256")))?.trim()
            != checksum(actual.as_bytes())
    {
        return Err(
            format!("{name} output or approved checksum differs; rebuild and review").into(),
        );
    }
    Ok(())
}
