use clap::Parser;
use std::collections::HashMap;

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

    let chords: HashMap<&str, &str> =
        HashMap::from([("C", "x32010"), ("G", "230003"), ("D", "xx0232")]);

    match chords.get(&args.name[..]) {
        None => println!("Unknown chord '{}'", args.name),
        Some(pattern) => {
            let mut board: Vec<char> = FRETBOARD.chars().collect();
            for (i, ch) in pattern.chars().enumerate() {
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
