/// Determine the horizontal position that the crabs can align to using the least fuel possible.
/// How much fuel must they spend to align to that position?
pub fn part1(input: &str) -> String {
    let positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let mut min_fuel = i32::MAX;

    for target in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
        let fuel = positions.iter().map(|p| (p - target).abs()).sum::<i32>();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel.to_string()
}

/// Determine the horizontal position that the crabs can align to using the least fuel possible
/// so they can make you an escape route! How much fuel must they spend to align to that position?
pub fn part2(input: &str) -> String {
    let positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let mut min_fuel = i32::MAX;

    for target in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
        let fuel = positions
            .iter()
            .map(|p| {
                let distance = (p - target).abs();
                (distance * (distance + 1)) / 2
            })
            .sum::<i32>();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "37");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "168");
    }
}
