# Ascii Chord

A command line tool for showing how to play guitar chords

## Usage

For a single chord:

```
$ aschord get G

This is how you play 'G' chord: 
    ◯ ◯ ◯  
┌─┬─┬─┬─┬─┐
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

Em             Esus4          Asus2          F              
◯     ◯ ◯ ◯    ◯       ◯ ◯    x ◯     ◯ ◯                   
╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕    ╒═╤═╤═╤═╤═╕    
│ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    ◯-------◯-◯    
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    
│ ◯ ◯ │ │ │    │ ◯ ◯ ◯ │ │    │ │ ◯ ◯ │ │    │ │ │ ◯ │ │    
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    
│ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    │ ◯ ◯ │ │ │    
├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    ├─┼─┼─┼─┼─┤    
│ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    │ │ │ │ │ │    
└─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘    └─┴─┴─┴─┴─┘   
```

For multiple chords with full names:

```
$ aschord list Em Esus4 Asus2 F --style full-name

E minor             E suspended fourth  A suspended 2nd     F major             
◯     ◯ ◯ ◯         ◯       ◯ ◯         x ◯     ◯ ◯                             
╒═╤═╤═╤═╤═╕         ╒═╤═╤═╤═╤═╕         ╒═╤═╤═╤═╤═╕         ╒═╤═╤═╤═╤═╕         
│ │ │ │ │ │         │ │ │ │ │ │         │ │ │ │ │ │         ◯-------◯-◯         
├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         
│ ◯ ◯ │ │ │         │ ◯ ◯ ◯ │ │         │ │ ◯ ◯ │ │         │ │ │ ◯ │ │         
├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         
│ │ │ │ │ │         │ │ │ │ │ │         │ │ │ │ │ │         │ ◯ ◯ │ │ │         
├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         ├─┼─┼─┼─┼─┤         
│ │ │ │ │ │         │ │ │ │ │ │         │ │ │ │ │ │         │ │ │ │ │ │         
└─┴─┴─┴─┴─┘         └─┴─┴─┴─┴─┘         └─┴─┴─┴─┴─┘         └─┴─┴─┴─┴─┘   
```

For all support chords:

```
$ aschord all
```

Also see all supported chords [here](all_supported_chords.md) (auto-generated, do not edit).

## Installation

Install local version:

```
git clone git@github.com:yzhong52/ascii_chord.git
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

