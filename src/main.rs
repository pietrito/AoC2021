use std::{env, process};

mod common;
mod day1;
mod day2;

fn main() {
    let challenges = [day1::part1, day1::part2, day2::part1, day2::part2];

    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("Error: args is empty.");
        process::exit(exitcode::OSERR);
    } else if args.len() == 1 {
        println!("Running all the days:");
        challenges.iter().for_each(|x| x());
    } else if args.len() == 2 {
        if let Ok(day) = args[1].parse::<usize>() {
            println!("Running day {}", day);

            challenges[day * 2 - 2]();
            challenges[day * 2 - 1]();
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
