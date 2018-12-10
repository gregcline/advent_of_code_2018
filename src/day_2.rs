use std::collections::HashMap;
use std::io;

use crate::solver::{Solution, Solver};

pub struct Day2Solver;

impl Solver for Day2Solver {
    fn part_1(&self, puzzle: &str) -> io::Result<Solution> {
        let counts = puzzle.split('\n').map(letter_counts);
        let twos = counts.clone().filter(|x| x.contains(&2)).count();
        let threes = counts.filter(|x| x.contains(&3)).count();
        Ok(Solution::USize(twos * threes))
    }

    fn part_2(&self, puzzle: &str) -> io::Result<Solution> {
        for i in puzzle.split('\n') {
            for j in puzzle.split('\n') {
                if hamming_distance(i, j) == 1 {
                    return Ok(Solution::String(
                        i.chars()
                            .zip(j.chars())
                            .filter(|(a, b)| a == b)
                            .map(|(a, _b)| a)
                            .collect::<String>(),
                    ));
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::Other, "Could not find pair"))
    }
}

fn letter_counts(letters: &str) -> Vec<u32> {
    let mut letter_count = HashMap::new();
    for letter in letters.chars() {
        let counter = letter_count.entry(letter).or_insert(0);
        *counter += 1;
    }
    letter_count.iter().map(|(_k, &v)| v).collect::<Vec<u32>>()
}

fn hamming_distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}
