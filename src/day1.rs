/// How many measurements are larger than the previous measurement?
use crate::common::read_input;

pub fn solve() -> String {
    let values: Vec<u32> = read_input(1);

    (1..values.len())
        .map(|i| u32::from(values[i - 1] < values[i]))
        .sum::<u32>()
        .to_string()
}
