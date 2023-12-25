pub mod solutions;
pub mod utils;

use aoc::aoc;
use clap::{arg, Parser};
use solutions::*;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    day: u8,
}

fn main() {
    let args = Cli::parse();
    aoc!(args.day => day_{:02}::print_solution(), 1..22)
}
