extern crate clap;

use clap::Parser;
mod chords;

const FRETBOARD: &str = "◯ ◯ ◯ ◯ ◯ ◯
┌─┬─┬─┬─┬─┐
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘";

/// A CLI to show you how to play a guitar chord
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Name of the chord
    #[clap()]
    name: String,
}

fn main() {
    let args = Args::parse();

    match chords::CHORDS
        .iter()
        .find(|&chord| chord.short_name.to_ascii_uppercase() == args.name.to_ascii_uppercase())
    {
        None => println!("Unknown chord '{}'", args.name),
        Some(chord) => {
            let mut board: Vec<char> = FRETBOARD.chars().collect();
            for (i, ch) in chord.pattern.chars().enumerate() {
                let idx: usize = i * 2;
                if ch == 'x' {
                    board[idx] = ch
                } else {
                    let value: usize = ch.to_digit(10).unwrap() as usize;
                    board[idx] = ' ';
                    board[idx + 24 * value] = '◯'
                }
            }
            println!(
                "This is how you play '{}' chord: \n{}",
                args.name,
                board.iter().collect::<String>()
            )
        }
    }
}
