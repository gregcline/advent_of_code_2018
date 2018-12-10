use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fmt;
use std::io;
use tap::TapOps;

use crate::day_3::point::{Point, Rectangle};
use crate::solver::{Solution, Solver};

pub mod point;

pub struct Day3Solver;

impl Solver for Day3Solver {
    fn part_1(&self, puzzle: &str) -> io::Result<Solution> {
        let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        Ok(Solution::Cloth(
            re.captures_iter(puzzle)
                .map(make_rectangle)
                .fold(HashMap::new(), claim_cloth)
                .iter()
                .filter(|(_point, &claims)| claims > 1)
                .count(),
        ))
    }

    fn part_2(&self, puzzle: &str) -> io::Result<Solution> {
        unimplemented!()
    }
}

pub type Claims = usize;

fn make_rectangle(data: Captures) -> Rectangle {
    Rectangle::new(
        data[1].parse::<usize>().unwrap_or(1),
        data[2].parse::<usize>().unwrap_or(1),
        data[3].parse::<usize>().unwrap_or(1),
        data[4].parse::<usize>().unwrap_or(1),
        data[5].parse::<usize>().unwrap_or(1),
    )
}

fn claim_cloth(mut cloth: HashMap<Point, Claims>, new_claim: Rectangle) -> HashMap<Point, Claims> {
    for point in new_claim.points() {
        let claim_count = cloth.entry(point).or_insert(0);
        *claim_count += 1;
    }
    cloth
}
