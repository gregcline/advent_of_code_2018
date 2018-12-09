use std::collections::HashMap;
use std::io;

use crate::solver::{Solution, Solver};

pub struct Day1Solver;

impl Solver for Day1Solver {
    fn part_1(&self, puzzle: &str) -> io::Result<Solution> {
        let solution = puzzle.split('\n').map(|line| line.parse::<i32>()).fold(0, {
            |acc, x| match x {
                Ok(x) => acc + x,
                _ => acc,
            }
        });
        Ok(Solution::I32(solution))
    }

    fn part_2(&self, puzzle: &str) -> io::Result<Solution> {
        let converted_puzzle = puzzle.split('\n').map(|line| line.parse::<i32>());

        let mut seen_freqs: HashMap<i32, u8> = HashMap::new();
        let mut prev: i32 = 0;

        for op in converted_puzzle.cycle() {
            if let Ok(x) = op {
                let next_freq = prev + x;
                if seen_freqs.contains_key(&next_freq) {
                    return Ok(Solution::I32(next_freq));
                } else {
                    seen_freqs.insert(next_freq, 0);
                    prev = next_freq;
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find a repeat",
        ))
    }
}
