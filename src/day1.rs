/// How many measurements are larger than the previous measurement?
use crate::common::read_input;

pub fn part1() {
    let values: Vec<u32> = read_input(1);

    let r = (1..values.len())
        .map(|i| u32::from(values[i - 1] < values[i]))
        .sum::<u32>()
        .to_string();

    println!("Day 1 - Part 1: {}", r);
}

/// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
pub fn part2() {
    let values: Vec<u32> = read_input(1);

    let r = (0..(values.len() - 3))
        .map(|i| {
            u32::from(values[i..i + 3].iter().sum::<u32>() < values[i + 1..i + 4].iter().sum())
        })
        .sum::<u32>()
        .to_string();

    println!("Day 1 - Part 2: {}", r);
}
