extern crate once_cell;
extern crate itertools;
extern crate clap;

mod commands;
mod stitcher;
mod chords;
mod chord;

use clap::Parser;
use commands::Args;

fn main() {
    Args::parse().run();
}
