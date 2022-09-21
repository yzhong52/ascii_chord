# Ascii Chord

A command line tool for showing how to play a guitar chord. 

## Usage

```
$ chord G               
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

## Installation

```
git clone git@github.com:yzhong52/ascii_chord.git
cd ascii_chord
cargo install --path .
```

## Development

```
$ cargo run -- G
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/chord G`
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
