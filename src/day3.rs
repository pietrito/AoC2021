fn get_start_bit(input: &[u32]) -> usize {
    for i in (0..31).rev() {
        let mask = 1 << i;
        if input.iter().any(|x| (x & mask) != 0) {
            return i + 1;
        }
    }

    panic!("Empty input ?");
}

/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
pub fn part1(input: &str) -> String {
    let values: Vec<u32> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    let (mut gamma, mut epsilon) = (0, 0);

    for i in (0..get_start_bit(&values)).rev() {
        let mask = 1 << i;

        if values
            .iter()
            .map(|x| usize::from((x & mask) != 0))
            .sum::<usize>()
            > values.len() / 2
        // Majority of 1, minority of 0
        {
            gamma |= mask;
        } else {
            // Majority of 0, minority of 1
            epsilon |= mask;
        }
    }

    (gamma * epsilon).to_string()
}

///Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
pub fn part2(input: &str) -> String {
    let mut oxys: Vec<u32> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let mut co2s = vec![0; oxys.len()];
    co2s.copy_from_slice(&oxys[0..]);

    let mut oxygen_generator_rating = 0;
    let mut co2_scrubber_rating = 0;

    // Calculate Oxygen Generator Rating
    for i in (0..get_start_bit(&oxys)).rev() {
        let mask = 1 << i;

        let ones = oxys
            .iter()
            .map(|x| usize::from((x & mask) != 0))
            .sum::<usize>();
        let zeros = oxys.len() - ones;

        if ones >= zeros {
            oxys.retain(|x| (x & mask) != 0);
        } else {
            oxys.retain(|x| (x & mask) == 0);
        }

        if oxys.len() == 1 {
            oxygen_generator_rating = oxys[0];
            break;
        }
    }

    for i in (0..get_start_bit(&co2s)).rev() {
        let mask = 1 << i;

        let zeros = co2s
            .iter()
            .map(|x| usize::from((x & mask) == 0))
            .sum::<usize>();
        let ones = co2s.len() - zeros;

        if zeros <= ones {
            co2s.retain(|x| (x & mask) == 0);
        } else {
            co2s.retain(|x| (x & mask) != 0);
        }

        if co2s.len() == 1 {
            co2_scrubber_rating = co2s[0];
            break;
        }
    }

    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
    "#
        .trim();

        assert_eq!(part1(input), "198".to_string());
    }

    #[test]
    fn test_part2() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
    "#
        .trim();

        assert_eq!(part2(input), "230".to_string());
    }
}
