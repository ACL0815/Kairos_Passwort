//! Concept references and comparisons; generator entropy always comes from core.

/// Fixed references from both profile tables in docs/Grundkonzept.md.
/// Model: 7,000 / 21,000 words, six letters per word. Never a runtime estimate.
pub const CONCEPT_BITS: [f64; 13] = [
    51.1, 57.4, 64.4, 72.0, 74.1, 75.8, 77.1, 78.0, 82.0, 102.0, 122.0, 131.5, 153.1,
];

pub fn decimal(value: f64) -> String {
    format!("{value:.1}").replace('.', ",")
}

/// Log2 of the number of length-n ASCII passwords containing all four classes.
/// Inclusion-exclusion for 26 lowercase, 26 uppercase, 10 digits, 32 punctuation.
fn ascii_bits(length: u32) -> f64 {
    let classes = [26.0, 26.0, 10.0, 32.0];
    let mut probability = 0.0;
    for mask in 0_u32..16 {
        let missing: f64 = classes
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, size)| size)
            .sum();
        let term = (1.0 - missing / 94.0_f64).powf(length as f64);
        probability += if mask.count_ones().is_multiple_of(2) {
            term
        } else {
            -term
        };
    }
    length as f64 * 94.0_f64.log2() + probability.log2()
}

/// Shortest uniformly random alternative, not an estimate for human passwords.
pub fn equivalent_characters(bits: f64) -> Option<u32> {
    if !bits.is_finite() || !(0.0..=4096.0).contains(&bits) {
        return None;
    }
    (4..=1024).find(|&length| ascii_bits(length) >= bits)
}

/// G50 model (2^H / 2), formatted without constructing a potentially huge power.
/// For conservative min-entropy this is a comparison/lower bound, not an exact
/// measured median of the generator's potentially nonuniform distribution.
pub fn g50(bits: f64) -> Option<String> {
    if !bits.is_finite() || bits < 0.0 {
        return None;
    }
    let logarithm = (bits - 1.0) * std::f64::consts::LOG10_2;
    let mut exponent = logarithm.floor() as i32;
    let mut mantissa = 10.0_f64.powf(logarithm - exponent as f64);
    if (mantissa * 100.0).round() >= 1000.0 {
        mantissa = 1.0;
        exponent += 1;
    }
    Some(format!(
        "{} × 10{}",
        format!("{mantissa:.2}").replace('.', ","),
        superscript(exponent)
    ))
}

fn superscript(exponent: i32) -> String {
    exponent
        .to_string()
        .chars()
        .map(|c| match c {
            '-' => '⁻',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!("integer digits"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_ascii_alternatives_match_both_concept_tables() {
        for (bits, expected) in CONCEPT_BITS
            .into_iter()
            .zip([8, 9, 10, 12, 12, 12, 12, 12, 13, 16, 19, 21, 24])
        {
            assert_eq!(equivalent_characters(bits), Some(expected));
            assert!(ascii_bits(expected) >= bits);
            assert!(ascii_bits(expected - 1) < bits);
        }
        assert!((ascii_bits(8) - 51.3).abs() < 0.05);
        assert!((ascii_bits(24) - 157.2).abs() < 0.05);
    }

    #[test]
    fn g50_uses_the_given_bits_and_german_scientific_notation() {
        assert_eq!(g50(8.0).as_deref(), Some("1,28 × 10²"));
        assert_eq!(g50(51.1).as_deref(), Some("1,21 × 10¹⁵"));
        assert_eq!(g50(153.1).as_deref(), Some("6,12 × 10⁴⁵"));
        assert_eq!(g50(0.0).as_deref(), Some("5,00 × 10⁻¹"));
        // The concept's G50 cells use unrounded source values; the displayed
        // reference 51.1 is rounded and must not be mapped to a hardcoded cell.
        assert_eq!(g50(4096.0).as_deref(), Some("5,22 × 10¹²³²"));
        for invalid in [f64::NAN, f64::INFINITY, -1.0] {
            assert_eq!(g50(invalid), None);
            assert_eq!(equivalent_characters(invalid), None);
        }
    }

    #[test]
    fn g50_matches_all_concept_rows_with_their_documented_rounding() {
        let references = [
            1.20e15, 9.72e16, 1.22e19, 2.43e21, 9.96e21, 3.36e22, 7.97e22, 1.56e23, 2.49e24,
            2.61e30, 2.64e36, 1.92e39, 6.15e45,
        ];
        for (bits, reference) in CONCEPT_BITS.into_iter().zip(references) {
            let formatted = g50(bits).unwrap();
            let (mantissa, exponent) = formatted.split_once(" × 10").unwrap();
            let mantissa: f64 = mantissa.replace(',', ".").parse().unwrap();
            let exponent: u32 = exponent.chars().fold(0, |value, c| {
                value * 10 + "⁰¹²³⁴⁵⁶⁷⁸⁹".chars().position(|digit| digit == c).unwrap() as u32
            });
            let actual = mantissa * 10.0_f64.powf(exponent as f64);
            // Concept G50 cells use unrounded H; the displayed H is rounded to
            // 0.1 bit (up to ~3.5% difference), plus G50's three-digit rounding.
            assert!(
                (actual / reference - 1.0).abs() < 0.04,
                "{bits}: {formatted}"
            );
        }
    }
}
