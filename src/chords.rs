#[derive(Debug)]
pub struct Chord<'a> {
    pub short_name: &'a str,
    pub pattern: &'a str,
    pub name: &'a str,
}

pub static CHORDS: &'static [&'static Chord] = &[
    &Chord {
        short_name: "A",
        pattern: "x02220",
        name: "A major",
    },
    &Chord {
        short_name: "Amaj7",
        pattern: "x02120",
        name: "A major 7th",
    },
    &Chord {
        short_name: "A7",
        pattern: "x02020",
        name: "A 7th",
    },
    &Chord {
        short_name: "Am",
        pattern: "x02210",
        name: "A minor",
    },
    &Chord {
        short_name: "Am7",
        pattern: "x02010",
        name: "A minor 7th",
    },
    &Chord {
        short_name: "B",
        pattern: "xx4442",
        name: "B major",
    },
    &Chord {
        short_name: "Bmaj7",
        pattern: "22130x",
        name: "B Major 7th",
    },
    &Chord {
        short_name: "B7",
        pattern: "x21202",
        name: "B 7th",
    },
    &Chord {
        short_name: "Bm",
        pattern: "xx4432",
        name: "B minor",
    },
    &Chord {
        short_name: "Bm7",
        pattern: "x20202",
        name: "B minor 7th",
    },
    &Chord {
        short_name: "C",
        pattern: "x32010",
        name: "C major",
    },
    &Chord {
        short_name: "Cmaj7",
        pattern: "x32000",
        name: "C Major 7th",
    },
    &Chord {
        short_name: "C7",
        pattern: "x32310",
        name: "C 7th",
    },
    &Chord {
        short_name: "Cm",
        pattern: "x310xx",
        name: "C minor",
    },
    &Chord {
        short_name: "Cm7",
        pattern: "x313xx",
        name: "C minor 7th",
    },
    &Chord {
        short_name: "D",
        pattern: "xx0232",
        name: "D major",
    },
    &Chord {
        short_name: "Dmaj7",
        pattern: "xx0222",
        name: "D Major 7th",
    },
    &Chord {
        short_name: "D7",
        pattern: "xx0212",
        name: "D 7th",
    },
    &Chord {
        short_name: "Dm",
        pattern: "xx0231",
        name: "D minor",
    },
    &Chord {
        short_name: "Dm7",
        pattern: "xx0211",
        name: "D minor 7th",
    },
    &Chord {
        short_name: "E",
        pattern: "022100",
        name: "E major",
    },
    &Chord {
        short_name: "Emaj7",
        pattern: "021100",
        name: "E Major 7th",
    },
    &Chord {
        short_name: "E7",
        pattern: "020100",
        name: "E 7th",
    },
    &Chord {
        short_name: "Em",
        pattern: "022000",
        name: "E minor",
    },
    &Chord {
        short_name: "Em7",
        pattern: "022030",
        name: "E minor 7th",
    },
    &Chord {
        short_name: "F",
        pattern: "xx3211",
        name: "F major",
    },
    &Chord {
        short_name: "Fmaj7",
        pattern: "xx3210",
        name: "F Major 7th",
    },
    &Chord {
        short_name: "F7",
        pattern: "131211",
        name: "F 7th",
    },
    &Chord {
        short_name: "Fm",
        pattern: "xx3111",
        name: "F minor",
    },
    &Chord {
        short_name: "Fm7",
        pattern: "131111",
        name: "F minor 7th",
    },
    &Chord {
        short_name: "G",
        pattern: "320003",
        name: "G major",
    },
    &Chord {
        short_name: "Gmaj7",
        pattern: "3x0002",
        name: "G Major 7th",
    },
    &Chord {
        short_name: "G7",
        pattern: "320001",
        name: "G 7th",
    },
    &Chord {
        short_name: "Gm",
        pattern: "xx0333",
        name: "G minor",
    },
    &Chord {
        short_name: "Gm7",
        pattern: "x13030",
        name: "G minor 7th",
    },
];
