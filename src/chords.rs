use chord::{Chord, BARRE_FRET1, BARRE_FRET2, BARRE_FRET3};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    Chord::new(&["A"], "x02220", &["A major"], None),
    Chord::new(&["A+"], "x03221", &["A augmented"], None),
    Chord::new(&["Amaj7"], "x02120", &["A major 7ᵗʰ"], None),
    Chord::new(&["A6"], "x02222", &["A 6ᵗʰ"], None),
    Chord::new(&["A7"], "x02020", &["A 7ᵗʰ (shape 1)"], None),
    Chord::new(&["A7"], "x02223", &["A 7ᵗʰ (shape 2)"], None),
    Chord::new(&["A9"], "x02423", &["A 9ᵗʰ"], None),
    Chord::new(&["Am"], "x02210", &["A minor"], None),
    Chord::new(&["Am7"], "x02010", &["A minor 7ᵗʰ"], None),
    Chord::new(&["Asus"], "xx2230", &["A suspended"], None),
    Chord::new(&["Asus2"], "x02200", &["A suspended 2nd"], None),
    Chord::new(&["B"], "xx4442", &["B major"], None),
    Chord::new(&["B+]"], "xx3221", &["B augmented"], Some(BARRE_FRET3)),
    Chord::new(&["Bmaj7"], "22130x", &["B major 7ᵗʰ"], None),
    Chord::new(&["B6"], "224444", &["B 6ᵗʰ"], None),
    Chord::new(&["B7"], "x21202", &["B 7ᵗʰ"], None),
    Chord::new(&["B9"], "x21222", &["B 9ᵗʰ"], None),
    Chord::new(&["Bm"], "224432", &["B minor"], Some(BARRE_FRET2)),
    Chord::new(&["Bm"], "xx4432", &["B minor (no bar)"], None),
    Chord::new(&["Bm7"], "x20202", &["B minor 7ᵗʰ"], None),
    Chord::new(&["Bsus"], "xx3341", &["B suspended"], None),
    Chord::new(&["C"], "x32010", &["C major"], None),
    Chord::new(&["C+"], "x32110", &["C augmented"], None),
    Chord::new(&["Cmaj7"], "x32000", &["C major 7ᵗʰ"], None),
    Chord::new(&["C6"], "x02213", &["C 6ᵗʰ"], None),
    Chord::new(&["C7"], "x32310", &["C 7ᵗʰ"], None),
    Chord::new(&["C9"], "x32333", &["C 9ᵗʰ"], None),
    Chord::new(&["Cm"], "x310xx", &["C minor"], None),
    Chord::new(&["Cm7"], "x313xx", &["C minor 7ᵗʰ"], None),
    Chord::new(&["Csus"], "xx3013", &["C suspended"], None),
    Chord::new(&["D"], "xx0232", &["D major"], None),
    Chord::new(&["D+"], "xx0332", &["D augmented"], None),
    Chord::new(&["Dmaj7"], "xx0222", &["D major 7ᵗʰ"], None),
    Chord::new(&["D6"], "x00202", &["D 6ᵗʰ"], None),
    Chord::new(&["D7"], "xx0212", &["D 7ᵗʰ"], None),
    Chord::new(&["D9"], "200210", &["D 9ᵗʰ"], None),
    Chord::new(&["Dm"], "xx0231", &["D minor"], None),
    Chord {
        short_names: &["Dm7", "F6"],
        pattern: "xx0211",
        names: &["D minor 7ᵗʰ", "F 6ᵗʰ"],
        barre: None,
    },
    Chord::new(&["Dsus"], "xx0233", &["D suspended"], None),
    Chord::new(&["E"], "022100", &["E major"], None),
    Chord::new(&["E+"], "03211x", &["E augmented"], None),
    Chord::new(&["Emaj7"], "021100", &["E major 7ᵗʰ"], None),
    Chord::new(&["E6"], "022120", &["E 6ᵗʰ"], None),
    Chord::new(&["E7"], "020100", &["E 7ᵗʰ (shape 1)"], None),
    Chord::new(&["E7"], "022130", &["E 7ᵗʰ (shape 2)"], None),
    Chord::new(&["E9"], "020102", &["E 9ᵗʰ"], None),
    Chord::new(&["Em"], "022000", &["E minor"], None),
    Chord::new(&["Em7"], "022030", &["E minor 7ᵗʰ"], None),
    Chord {
        short_names: &["Esus", "Esus4"],
        pattern: "022200",
        names: &["E suspended", "E suspended 4ᵗʰ"],
        barre: None,
    },
    Chord::new(&["F"], "133211", &["F major"], Some(BARRE_FRET1)),
    Chord::new(&["F+"], "xx3221", &["F augmented"], None),
    Chord::new(&["Fmaj7"], "xx3210", &["F major 7ᵗʰ"], None),
    Chord::new(&["F7"], "131211", &["F 7ᵗʰ"], None),
    Chord::new(&["F9"], "xx3243", &["F 9ᵗʰ"], None),
    Chord::new(&["Fm"], "133111", &["F minor"], Some(BARRE_FRET1)),
    Chord::new(&["Fm"], "xx3111", &["F minor (no bar)"], None),
    Chord::new(&["Fm7"], "131111", &["F minor 7ᵗʰ"], None),
    Chord::new(&["Fsus"], "xx3311", &["F suspended"], None),
    Chord::new(&["G"], "320003", &["G major"], None),
    Chord::new(&["G+"], "321003", &["G augmented"], None),
    Chord::new(&["Gmaj7"], "3x0002", &["G major 7ᵗʰ"], None),
    Chord::new(&["G6"], "320000", &["G 6ᵗʰ"], None),
    Chord::new(&["G7"], "320001", &["G 7ᵗʰ"], None),
    Chord::new(&["G9"], "300201", &["G 9ᵗʰ"], None),
    Chord::new(&["Gm"], "xx0333", &["G minor"], None),
    Chord::new(&["Gm7"], "x13030", &["G minor 7ᵗʰ"], None),
    Chord::new(&["Gsus"], "xx0013", &["G suspended"], None),
];

pub static ALL_CHORDS_BY_SHORT_NAMES: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            for sn in chord.short_names {
                map.entry(sn.to_ascii_lowercase())
                    .or_default()
                    .push(chord);
            }
        }
        map
    });

#[cfg(test)]
mod tests {
    // NOTE: Useful idiom - importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chord_pattern_length() {
        for chord in ALL_CHORDS {
            assert_eq!(
                chord.pattern.chars().count(),
                6,
                "Guitar has 6 strings. This is a invalid pattern {:?}",
                chord.pattern
            )
        }
    }

    #[test]
    fn test_digit_or_x() {
        for chord in ALL_CHORDS {
            for char in chord.pattern.chars() {
                match char.to_digit(10) {
                    None => assert_eq!(char, 'x', "Only digits or 'x' is allowed"),
                    Some(digit) => assert!(digit < 6),
                }
            }
        }
    }
}
