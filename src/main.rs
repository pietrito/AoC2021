use std::{env, fs, process};

// mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let challenges = [
        ("Day 1 - Part 1", "1", day1::part1 as fn(&str) -> String),
        ("Day 1 - Part 2", "1", day1::part2),
        ("Day 2 - Part 1", "2", day2::part1),
        ("Day 2 - Part 2", "2", day2::part2),
        ("Day 3 - Part 1", "3", day3::part1),
        ("Day 3 - Part 2", "3", day3::part2),
        ("Day 4 - Part 1", "4", day4::part1),
        ("Day 4 - Part 2", "4", day4::part2),
        ("Day 5 - Part 1", "5", day5::part1),
        ("Day 5 - Part 2", "5", day5::part2),
        ("Day 6 - Part 1", "6", day6::part1),
        ("Day 6 - Part 2", "6", day6::part2),
        ("Day 7 - Part 1", "7", day7::part1),
        ("Day 7 - Part 2", "7", day7::part2),
        ("Day 8 - Part 1", "8", day8::part1),
        ("Day 8 - Part 2", "8", day8::part2),
    ];

    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("Error: args is empty.");
        process::exit(exitcode::OSERR);
    } else if args.len() == 1 {
        println!("Running all the days:");
        challenges.iter().for_each(|(name, input_file, function)| {
            println!(
                "{}: {}",
                name,
                function(
                    &fs::read_to_string(format!("inputs/{}", input_file))
                        .expect("Error reading input file.")
                )
            )
        });
    } else if args.len() == 2 {
        if let Ok(day) = args[1].parse::<usize>() {
            println!("Running day {}", day);

            let (name, input_file, function) = challenges[day * 2 - 2];
            println!(
                "{}: {}",
                name,
                function(
                    &fs::read_to_string(format!("inputs/{}", input_file))
                        .expect("Error reading input file.")
                )
            );
            let (name, input_file, function) = challenges[day * 2 - 1];
            println!(
                "{}: {}",
                name,
                function(
                    &fs::read_to_string(format!("inputs/{}", input_file))
                        .expect("Error reading input file.")
                )
            );
        } else {
            eprintln!("Error: '{}' is not a valid day number.", args[1]);
            process::exit(exitcode::USAGE);
        }
    } else {
        println!("Day specification [] is not supported yet.");
        process::exit(exitcode::USAGE);
    }

    process::exit(exitcode::OK);
}
