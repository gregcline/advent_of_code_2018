extern crate regex;
extern crate spinners;
extern crate structopt;
extern crate tap;

use spinners::{Spinner, Spinners};
use structopt::StructOpt;

use crate::solver::Puzzle;

mod day_1;
mod day_2;
mod day_3;
mod solver;

#[derive(StructOpt, Debug)]
enum Days {
    #[structopt(name = "day_1")]
    Day1 {
        /// The name of the puzzle input file
        input: String,
    },
    #[structopt(name = "day_2")]
    Day2 {
        /// The name of the puzzle input file
        input: String,
    },
    #[structopt(name = "day_3")]
    Day3 {
        /// The name of the puzzle input file
        input: String,
    },
}

fn main() {
    let opt = Days::from_args();
    let solver = match opt {
        Days::Day1 { input } => Puzzle::new(&input, Box::new(day_1::Day1Solver)).unwrap(),
        Days::Day2 { input } => Puzzle::new(&input, Box::new(day_2::Day2Solver)).unwrap(),
        Days::Day3 { input } => Puzzle::new(&input, Box::new(day_3::Day3Solver)).unwrap(),
    };

    let sp = Spinner::new(Spinners::Dots, "Working on part 1".into());
    let part_1 = solver.solver.part_1(&solver.puzzle);
    sp.stop();
    match part_1 {
        Ok(val) => println!("\n{}", val),
        Err(val) => println!("\n{}", val),
    }

    let sp = Spinner::new(Spinners::Dots, "Working on part 2".into());
    let part_2 = solver.solver.part_2(&solver.puzzle);
    sp.stop();
    match part_2 {
        Ok(val) => println!("\n{}", val),
        Err(val) => println!("\n{}", val),
    }
}
