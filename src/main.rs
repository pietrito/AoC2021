use std::{env, process};

mod common;
mod day1;

fn main() {
    let challenges = vec![day1::solve];

    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!("Error: args is empty.");
        process::exit(exitcode::OSERR);
    } else if args.len() == 1 {
        println!("Running all the days:");
        for (i, challenge) in challenges.iter().enumerate() {
            println!("Day {}: {}", i, challenge());
        }
    } else if args.len() == 2 {
        if let Ok(day) = args[1].parse::<usize>() {
            println!("Running day {}", day);
            println!("Result: {}", challenges[day - 1]());
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
