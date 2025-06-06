use chord::Chord;
use chord::{BARRE_FRET1, BARRE_FRET2};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    Chord {
        short_name: "A",
        pattern: "x02220",
        name: "A major",
        barre: None,
    },
    Chord {
        short_name: "A+",
        pattern: "x03221",
        name: "A augmented",
        barre: None,
    },
    Chord {
        short_name: "Amaj7",
        pattern: "x02120",
        name: "A major 7th",
        barre: None,
    },
    Chord {
        short_name: "A6",
        pattern: "x02222",
        name: "A 6th",
        barre: None,
    },
    Chord {
        short_name: "A7",
        pattern: "x02020",
        name: "A 7th (shape 1)",
        barre: None,
    },
    Chord {
        short_name: "A7",
        pattern: "x02223",
        name: "A 7th (shape 2)",
        barre: None,
    },
    Chord {
        short_name: "A9",
        pattern: "x02423",
        name: "A 9th",
        barre: None,
    },
    Chord {
        short_name: "Am",
        pattern: "x02210",
        name: "A minor",
        barre: None,
    },
    Chord {
        short_name: "Am7",
        pattern: "x02010",
        name: "A minor 7th",
        barre: None,
    },
    Chord {
        short_name: "Asus2",
        pattern: "x02200",
        name: "A suspended 2nd",
        barre: None,
    },
    Chord {
        short_name: "B",
        pattern: "xx4442",
        name: "B major",
        barre: None,
    },
    Chord {
        short_name: "B+",
        pattern: "xx3221",
        name: "B augmented",
        barre: None,
    },
    Chord {
        short_name: "Bmaj7",
        pattern: "22130x",
        name: "B major 7th",
        barre: None,
    },
    Chord {
        short_name: "B6",
        pattern: "224444",
        name: "B 6th",
        barre: None,
    },
    Chord {
        short_name: "B7",
        pattern: "x21202",
        name: "B 7th",
        barre: None,
    },
    Chord {
        short_name: "B9",
        pattern: "x21222",
        name: "B 9th",
        barre: None,
    },
    Chord {
        short_name: "Bm",
        pattern: "224432",
        name: "B minor",
        barre: Some(BARRE_FRET2),
    },
    Chord {
        short_name: "Bm",
        pattern: "xx4432",
        name: "B minor (no bar)",
        barre: None,
    },
    Chord {
        short_name: "Bm7",
        pattern: "x20202",
        name: "B minor 7th",
        barre: None,
    },
    Chord {
        short_name: "C",
        pattern: "x32010",
        name: "C major",
        barre: None,
    },
    Chord {
        short_name: "C+",
        pattern: "x32110",
        name: "C augmented",
        barre: None,
    },
    Chord {
        short_name: "Cmaj7",
        pattern: "x32000",
        name: "C major 7th",
        barre: None,
    },
    Chord {
        short_name: "C6",
        pattern: "x02213",
        name: "C 6th",
        barre: None,
    },
    Chord {
        short_name: "C7",
        pattern: "x32310",
        name: "C 7th",
        barre: None,
    },
    Chord {
        short_name: "C9",
        pattern: "x32333",
        name: "C 9th",
        barre: None,
    },
    Chord {
        short_name: "Cm",
        pattern: "x310xx",
        name: "C minor",
        barre: None,
    },
    Chord {
        short_name: "Cm7",
        pattern: "x313xx",
        name: "C minor 7th",
        barre: None,
    },
    Chord {
        short_name: "D",
        pattern: "xx0232",
        name: "D major",
        barre: None,
    },
    Chord {
        short_name: "D+",
        pattern: "xx0332",
        name: "D augmented",
        barre: None,
    },
    Chord {
        short_name: "Dmaj7",
        pattern: "xx0222",
        name: "D major 7th",
        barre: None,
    },
    Chord {
        short_name: "D6",
        pattern: "x00202",
        name: "D 6th",
        barre: None,
    },
    Chord {
        short_name: "D7",
        pattern: "xx0212",
        name: "D 7th",
        barre: None,
    },
    Chord {
        short_name: "D9",
        pattern: "200210",
        name: "D 9th",
        barre: None,
    },
    Chord {
        short_name: "Dm",
        pattern: "xx0231",
        name: "D minor",
        barre: None,
    },
    Chord {
        short_name: "Dm7",
        pattern: "xx0211",
        name: "D minor 7th",
        barre: None,
    },
    Chord {
        short_name: "E",
        pattern: "022100",
        name: "E major",
        barre: None,
    },
    Chord {
        short_name: "E+",
        pattern: "03211x",
        name: "E augmented",
        barre: None,
    },
    Chord {
        short_name: "Emaj7",
        pattern: "021100",
        name: "E major 7th",
        barre: None,
    },
    Chord {
        short_name: "E6",
        pattern: "022120",
        name: "E 6th",
        barre: None,
    },
    Chord {
        short_name: "E7",
        pattern: "020100",
        name: "E 7th (shape 1)",
        barre: None,
    },
    Chord {
        short_name: "E7",
        pattern: "022130",
        name: "E 7th (shape 2)",
        barre: None,
    },
    Chord {
        short_name: "E9",
        pattern: "020102",
        name: "E 9th",
        barre: None,
    },
    Chord {
        short_name: "Em",
        pattern: "022000",
        name: "E minor",
        barre: None,
    },
    Chord {
        short_name: "Em7",
        pattern: "022030",
        name: "E minor 7th",
        barre: None,
    },
    Chord {
        short_name: "Esus4",
        pattern: "022200",
        name: "E suspended 4th",
        barre: None,
    },
    Chord {
        short_name: "F",
        pattern: "133211",
        name: "F major",
        barre: Some(BARRE_FRET1),
    },
    Chord {
        short_name: "F+",
        pattern: "xx3221",
        name: "F augmented",
        barre: None,
    },
    Chord {
        short_name: "Fmaj7",
        pattern: "xx3210",
        name: "F major 7th",
        barre: None,
    },
    Chord {
        short_name: "F6",
        pattern: "xx0211",
        name: "F 6th",
        barre: None,
    },
    Chord {
        short_name: "F7",
        pattern: "131211",
        name: "F 7th",
        barre: None,
    },
    Chord {
        short_name: "F9",
        pattern: "xx3243",
        name: "F 9th",
        barre: None,
    },
    Chord {
        short_name: "Fm",
        pattern: "133111",
        name: "F minor",
        barre: Some(BARRE_FRET1),
    },
    Chord {
        short_name: "Fm",
        pattern: "xx3111",
        name: "F minor (no bar)",
        barre: None,
    },
    Chord {
        short_name: "Fm7",
        pattern: "131111",
        name: "F minor 7th",
        barre: None,
    },
    Chord {
        short_name: "G",
        pattern: "320003",
        name: "G major",
        barre: None,
    },
    Chord {
        short_name: "G+",
        pattern: "321003",
        name: "G augmented",
        barre: None,
    },
    Chord {
        short_name: "Gmaj7",
        pattern: "3x0002",
        name: "G major 7th",
        barre: None,
    },
    Chord {
        short_name: "G6",
        pattern: "320000",
        name: "G 6th",
        barre: None,
    },
    Chord {
        short_name: "G7",
        pattern: "320001",
        name: "G 7th",
        barre: None,
    },
    Chord {
        short_name: "G9",
        pattern: "300201",
        name: "G 9th",
        barre: None,
    },
    Chord {
        short_name: "Gm",
        pattern: "xx0333",
        name: "G minor",
        barre: None,
    },
    Chord {
        short_name: "Gm7",
        pattern: "x13030",
        name: "G minor 7th",
        barre: None,
    },
];

pub static ALL_CHORDS_BY_SHORT_NAME: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            map.entry(chord.short_name.to_owned().to_ascii_lowercase())
                .or_default()
                .push(chord);
        }

        map
    });

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
