pub mod solutions;

use clap::{arg, Parser};
use solutions::*;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    day: u8,
}

fn main() {
    let args = Cli::parse();
    match args.day {
        1 => day_01::print_solution(),
        2 => day_02::print_solution(),
        _ => unimplemented!(),
    }
}
