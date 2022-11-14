use std::cmp::{max, min};

/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
pub fn part1(input: &str) -> String {
    let input_lines = input.lines();
    let mut lines: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input_lines {
        let (left, right) = if let Some((left, right)) = line.split_once(" -> ") {
            (left, right)
        } else {
            unreachable!()
        };
        let (x1, y1) = if let Some((x1, y1)) = left.split_once(',') {
            (x1, y1)
        } else {
            unreachable!()
        };
        let (x2, y2) = if let Some((x2, y2)) = right.split_once(',') {
            (x2, y2)
        } else {
            unreachable!()
        };

        lines.push((
            (x1.parse().unwrap(), y1.parse().unwrap()),
            (x2.parse().unwrap(), y2.parse().unwrap()),
        ));
    }

    let (mut max_x, mut max_y) = (0, 0);

    // TODO: Refactor using Iterator::max()
    for ((x1, y1), (x2, y2)) in &lines {
        if x1 > &max_x {
            max_x = *x1;
        }
        if x2 > &max_x {
            max_x = *x2;
        }
        if y1 > &max_y {
            max_y = *y1;
        }
        if y2 > &max_y {
            max_y = *y2;
        }
    }

    max_x += 1;
    max_y += 1;

    let mut cells = vec![0; (max_x * max_y) as usize];

    for ((x1, y1), (x2, y2)) in lines {
        if x1 != x2 && y1 != y2 {
            continue;
        }
        for i in min(x1, x2)..=max(x1, x2) {
            for j in min(y1, y2)..=max(y1, y2) {
                cells[(j as usize) * (max_x as usize) + (i as usize)] += 1;
            }
        }
    }

    cells.iter().filter(|n| n > &&1).count().to_string()
}

pub fn part2(_: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
            "#
        .trim();

        assert_eq!(part1(input), "5".to_string());
    }

    #[test]
    fn test_part2() {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
            "#
        .trim();

        assert_eq!(part2(input), "12".to_string());
    }
}
