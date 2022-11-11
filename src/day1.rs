/// How many measurements are larger than the previous measurement?
pub fn part1(input: &str) -> String {
    let values: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    (1..values.len())
        .map(|i| u32::from(values[i - 1] < values[i]))
        .sum::<u32>()
        .to_string()
}

/// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
pub fn part2(input: &str) -> String {
    let values: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    (0..(values.len() - 3))
        .map(|i| {
            u32::from(values[i..i + 3].iter().sum::<u32>() < values[i + 1..i + 4].iter().sum())
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
            "#
        .trim();

        assert_eq!(part1(input), "7".to_string());
    }

    #[test]
    fn test_part2() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
            "#
        .trim();

        assert_eq!(part2(input), "5".to_string());
    }
}
