use crate::intcode::io::BasicProgramIO;
use crate::intcode::Program;

use anyhow::Result;
use clap::Clap;

use crate::{input, Command};

#[derive(Debug, Clap)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

fn load_program(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

fn part_one() -> Result<String> {
    let mut io = BasicProgramIO::new(&[1]);
    let mut program = Program::new("Day5 - Problem 1", &load_program(&input("day05")?));

    program.run(&mut io);

    Ok(format!("{:?}", io.outputs()))
}

fn part_two() -> Result<String> {
    let mut io = BasicProgramIO::new(&[5]);
    let mut program = Program::new("Day5 - Problem 2", &load_program(&input("day05")?));

    program.run(&mut io);

    Ok(format!("{:?}", io.outputs()))
}
