use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum Solution {
    I32(i32),
    USize(usize),
    String(String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Solution::I32(x) => write!(f, "{}", x),
            Solution::USize(x) => write!(f, "{}", x),
            Solution::String(x) => write!(f, "{}", x),
        }
    }
}

pub trait Solver {
    fn part_1(&self, puzzle: &str) -> io::Result<Solution>;
    fn part_2(&self, puzzle: &str) -> io::Result<Solution>;
}

pub struct Puzzle {
    pub puzzle: String,
    pub solver: Box<Solver + 'static>,
}

impl Puzzle {
    pub fn new(file: &str, solver: Box<Solver + 'static>) -> io::Result<Self> {
        Ok(Puzzle {
            puzzle: Self::read_file(file)?,
            solver: solver,
        })
    }

    fn read_file(file: &str) -> io::Result<String> {
        let file = File::open(file)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
