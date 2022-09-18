use clap::{Parser, Subcommand};

use crate::chords;
use std::fs;

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
            Command::All(all_args) => all_args.run(),

            Command::Get(get_args) => get_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get(GetArgs),
    All(AllArgs),
}

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub struct AllArgs {
    #[clap(default_value_t = false)]
    save: bool,
}
impl AllArgs {
    fn run(self) {
        let mut buffer: String = String::default();
        buffer += "# All Supported Chords";

        for chord in chords::ALL_CHORDS {
            buffer += &format!("\n## {}\n", chord.name).to_string();
            buffer += &format!("```\n{}\n```", chord.fretboard()).to_string();
        }
        
        if self.save {
            fs::write("./all_supported_chords.md", &buffer).expect("Unable to write file");
        }
        println!("{}\n", buffer);
    }
}
