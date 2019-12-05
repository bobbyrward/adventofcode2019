use clap::{App, Arg, SubCommand};
use std::time::Instant;

mod inputs;
mod solution;

mod day01;
mod day02;
mod day03;

use solution::Solution;

fn load_solution(day: i32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::create_solution()),
        2 => Box::new(day02::create_solution()),
        3 => Box::new(day03::create_solution()),
        _ => panic!("Unknown day"),
    }
}

fn run_problem(solution: Box<dyn Solution>, input: &str, problem: i32) -> String {
    match problem {
        1 => solution.problem1(input),
        2 => solution.problem2(input),
        _ => panic!("Invalid problem"),
    }
}

fn run_solution(day: i32, problem: i32) -> String {
    run_problem(load_solution(day), &inputs::load_input(day), problem)
}

fn main() {
    let matches = App::new("adventofcode")
        .subcommand(
            SubCommand::with_name("problem")
                .arg(
                    Arg::with_name("day")
                        .help("The day to run")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("problem")
                        .help("The problem to run")
                        .possible_values(&["1", "2"])
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("problem", Some(problem_matches)) => {
            let day = problem_matches
                .value_of("day")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let problem = problem_matches
                .value_of("problem")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let start = Instant::now();
            let result = run_solution(day, problem);
            let elapsed = start.elapsed().as_millis();

            println!(
                "Day {} Problem {} solution({}ms): {}",
                day, problem, elapsed, result
            );
        }
        _ => {
            panic!("unknown subcommand");
        }
    }
}
