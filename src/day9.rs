use std::collections::HashSet;

fn valid_cardinals(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if new_x >= 0 && (new_x as usize) < max_x && new_y >= 0 && (new_y as usize) < max_y {
            ret.push((new_y as usize, new_x as usize));
        }
    }

    ret
}

/// Find all of the low points on your heightmap. What is the sum of the risk levels of all low
/// points on your heightmap?
pub fn part_1(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut ret = 0;

    let max_y = map.len();
    let max_x = map[0].len();

    for (y, line) in map.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if valid_cardinals(x, y, max_x, max_y)
                .iter()
                .all(|(adj_y, adj_x)| val < &map[*adj_y][*adj_x])
            {
                ret += val + 1;
            }
        }
    }

    ret.to_string()
}

/// What do you get if you multiply together the sizes of the three largest basins?
pub fn part_2(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut basins = Vec::new();

    let max_y = map.len();
    let max_x = map[0].len();

    for (y, line) in map.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if valid_cardinals(x, y, max_x, max_y)
                .iter()
                .all(|(adj_y, adj_x)| val < &map[*adj_y][*adj_x])
            {
                basins.push((y, x));
            }
        }
    }

    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();

    // For each basin
    for (basin_y, basin_x) in basins {
        // Current basin size and list of points to explore
        let mut basin_size = 0;
        let mut to_explore = vec![(basin_y, basin_x)];

        // As long as there are points to explore next
        while !to_explore.is_empty() {
            // Take the next point in the list of the ones to explore, do nothing if already
            // explored
            let (current_y, current_x) = to_explore.pop().unwrap();
            if explored.contains(&(current_y, current_x)) {
                continue;
            }

            // Set the current point as explored and count it in the current basin
            basin_size += 1;
            explored.insert((current_y, current_x));
            // Get the current point's value
            let current_val = map[current_y][current_x];

            // For each valid adjacent point
            for (adj_y, adj_x) in valid_cardinals(current_x, current_y, max_x, max_y) {
                // Get the value of the adjacent point
                let adj_val = map[adj_y][adj_x];

                // If the neighbor is a basin point too, add it to the ones to explore next
                if current_val < adj_val && adj_val != 9 {
                    to_explore.push((adj_y, adj_x));
                }
            }
        }

        // Store the final basin size
        basin_sizes.push(basin_size);
    }

    // Sort the calculated basin sizes
    basin_sizes.sort();
    // Return the product of the 3 biggest basins
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "15");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/9")), "506");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(INPUT), "1134");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../inputs/9")), "931200");
    }
}
