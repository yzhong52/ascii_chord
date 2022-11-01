#[derive(Debug, Clone, Copy)]
pub struct Barre {
    // Guitar normally call the string with the highest pitch, or the thinnest,
    // the 1st string; and the thickest, lowest-pitched string, the 6th string.
    // BUT, we are counting the string from left to right as 0 to 5 here.
    pub from_string: u8,
    pub to_string: u8,

    pub fret: u16,
}

pub const BARRE_FRET1: Barre = Barre {
    from_string: 0,
    to_string: 5,
    fret: 1,
};

pub const BARRE_FRET2: Barre = Barre {
    from_string: 0,
    to_string: 5,
    fret: 2,
};

#[derive(Debug, Clone, Copy)]
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

impl Chord<'_> {
    pub fn both_names(&self) -> String {
        format!("{} ({})", self.name, self.short_name)
    }

    pub fn fretboard(&self, finger_notation: char) -> String {
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
                if value > 0 {
                    board[idx] = ' ';
                    board[idx + 24 * value] = finger_notation
                }
            }
        }

        board.iter().collect()
    }
}
