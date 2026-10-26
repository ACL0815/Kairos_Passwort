//! Password state and UTF-8 highlighting, independent of the native window.
//!
//! Modelled after: KeePassXC https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/PasswordWidget.cpp
//! The GUI retains one current result and invalidates its assessment after editing.

use eframe::egui::{text::LayoutJob, Color32, FontId, Stroke, TextFormat};
use kairos_core::{GenerateError, GeneratedPassword, GeneratorConfig, Segment, SegmentKind};
use zeroize::{Zeroize, Zeroizing};

pub const MAX_SECRET_CHARS: usize = 4096;
const SECRET_BUFFER_BYTES: usize = MAX_SECRET_CHARS * 4;

pub const SEPARATORS: [char; 20] = [
    '-', '+', '.', ',', '#', '*', '!', '?', '_', ':', ';', '/', '(', ')', '\'', '"', '$', '%', '&',
    '=',
];

#[derive(Clone, Copy, PartialEq)]
pub enum Assessment {
    Unavailable,
    Generated(f64),
    Edited,
}

pub struct PasswordState {
    pub secret: Zeroizing<String>,
    pub segments: Vec<Segment>,
    pub assessment: Assessment,
    pub message: &'static str,
    block_candidates: Vec<Zeroizing<String>>,
}

impl Default for PasswordState {
    fn default() -> Self {
        Self {
            secret: Zeroizing::new(String::with_capacity(SECRET_BUFFER_BYTES)),
            segments: Vec::new(),
            assessment: Assessment::Unavailable,
            message: "Generator und deutsche Wortliste werden vorbereitet.",
            block_candidates: Vec::new(),
        }
    }
}

impl PasswordState {
    /// Offers original dictionary spellings only for an unedited generation.
    pub fn block_candidates(&self) -> &[Zeroizing<String>] {
        &self.block_candidates
    }

    pub fn set_result(&mut self, result: Result<GeneratedPassword, GenerateError>) {
        self.clear();
        match result {
            Ok(result) => {
                if result.secret.chars().count() > MAX_SECRET_CHARS
                    || result.secret.chars().any(char::is_control)
                {
                    self.message = "Das erzeugte Passwort ist zu lang oder enthält nicht darstellbare Steuerzeichen.";
                    return;
                }
                // Keep the editor allocation; the core's temporary is zeroized on drop.
                self.secret.push_str(&result.secret);
                self.segments = result.segments;
                self.assessment = if result.entropy_bits.is_finite() && result.entropy_bits >= 0.0 {
                    Assessment::Generated(result.entropy_bits)
                } else {
                    Assessment::Unavailable
                };
                self.message = "Zum Abschreiben farbig hervorgehoben. Direkt im Feld bearbeitbar.";
                if matches!(self.assessment, Assessment::Generated(_)) {
                    for part in self.secret.split(|c: char| !c.is_alphanumeric()) {
                        let mut word = Zeroizing::new(
                            part.chars()
                                .filter(|c| c.is_alphabetic())
                                .collect::<String>(),
                        );
                        word.make_ascii_lowercase();
                        if (3..=9).contains(&word.len())
                            && word.bytes().all(|byte| byte.is_ascii_lowercase())
                        {
                            self.block_candidates.push(word);
                        }
                    }
                    self.block_candidates
                        .sort_by(|a, b| a.as_str().cmp(b.as_str()));
                    self.block_candidates
                        .dedup_by(|a, b| a.as_str() == b.as_str());
                }
            }
            Err(error) => {
                self.message = match error {
                    GenerateError::NotImplemented => "Der Generator ist noch nicht verfügbar. Die Oberfläche ist bereit; Generator und deutsche Wortliste werden noch vorbereitet.",
                    GenerateError::EmptySource => "Noch keine Wörter verfügbar. Bitte zuerst eine Wortliste einbinden.",
                    GenerateError::InvalidConfig => "Diese Auswahl ist nicht möglich. Bitte weniger Trenner ausschließen.",
                    GenerateError::RandomSourceUnavailable => "Windows konnte keinen sicheren Zufall liefern. Bitte erneut versuchen.",
                    _ => "Das Passwort konnte nicht sicher erzeugt werden. Bitte erneut versuchen.",
                };
            }
        }
    }

