use itertools::join;

#[derive(Debug, Clone, Copy)]
pub struct Barre {
    // Guitar normally call the string with the highest pitch, or the thinnest,
    // the 1st string; and the thickest, lowest-pitched string, the 6th string.
    // BUT, we are counting the string from left to right as 0 to 5 here.
    pub from_string: u8,
    pub to_string: u8,

    pub fret: u16,
}

impl Barre {
    pub const fn new(fret: u16) -> Self {
        Self {
            from_string: 0,
            to_string: 5,
            fret: fret,
        }
    }
}

pub const BARRE_FRET1: Barre = Barre::new(1);
pub const BARRE_FRET2: Barre = Barre::new(2);
pub const BARRE_FRET3: Barre = Barre::new(3);

pub const FRETBOARD: &str = "◯ ◯ ◯ ◯ ◯ ◯
╒═╤═╤═╤═╤═╕
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘";

#[derive(Debug, Clone, Copy)]
pub struct Chord<'a> {
    pub short_names: [Option<&'a str>; 2],
    // Require to be length of 6.
    // 'x' indicates that the string should be muted.
    // '0' indicates that playing the string as it (without finger placement).
    // '1' indicates that we place a finger on the first fret, '2' on the 2nd fret, and etc.
    pub pattern: &'a str,
    pub name: &'a str,
    pub barre: Option<Barre>,
}

impl<'a> Chord<'a> {
    pub const fn new(
        short_name: &'a str,
        pattern: &'a str,
        name: &'a str,
        barre: Option<Barre>,
    ) -> Self {
        Self {
            short_names: [Some(short_name), None],
            pattern: pattern,
            name: name,
            barre: barre,
        }
    }

    pub fn both_names(&self) -> String {
        format!(
            "{} ({})",
            self.name,
            join(self.short_names.iter().filter_map(|&name| name), " | ")
        )
    }

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
