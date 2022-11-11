/// Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
pub fn part1(input: &str) -> String {
    let lines = input.lines();

    let (mut horizontal, mut vertical) = (0, 0);

    for line in lines {
        let (direction, value) =
            if let [direction, value] = line.split_whitespace().collect::<Vec<&str>>()[0..2] {
                (direction, value.parse::<i32>().unwrap())
            } else {
                panic!("Invalid line.")
            };

        match direction {
            "up" => vertical -= value,
            "down" => vertical += value,
            "forward" => horizontal += value,
            _ => panic!("Invalid direction."),
        }
    }

    (horizontal * vertical).to_string()
}

/// Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
pub fn part2(input: &str) -> String {
    let lines = input.lines();

    let (mut horizontal, mut vertical) = (0, 0);
    let mut aim = 0;

    for line in lines {
        let (direction, value) =
            if let [direction, value] = line.split_whitespace().collect::<Vec<&str>>()[0..2] {
                (direction, value.parse::<i32>().unwrap())
            } else {
                panic!("Invalid line.")
            };

        match direction {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                horizontal += value;
                vertical += value * aim;
            }
            _ => panic!("Invalid direction."),
        }
    }

    (horizontal * vertical).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
            "#
        .trim();

        assert_eq!(part1(input), "150".to_string());
    }

    #[test]
    fn test_part2() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
            "#
        .trim();

        assert_eq!(part2(input), "900".to_string());
    }
}
