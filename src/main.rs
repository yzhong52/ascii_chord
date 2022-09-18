extern crate clap;
extern crate once_cell;

mod chord;
mod chords;
mod commands;
mod stitcher;

use clap::Parser;
use commands::Args;

fn main() {
    Args::parse().run();
}
