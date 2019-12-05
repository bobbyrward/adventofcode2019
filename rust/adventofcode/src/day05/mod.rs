mod intcode;

use crate::solution::Solution;
use intcode::io::BasicProgramIO;
use intcode::Program;

#[derive(Debug, Clone)]
pub struct Day05 {}

pub fn create_solution() -> Day05 {
    Day05 {}
}

fn load_program(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

impl Solution for Day05 {
    fn problem1(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[1]);
        let mut program = Program::new(&load_program(input));

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }

    fn problem2(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[5]);
        let mut program = Program::new(&load_program(input));

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }
}
