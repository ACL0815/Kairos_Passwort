//! Public wordlist data masked at build time, checked before indexing.
//!
//! Modelled after: CasualX/obfstr
//! <https://github.com/CasualX/obfstr/blob/9e56c1938dfd3bd65fe305fbafcb57394c701356/src/bytes.rs>.
//! Only storage/runtime separation is modelled. No implementation code copied.
//! Both mask and SHA-256 are in the executable: neither is a trust anchor.

use std::sync::OnceLock;

use sha2::{Digest, Sha256};

struct EmbeddedData {
    encoded: &'static [u8],
    mask: &'static [u8],
    sha256: [u8; 32],
    text: OnceLock<String>,
}

include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

impl EmbeddedData {
    fn text(&'static self, label: &str) -> &'static str {
        self.text.get_or_init(|| {
            decode(self.encoded, self.mask, &self.sha256).unwrap_or_else(|error| {
                panic!("Embedded word list integrity failure ({label}): {error}")
            })
        })
    }
}

fn decode(encoded: &[u8], mask: &[u8], expected: &[u8; 32]) -> Result<String, &'static str> {
    if encoded.len() != mask.len() {
        return Err("masked data length mismatch");
    }
    // Keep the optimizer from folding embedded data back into plaintext.
    // The mask is deliberately recoverable; black_box is not a security boundary.
    let mask = std::hint::black_box(mask);
    let plaintext: Vec<_> = encoded
        .iter()
        .zip(mask)
        .map(|(byte, key)| byte ^ key)
        .collect();
    if Sha256::digest(&plaintext).as_slice() != expected {
        return Err("SHA-256 mismatch; embedded word list may have been modified");
    }
    String::from_utf8(plaintext).map_err(|_| "embedded word list is not valid UTF-8")
}

pub(super) fn german() -> &'static str {
    GERMAN.text(obfstr::obfstr!("kairos.wordlists.de.sha256.v1"))
}

pub(super) fn english() -> &'static str {
    ENGLISH.text(obfstr::obfstr!("kairos.wordlists.en.sha256.v1"))
}

pub(super) fn latin() -> &'static str {
    LATIN.text(obfstr::obfstr!("kairos.wordlists.la.sha256.v1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_output_roundtrips_exact_reviewed_plaintext_and_hashes() {
        for (data, original, hash) in [
            (
                &GERMAN,
                include_str!("../data/german.txt"),
                include_str!("../data/german.sha256"),
            ),
            (
                &ENGLISH,
                include_str!("../data/languages/english.txt"),
                include_str!("../data/languages/english.sha256"),
            ),
            (
                &LATIN,
                include_str!("../data/languages/latin.txt"),
                include_str!("../data/languages/latin.sha256"),
            ),
        ] {
            assert_ne!(data.encoded, original.as_bytes());
            assert_eq!(
                decode(data.encoded, data.mask, &data.sha256).unwrap(),
                original
            );
            assert_eq!(format!("{:x}", Sha256::digest(original)), hash.trim());
        }
    }

    #[test]
    fn build_masks_change_most_plaintext_positions() {
        // Catch regressions such as a zero-filled mask with just one changed
        // byte. This is a coarse sanity check, not an RNG quality/security test.
        for data in [&GERMAN, &ENGLISH, &LATIN] {
            let original = decode(data.encoded, data.mask, &data.sha256).unwrap();
            let changed = data
                .encoded
                .iter()
                .zip(original.as_bytes())
                .filter(|(encoded, plain)| encoded != plain)
                .count();
            assert!(changed > data.encoded.len() * 9 / 10);
        }
    }

    #[test]
    fn changed_data_mask_and_checksum_are_rejected_before_indexing() {
        for data in [&GERMAN, &ENGLISH, &LATIN] {
            for position in [0, data.encoded.len() / 2, data.encoded.len() - 1] {
                let mut encoded = data.encoded.to_vec();
                encoded[position] ^= 1;
                assert!(decode(&encoded, data.mask, &data.sha256)
                    .unwrap_err()
                    .starts_with("SHA-256 mismatch"));
                let mut mask = data.mask.to_vec();
                mask[position] ^= 1;
                assert!(decode(data.encoded, &mask, &data.sha256)
                    .unwrap_err()
                    .starts_with("SHA-256 mismatch"));
            }
            let mut wrong_hash = data.sha256;
            wrong_hash[0] ^= 1;
            assert!(decode(data.encoded, data.mask, &wrong_hash).is_err());
            assert_eq!(
                decode(&data.encoded[1..], data.mask, &data.sha256).unwrap_err(),
                "masked data length mismatch"
            );
        }
    }

    #[test]
    #[should_panic(expected = "Embedded word list integrity failure (test-list): SHA-256 mismatch")]
    fn corrupt_embedded_list_panics_clearly() {
        static BAD: EmbeddedData = EmbeddedData {
            encoded: b"abc",
            mask: b"xyz",
            sha256: [0; 32],
            text: OnceLock::new(),
        };
        BAD.text("test-list");
    }

    #[test]
    fn decoded_text_is_shared_across_threads() {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                std::thread::spawn(|| {
                    [german(), english(), latin()]
                        .map(str::as_ptr)
                        .map(|ptr| ptr as usize)
                })
            })
            .collect();
        let expected = [german(), english(), latin()]
            .map(str::as_ptr)
            .map(|ptr| ptr as usize);
        for handle in handles {
            assert_eq!(handle.join().unwrap(), expected);
        }
    }
}
