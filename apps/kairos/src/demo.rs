//! Public screenshot fixture, excluded from all release builds.
//!
//! Modelled after: egui https://github.com/emilk/egui/tree/0.37.0/examples

use kairos_core::{
    GenerateError, GeneratedPassword, GeneratorConfig, Segment, SegmentKind, WordSource,
};
use zeroize::Zeroizing;

pub fn generate(
    _source: &dyn WordSource,
    config: &GeneratorConfig,
) -> Result<GeneratedPassword, GenerateError> {
    let separator = super::model::SEPARATORS
        .iter()
        .find(|character| !config.excluded_chars.contains(character))
        .ok_or(GenerateError::InvalidConfig)?;
    let text = format!("Ähre{separator}Wolke7{separator}Kiesel{separator}Möwe");
    let segments = text
        .char_indices()
        .map(|(start, character)| Segment {
            range: start..start + character.len_utf8(),
            kind: if character == *separator {
                SegmentKind::Separator
            } else if character.is_ascii_digit() {
                SegmentKind::Digit
            } else if character.is_uppercase() {
                SegmentKind::Uppercase
            } else {
                SegmentKind::Word
            },
        })
        .collect();
    Ok(GeneratedPassword {
        secret: Zeroizing::new(text),
        segments,
        entropy_bits: 82.0,
    })
}