    pub fn edited(&mut self) {
        self.block_candidates.clear();
        self.segments.clear();
        self.assessment = Assessment::Edited;
        self.message = "bearbeitet – nicht mehr bewertbar";
    }

    pub fn clear(&mut self) {
        self.block_candidates.clear();
        self.secret.zeroize();
        self.segments.clear();
        self.assessment = Assessment::Unavailable;
    }
}

pub fn apply_exclusions(config: &mut GeneratorConfig, excluded: &[bool; 20]) {
    config.excluded_chars = SEPARATORS
        .iter()
        .zip(excluded)
        .filter_map(|(&character, &is_excluded)| is_excluded.then_some(character))
        .collect();
}

pub fn segment_color(kind: SegmentKind) -> Color32 {
    match kind {
        SegmentKind::Word => Color32::from_rgb(234, 242, 255),
        SegmentKind::Digit => Color32::from_rgb(251, 191, 36),
        SegmentKind::Separator => Color32::from_rgb(244, 114, 208),
        SegmentKind::Uppercase => Color32::from_rgb(103, 232, 249),
    }
}

/// Uses core byte ranges without interpreting them as character offsets.
///
/// Modelled after: egui https://github.com/emilk/egui/blob/0.37.0/crates/egui_demo_lib/src/demo/code_editor.rs
pub fn password_layout(text: &str, segments: &[Segment], size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    let mut offset = 0;
    let valid = segments.iter().all(|segment| {
        let valid = segment.range.start >= offset
            && segment.range.start < segment.range.end
            && text.get(segment.range.clone()).is_some();
        offset = segment.range.end;
        valid
    });
    let mut append = |part: &str, kind| {
        if part.is_empty() {
            return;
        }
        job.append(
            part,
            0.0,
            TextFormat {
                font_id: FontId::monospace(size),
                color: segment_color(kind),
                underline: if kind == SegmentKind::Uppercase {
                    Stroke::new(1.0, segment_color(kind))
                } else {
                    Stroke::NONE
                },
                ..Default::default()
            },
        );
    };
    offset = 0;
    if valid && !segments.is_empty() {
        for segment in segments {
            append(&text[offset..segment.range.start], SegmentKind::Word);
            append(&text[segment.range.clone()], segment.kind);
            offset = segment.range.end;
        }
        append(&text[offset..], SegmentKind::Word);
    } else {
        for (start, character) in text.char_indices() {
            let kind = if character.is_ascii_digit() {
                SegmentKind::Digit
            } else if character.is_uppercase() {
                SegmentKind::Uppercase
            } else if !character.is_alphanumeric() {
                SegmentKind::Separator
            } else {
                SegmentKind::Word
            };
            append(&text[start..start + character.len_utf8()], kind);
        }
    }
    job
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn generated() -> GeneratedPassword {
        GeneratedPassword {
            secret: Zeroizing::new("Ähre-7öl".into()),
            segments: vec![
                Segment {
                    kind: SegmentKind::Uppercase,
                    range: 0..2,
                },
                Segment {
                    kind: SegmentKind::Word,
                    range: 2..5,
                },
                Segment {
                    kind: SegmentKind::Separator,
                    range: 5..6,
                },
                Segment {
                    kind: SegmentKind::Digit,
                    range: 6..7,
                },
                Segment {
                    kind: SegmentKind::Word,
                    range: 7..10,
                },
            ],
            entropy_bits: 72.5,
        }
    }

    #[test]
    fn every_semantic_role_has_a_distinct_color() {
        let colors: HashSet<_> = [
            SegmentKind::Word,
            SegmentKind::Digit,
            SegmentKind::Separator,
            SegmentKind::Uppercase,
        ]
        .map(segment_color)
        .into_iter()
        .collect();
        assert_eq!(colors.len(), 4);
    }

    #[test]
    fn utf8_byte_segments_preserve_text_and_colors() {
        let result = generated();
        let layout = password_layout(&result.secret, &result.segments, 24.0);
        assert_eq!(layout.text, "Ähre-7öl");
        assert_eq!(layout.sections[0].byte_range.start.0, 0);
        assert_eq!(layout.sections[0].byte_range.end.0, 2);
        assert_eq!(
            layout.sections[3].format.color,
            segment_color(SegmentKind::Digit)
        );
    }

    #[test]
    fn malformed_ranges_fall_back_without_panicking_or_losing_text() {
        for (start, end) in [(1, 2), (0, 100), (5, 3)] {
            let range = start..end;
            let layout = password_layout(
                "Ähre",
                &[Segment {
                    kind: SegmentKind::Digit,
                    range,
                }],
                24.0,
            );
            assert_eq!(layout.text, "Ähre");
            assert_eq!(
                layout.sections[0].format.color,
                segment_color(SegmentKind::Uppercase)
            );
        }
    }

    #[test]
    fn editing_invalidates_entropy_even_if_user_later_restores_text() {
        let mut state = PasswordState::default();
        state.set_result(Ok(generated()));
        assert!(matches!(state.assessment, Assessment::Generated(72.5)));
        state.edited();
        assert!(matches!(state.assessment, Assessment::Edited));
        assert!(state.segments.is_empty());
        assert_eq!(state.message, "bearbeitet – nicht mehr bewertbar");
        state.set_result(Ok(generated()));
        assert!(matches!(state.assessment, Assessment::Generated(72.5)));
    }

    #[test]
    fn errors_remove_previous_secret_and_assessment() {
        let mut state = PasswordState::default();
        state.set_result(Ok(generated()));
        state.set_result(Err(GenerateError::NotImplemented));
        assert!(state.secret.is_empty());
        assert!(matches!(state.assessment, Assessment::Unavailable));
        assert!(state.message.contains("noch nicht verfügbar"));
    }

    #[test]
    fn nonfinite_entropy_is_never_presented_as_strength() {
        let mut result = generated();
        result.entropy_bits = f64::NAN;
        let mut state = PasswordState::default();
        state.set_result(Ok(result));
        assert!(matches!(state.assessment, Assessment::Unavailable));
    }

    #[test]
    fn all_twenty_exclusions_map_once_in_canonical_order() {
        let mut config = GeneratorConfig::default();
        apply_exclusions(&mut config, &[true; 20]);
        assert_eq!(config.excluded_chars, SEPARATORS);
        assert_eq!(
            config.excluded_chars.iter().collect::<HashSet<_>>().len(),
            20
        );
        let mut excluded = [false; 20];
        excluded[0] = true;
        excluded[19] = true;
        apply_exclusions(&mut config, &excluded);
        assert_eq!(config.excluded_chars, ['-', '=']);
        apply_exclusions(&mut config, &[false; 20]);
        assert!(config.excluded_chars.is_empty());
    }

    #[test]
    fn generated_results_and_maximum_utf8_edits_keep_the_reserved_allocation() {
        let mut state = PasswordState::default();
        let pointer = state.secret.as_ptr();
        let capacity = state.secret.capacity();
        state.set_result(Ok(generated()));
        assert_eq!(state.secret.as_ptr(), pointer);
        assert_eq!(state.secret.capacity(), capacity);
        state.clear();
        state.secret.push_str(&"🦀".repeat(MAX_SECRET_CHARS));
        state.edited();
        assert_eq!(state.secret.as_ptr(), pointer);
        assert_eq!(state.secret.capacity(), capacity);
        state.set_result(Ok(generated()));
        assert_eq!(state.secret.as_ptr(), pointer);
    }

    #[test]
    fn unsafe_or_oversized_core_results_are_not_displayed() {
        for text in ["a\nb".to_owned(), "x".repeat(MAX_SECRET_CHARS + 1)] {
            let mut result = generated();
            result.secret = Zeroizing::new(text);
            let mut state = PasswordState::default();
            state.set_result(Ok(result));
            assert!(state.secret.is_empty());
            assert!(matches!(state.assessment, Assessment::Unavailable));
        }
    }
}
