use clap::{Parser, Subcommand};
use stitcher::NameStyle;

use crate::chords;
use crate::stitcher;
use chord::Chord;
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
            Command::Get(get_args) => get_args.run(),
            Command::List(list_args) => list_args.run(),
            Command::All(all_args) => all_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Get a single chord
    Get(GetArgs),
    /// List multiple chords
    List(ListArgs),
    /// Print all the supported chords
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
            buffer += &format!("\n\n## {}\n\n", chord.both_names()).to_string();
            buffer += &format!("{{% include chord-begin %}}\n{}\n{{% include chord-end %}}", chord.fretboard()).to_string();
        }

        if self.save {
            fs::write("./all_supported_chords.md", &buffer).expect("Unable to write file");
        }
        println!("{}\n", buffer);
    }
}

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Names of the chords
    #[clap()]
    names: Vec<String>,

    /// In the output, which name to include
    #[clap(arg_enum, long="style", default_value_t=NameStyle::ShortName)]
    name_style: NameStyle,
}

impl ListArgs {
    fn run(self) {
        // We would like to keep the order that 'names' are passed in
        let chords: Vec<Chord<'static>> = self
            .names
            .iter()
            .map(|name| -> Vec<Chord<'static>> {
                // Find the ones that matches the chord name
                match chords::ALL_CHORDS_BY_SHORT_NAME.get(&name.to_ascii_lowercase()) {
                    Some::<&Vec<&'static Chord<'static>>>(matched_chords) => matched_chords
                        .iter()
                        .map(|chord: &&'static Chord<'static>| -> Chord<'static> { *chord.clone() })
                        .collect(),
                    None => {
                        println!("Unknown chord '{}'", name);
                        vec![]
                    }
                }
            })
            .flatten()
            .collect();
        let row = stitcher::row(chords, self.name_style);
        println!("{}", row);
    }
}
