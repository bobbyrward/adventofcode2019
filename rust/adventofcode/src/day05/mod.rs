use crate::intcode::io::BasicProgramIO;
use crate::intcode::Program;
use crate::solution::Solution;

#[derive(Debug, Clone)]
pub struct Day05 {}

pub fn create_solution() -> Day05 {
    Day05 {}
}

fn load_program(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

impl Solution for Day05 {
    fn problem1(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[1]);
        let mut program = Program::new("Day5 - Problem 1", &load_program(input));

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }

    fn problem2(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[5]);
        let mut program = Program::new("Day5 - Problem 2", &load_program(input));

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }
}
