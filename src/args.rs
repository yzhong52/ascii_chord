use clap::{Parser, Subcommand};

use crate::chords;

/// A CLI to show you how to play a guitar chord
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    pub fn run(self) {
        match self.command {
            Command::All => {
                for chord in chords::ALL_CHORDS {
                    println!("{}:\n{}", chord.name, chord.fretboard())
                }
                
            },
            Command::Get(get_args) => get_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get(GetArgs),
    All,
}

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct GetArgs {
    /// Name of the chord
    #[clap()]
    name: String,
}

impl GetArgs {
    fn run(self) {
        match chords::ALL_CHORDS
            .iter()
            .find(|&chord| chord.short_name.to_ascii_uppercase() == self.name.to_ascii_uppercase())
        {
            None => println!("Unknown chord '{}'", self.name),
            Some(chord) => println!(
                "This is how you play '{}' chord: \n{}",
                chord.name,
                chord.fretboard()
            ),
        }
    }
}
