use crate::common::read_input;

/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
/**
 * Note: Each line contains a 12-bits number (e.g. 011111111101).
 */
pub fn part1() {
    let lines = read_input::<String>(3);

    let (mut gamma, mut epsilon) = (0, 0);

    for i in 0..12 {
        let mask = 1 << i;

        if lines
            .iter()
            .map(|x| usize::from((u32::from_str_radix(x, 2).unwrap() & mask) != 0))
            .sum::<usize>()
            > lines.len() / 2
        // Majority of 1, minority of 0
        {
            gamma |= mask;
        } else {
            // Majority of 0, minority of 1
            epsilon |= mask;
        }
    }

    println!("Day 3 - Part 1: {}", gamma * epsilon);
}
