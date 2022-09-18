#[derive(Debug)]
pub struct Barre {
    // Guitar normally call the string with the highest pitch, or the thinnest,
    // the 1st string; and the thickest, lowest-pitched string, the 6th string.
    // BUT, we are counting the string from left to right as 0 to 5 here.
    pub from_string: u8,
    pub to_string: u8,

    pub fret: u16,
}

#[derive(Debug)]
pub struct Chord<'a> {
    pub short_name: &'a str,
    // Require to be length of 6.
    // 'x' indicates that the string should be muted.
    // '0' indicates that playing the string as it (without finger placement).
    // '1' indicates that we place a finger on the first fret, '2' on the 2nd fret, and etc.
    pub pattern: &'a str,
    pub name: &'a str,
    pub barre: Option<Barre>,
}

const FRETBOARD: &str = "◯ ◯ ◯ ◯ ◯ ◯
╒═╤═╤═╤═╤═╕
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘";

impl Chord<'_> {
    pub fn fretboard(&self) -> String {
        let mut board: Vec<char> = FRETBOARD.chars().collect();

        if let Some(barre) = &self.barre {
            for i in barre.from_string..barre.to_string * 2 + 1 {
                board[i as usize + 24 * barre.fret as usize] = '-'
            }
        }

        for (i, ch) in self.pattern.chars().enumerate() {
            let idx: usize = i * 2;
            if ch == 'x' {
                board[idx] = ch
            } else {
                let value: usize = ch.to_digit(10).unwrap() as usize;
                board[idx] = ' ';
                board[idx + 24 * value] = '◯'
            }
        }

        board.iter().collect()
    }
}

pub static ALL_CHORDS: &'static [&'static Chord] = &[
    &Chord {
        short_name: "A",
        pattern: "x02220",
        name: "A major",
        barre: None,
    },
    &Chord {
        short_name: "Amaj7",
        pattern: "x02120",
        name: "A major 7th",
        barre: None,
    },
    &Chord {
        short_name: "A7",
        pattern: "x02020",
        name: "A 7th",
        barre: None,
    },
    &Chord {
        short_name: "Am",
        pattern: "x02210",
        name: "A minor",
        barre: None,
    },
    &Chord {
        short_name: "Am7",
        pattern: "x02010",
        name: "A minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "B",
        pattern: "xx4442",
        name: "B major",
        barre: None,
    },
    &Chord {
        short_name: "Bmaj7",
        pattern: "22130x",
        name: "B Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "B7",
        pattern: "x21202",
        name: "B 7th",
        barre: None,
    },
    &Chord {
        short_name: "Bm",
        pattern: "xx4432",
        name: "B minor",
        barre: None,
    },
    &Chord {
        short_name: "Bm7",
        pattern: "x20202",
        name: "B minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "C",
        pattern: "x32010",
        name: "C major",
        barre: None,
    },
    &Chord {
        short_name: "Cmaj7",
        pattern: "x32000",
        name: "C Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "C7",
        pattern: "x32310",
        name: "C 7th",
        barre: None,
    },
    &Chord {
        short_name: "Cm",
        pattern: "x310xx",
        name: "C minor",
        barre: None,
    },
    &Chord {
        short_name: "Cm7",
        pattern: "x313xx",
        name: "C minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "D",
        pattern: "xx0232",
        name: "D major",
        barre: None,
    },
    &Chord {
        short_name: "Dmaj7",
        pattern: "xx0222",
        name: "D Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "D7",
        pattern: "xx0212",
        name: "D 7th",
        barre: None,
    },
    &Chord {
        short_name: "Dm",
        pattern: "xx0231",
        name: "D minor",
        barre: None,
    },
    &Chord {
        short_name: "Dm7",
        pattern: "xx0211",
        name: "D minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "E",
        pattern: "022100",
        name: "E major",
        barre: None,
    },
    &Chord {
        short_name: "Emaj7",
        pattern: "021100",
        name: "E Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "E7",
        pattern: "020100",
        name: "E 7th",
        barre: None,
    },
    &Chord {
        short_name: "Em",
        pattern: "022000",
        name: "E minor",
        barre: None,
    },
    &Chord {
        short_name: "Em7",
        pattern: "022030",
        name: "E minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "F",
        pattern: "133211",
        name: "F major",
        barre: Some(Barre {
            from_string: 0,
            to_string: 5,
            fret: 1,
        }),
    },
    &Chord {
        short_name: "Fmaj7",
        pattern: "xx3210",
        name: "F Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "F7",
        pattern: "131211",
        name: "F 7th",
        barre: None,
    },
    &Chord {
        short_name: "Fm",
        pattern: "xx3111",
        name: "F minor",
        barre: None,
    },
    &Chord {
        short_name: "Fm7",
        pattern: "131111",
        name: "F minor 7th",
        barre: None,
    },
    &Chord {
        short_name: "G",
        pattern: "320003",
        name: "G major",
        barre: None,
    },
    &Chord {
        short_name: "Gmaj7",
        pattern: "3x0002",
        name: "G Major 7th",
        barre: None,
    },
    &Chord {
        short_name: "G7",
        pattern: "320001",
        name: "G 7th",
        barre: None,
    },
    &Chord {
        short_name: "Gm",
        pattern: "xx0333",
        name: "G minor",
        barre: None,
    },
    &Chord {
        short_name: "Gm7",
        pattern: "x13030",
        name: "G minor 7th",
        barre: None,
    },
];

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chord_pattern_length() {
        for chord in CHORDS {
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
        for chord in CHORDS {
            for char in chord.pattern.chars() {
                match char.to_digit(10) {
                    None => assert_eq!(char, 'x', "Only digits or 'x' is allowed"),
                    Some(digit) => assert!(digit < 6),
                }
            }
        }
    }
}
