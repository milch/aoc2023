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
        3 => day_03::print_solution(),
        4 => day_04::print_solution(),
        5 => day_05::print_solution(),
        6 => day_06::print_solution(),
        7 => day_07::print_solution(),
        8 => day_08::print_solution(),
        9 => day_09::print_solution(),
        10 => day_10::print_solution(),
        11 => day_11::print_solution(),
        _ => unimplemented!(),
    }
}
