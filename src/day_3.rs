use regex::{Captures, Regex};
use std::fmt;
use std::io;

use crate::solver::{Solution, Solver};

pub struct Day3Solver;

impl Solver for Day3Solver {
    fn part_1(&self, puzzle: &str) -> io::Result<Solution> {
        let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        Ok(re
            .captures_iter(puzzle)
            .map(make_rectangle)
            .fold(Cloth::new(), claim_cloth))
    }

    fn part_2(&self, puzzle: &str) -> io::Result<Solution> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn make_rectangle(data: Captures) -> Solution {
    Solution::Rectangle(Rectangle {
        id: data[1].parse::<usize>().unwrap_or(1),
        x: data[2].parse::<usize>().unwrap_or(1),
        y: data[3].parse::<usize>().unwrap_or(1),
        width: data[4].parse::<usize>().unwrap_or(1),
        height: data[5].parse::<usize>().unwrap_or(1),
    })
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{} @ {},{}: {}x{}",
            self.id, self.x, self.y, self.width, self.height
        )
    }
}
