[![CI](https://github.com/yzhong52/ascii_chord/actions/workflows/rust.yml/badge.svg)](
  https://github.com/yzhong52/ascii_chord/actions?query=workflow%3Arust
)


# Ascii Chord

A command line tool for showing how to play guitar chords

## Usage

For a single chord:

```
$ aschord get G

This is how you play 'G' chord: 
    ◯ ◯ ◯  
╒═╤═╤═╤═╤═╕
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ ◯ │ │ │ │
├─┼─┼─┼─┼─┤
◯ │ │ │ │ ◯
└─┴─┴─┴─┴─┘
```

For multiple chords:

```
$ aschord list Em Esus4 Asus2 F
Em             Esus|Esus4     Asus2          F          
◯     ◯ ◯ ◯    ◯       ◯ ◯    ✕ ◯     ◯ ◯               
╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕
│ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    ◯―――――――◯―◯
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤
│ ◯ ◯ │ │ │    │ ◯ ◯ ◯ │ │    │ │ ◯ ◯ │ │    │ │ │ ◯ │ │
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤
│ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    │ ◯ ◯ │ │ │
└─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘
```

For different ways to play the same chord:

```
$ aschord list Bm --style full-names
B minor          B minor (no bar)
✕                ✕ ✕             
╒═╤═╤═╤═╤═╕      ╒═╤═╤═╤═╤═╕     
│ │ │ │ │ │      │ │ │ │ │ │     
├─┼─┼─┼─┼─┤      ├─┼─┼─┼─┼─┤     
――◯―――――――◯      │ │ │ │ │ ◯     
├─┼─┼─┼─┼─┤      ├─┼─┼─┼─┼─┤     
│ │ │ │ ◯ │      │ │ │ │ ◯ │     
├─┼─┼─┼─┼─┤      ├─┼─┼─┼─┼─┤     
│ │ ◯ ◯ │ │      │ │ ◯ ◯ │ │     
└─┴─┴─┴─┴─┘      └─┴─┴─┴─┴─┘     
```

For multiple chords with full names:

```
$ aschord list Em Esus4 Asus2 F --style full-names
E minor                        E suspended|E suspended 4ᵗʰ    A suspended 2ⁿᵈ                F          
◯     ◯ ◯ ◯                    ◯       ◯ ◯                    ✕ ◯     ◯ ◯                               
╒═╤═╤═╤═╤═╕                    ╒═╤═╤═╤═╤═╕                    ╒═╤═╤═╤═╤═╕                    ╒═╤═╤═╤═╤═╕
│ │ │ │ │ │                    │ │ │ │ │ │                    │ │ │ │ │ │                    ◯―――――――◯―◯
├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤
│ ◯ ◯ │ │ │                    │ ◯ ◯ ◯ │ │                    │ │ ◯ ◯ │ │                    │ │ │ ◯ │ │
├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤                    ├─┼─┼─┼─┼─┤
│ │ │ │ │ │                    │ │ │ │ │ │                    │ │ │ │ │ │                    │ ◯ ◯ │ │ │
└─┴─┴─┴─┴─┘                    └─┴─┴─┴─┴─┘                    └─┴─┴─┴─┴─┘                    └─┴─┴─┴─┴─┘
```

Here is an example of using a [bar chord](https://en.wikipedia.org/wiki/Bar_chord) on the first (1st) fret for F and a [capo](https://en.wikipedia.org/wiki/Capo_(musical_device)) placed on the fourth (4th) fret for G#:

```
$ aschord list F G#
F              G#         
               ◯       ◯ ◯
╒═╤═╤═╤═╤═╕    ╒𝟺╤𝟺╤𝟺╤𝟺╤𝟺╕
◯―――――――◯―◯    │ │ │ ◯ │ │
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤
│ │ │ ◯ │ │    │ ◯ ◯ │ │ │
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤
│ ◯ ◯ │ │ │    │ │ │ │ │ │
└─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘
```

For display purposes, one can adjust the amount of space padding between displayed chords using the `-p` short option or the longer `--padding` option:
```
$ aschord list -p 1 C E
C           E          
✕     ◯   ◯ ◯       ◯ ◯
╒═╤═╤═╤═╤═╕ ╒═╤═╤═╤═╤═╕
│ │ │ │ ◯ │ │ │ │ ◯ │ │
├─┼─┼─┼─┼─┤ ├─┼─┼─┼─┼─┤
│ │ ◯ │ │ │ │ ◯ ◯ │ │ │
├─┼─┼─┼─┼─┤ ├─┼─┼─┼─┼─┤
│ ◯ │ │ │ │ │ │ │ │ │ │
└─┴─┴─┴─┴─┘ └─┴─┴─┴─┴─┘

```

For all support chords:

```
$ aschord all
```

Also see all supported chords [here](all_supported_chords.md) (auto-generated, do not edit).

## Installation

Install local version:

```
git clone git@github.com:ascii-music/ascii_chord.git
cd ascii_chord
cargo install --path .
```

Install from <https://crates.io/crates/aschord>:

```
cargo install aschord
```

## Development

### Build & Run

Show a single chord:

```
cargo run -- G
```

Show multiple chords:

```
cargo run -- list Em Esus4 Asus2 Fm D Am
```

Show all chords:

```
cargo run -- all
```

### Unit Tests

```
cargo test
```

### Release

```
cargo publish
```

## Clawhub

Available on clawhub here: https://clawhub.ai/yzhong52/ascii-chord

