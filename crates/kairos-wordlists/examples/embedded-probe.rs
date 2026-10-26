//! Minimal release artifact for the offline embedded-wordlist scan.
//!
//! Does not embed test fixtures or print dictionary words. Run the matching
//! `tools/check-embedded.ps1` scanner outside this process.

use kairos_core::WordSource;

fn main() {
    let words = kairos_wordlists::combined();
    for index in 0..words.len() {
        std::hint::black_box(words.word(index));
    }
    println!("{}", words.len());
}
