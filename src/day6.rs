use std::collections::HashMap;

/**
 * Memoization + Recursive !!!
 *
 * If the result of a given combination of (state, days left) is already known, it returns it.
 * Otherwise, we calculate it for that fish and store it as a known/memorised one.
 */
fn solve_fish(orig_state: u32, days: usize, memo: &mut HashMap<(u32, usize), usize>) -> usize {
    // Check if we already know the result of this fish
    if let Some(count) = memo.get(&(orig_state, days)) {
        return *count;
    }

    // Count the current fish
    let mut count = 1;
    // Current state of the fish
    let mut state = orig_state;

    // Calculate the count for the current fish
    for d in 0..days {
        if state == 0 {
            // Spawn a new fish and add its own count to the count of the current one.
            count += solve_fish(8, days - d - 1, memo);
            state = 6
        } else {
            state -= 1;
        }
    }

    // Store/Memorize the result of the current fish
    memo.insert((orig_state, days), count);
    // Return the count of the current fish
    count
}

/**
 * Generic function to calculate the number of fishs after given `days` and `state`.
 */
fn solve(input: &str, days: usize) -> String {
    // Parse the input (comma separated list of fish)
    let fishs: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // Fish counter
    let mut count = 0;
    // Memoization HashMap, maps a combination of `state` and `days` to a count
    let mut memo = HashMap::new();

    // For each input fish, add its calculated count
    for fish in fishs {
        count += solve_fish(fish, days, &mut memo);
    }

    // Return the sum of counts
    count.to_string()
}

/// Find a way to simulate lanternfish. How many lanternfish would there be after 80 days?
pub fn part1(input: &str) -> String {
    solve(input, 80)
}

/// How many lanternfish would there be after 256 days?
pub fn part2(input: &str) -> String {
    solve(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3,4,3,1,2"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "5934");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "26984457539");
    }
}
