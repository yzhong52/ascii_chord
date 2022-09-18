extern crate clap;

mod args;
mod chords;

use args::Args;
use clap::Parser;

fn main() {
    Args::parse().run();
}
