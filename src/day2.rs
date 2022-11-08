use crate::common::read_input;

/// Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
pub fn part1() {
    let lines: Vec<String> = read_input::<String>(2);

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

    println!("Day 2 - Part 1: {}", horizontal * vertical);
}

/// Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
pub fn part2() {
    let lines: Vec<String> = read_input::<String>(2);

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

    println!("Day 2 - Part 1: {}", horizontal * vertical);
}
