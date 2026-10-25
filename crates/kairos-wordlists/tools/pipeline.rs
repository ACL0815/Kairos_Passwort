//! Deterministic, offline curation of the vendored German sources.
//!
//! Modelled after: xkcdpass <https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py>.
//! Only the filter-before-generation principle is used; no code was copied.

use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

pub struct Build {
    pub text: String,
    pub stages: Vec<(&'static str, usize)>,
}

pub fn eligible(word: &str) -> bool {
    (3..=9).contains(&word.len()) && word.bytes().all(|byte| byte.is_ascii_lowercase())
}

pub fn dictionary_word(word: &str) -> Option<String> {
    let bytes = word.as_bytes();
    if bytes.first()?.is_ascii_alphabetic() && bytes[1..].iter().all(u8::is_ascii_lowercase) {
        Some(word.to_ascii_lowercase())
    } else {
        None
    }
}

pub const REASONS: [&str; 5] = [
    "vulgär",
    "beleidigend",
    "belastend",
    "Eigenname",
    "Schreibbarkeit",
];

fn read_verified(root: &Path, name: &str) -> Result<String, Box<dyn Error>> {
    let bytes = fs::read(root.join(name))?;
    let raw = if name.ends_with(".gz") {
        let mut raw = Vec::new();
        GzDecoder::new(&bytes[..]).read_to_end(&mut raw)?;
        raw
    } else {
        bytes
    };
    let digest = format!("{:x}", Sha256::digest(&raw));
    let manifest = fs::read_to_string(root.join("upstream/SHA256SUMS"))?;
    let entry = format!("{digest}  {}", name.trim_end_matches(".gz"));
    if !manifest.lines().any(|line| line == entry) {
        return Err(format!("source checksum mismatch: {name}").into());
    }
    Ok(String::from_utf8(raw)?)
}

pub fn build() -> Result<Build, Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let long = read_verified(&root, "upstream/dys2p/de-7776-v1.txt")?;
    let short = read_verified(&root, "upstream/dys2p/de-2048-v1.txt")?;
    let dictionary = read_verified(&root, "upstream/enz/words.gz")?;
    let dictionary_count = dictionary.lines().count();
    let dictionary: BTreeSet<_> = dictionary.lines().filter_map(dictionary_word).collect();
    let mut stages = vec![("dys2p raw", long.lines().count() + short.lines().count())];

    let ascii: Vec<_> = long
        .lines()
        .chain(short.lines())
        .filter(|word| word.bytes().all(|byte| byte.is_ascii_lowercase()))
        .collect();
    stages.push(("dys2p a-z", ascii.len()));
    let sized: Vec<_> = ascii.into_iter().filter(|word| eligible(word)).collect();
    stages.push(("dys2p 3-9 letters", sized.len()));
    let mut words: BTreeSet<String> = sized.into_iter().map(str::to_owned).collect();
    stages.push(("dys2p distinct", words.len()));

    let rejected = fs::read_to_string(root.join("rejected.tsv"))?;
    let mut rejected_words = BTreeSet::new();
    for line in rejected.lines() {
        let (word, reason) = line.split_once('\t').ok_or("invalid rejection entry")?;
        if !REASONS.contains(&reason)
            || (!words.contains(word) && !dictionary.contains(word))
            || !rejected_words.insert(word)
        {
            return Err(format!("invalid or duplicate rejection: {word}").into());
        }
    }
    words.retain(|word| !rejected_words.contains(word.as_str()));
    stages.push(("dys2p curated", words.len()));

    stages.push(("enz raw", dictionary_count));
    let supplement = fs::read_to_string(root.join("supplement.txt"))?;
    let mut count = 0;
    for word in supplement.lines() {
        if !eligible(word)
            || word.len() > 7
            || rejected_words.contains(word)
            || !dictionary.contains(word)
            || !words.insert(word.to_owned())
        {
            return Err(format!("invalid or duplicate supplement: {word}").into());
        }
        count += 1;
    }
    stages.push(("enz selected", count));

    // Conservatively keep the shorter spelling for common ending variants.
    // This is a typing heuristic, not a linguistic claim about shared lemmas.
    let mut ordered: Vec<_> = words.into_iter().collect();
    ordered.sort_by_key(String::len);
    let mut words = BTreeSet::new();
    for word in ordered {
        let near = ["e", "n", "s", "en", "er", "es", "em", "r"]
            .iter()
            .filter_map(|ending| word.strip_suffix(ending))
            .any(|stem| words.contains(stem));
        if !near {
            words.insert(word);
        }
    }
    stages.push(("after ending filter", words.len()));
    stages.push(("final distinct", words.len()));
    let mut text = words.into_iter().collect::<Vec<_>>().join("\n");
    text.push('\n');
    Ok(Build { text, stages })
}
