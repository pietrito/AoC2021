use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead};

static INPUT_FOLDER: &str = "inputs";

pub fn read_input<T>(day_number: usize) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    let file =
        File::open(format!("{}/{}", INPUT_FOLDER, day_number)).expect("Error in reading file");
    let reader = io::BufReader::new(file);

    let out: Vec<T> = reader
        .lines()
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect();

    out
}

struct Challenge {
    input_path: String,
    day: usize,
}

pub trait Solver<T> {
    fn part1(&self, input: &str) -> T
    where
        T: Display;
    fn part2(&self, input: &str) -> T
    where
        T: Display;
}
