/// Find all of the low points on your heightmap. What is the sum of the risk levels of all low
/// points on your heightmap?
pub fn part1(input: &str) -> String {
    let map: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let mut ret = 0;

    for i in 0..map.len() as i32 {
        for j in 0..map[i as usize].len() as i32 {
            if [(-1, 0), (0, -1), (0, 1), (1, 0)].iter().all(|(x, y)| {
                &map[i as usize][j as usize]
                    < map
                        .get((i + y) as usize)
                        .unwrap_or(&Vec::new())
                        .get((j + x) as usize)
                        .unwrap_or(&10)
            }) {
                ret += (map[i as usize][j as usize] as u32) + 1;
            }
        }
    }

    ret.to_string()
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
        assert_eq!(part1(INPUT), "15");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../inputs/9")), "506");
    }
}
