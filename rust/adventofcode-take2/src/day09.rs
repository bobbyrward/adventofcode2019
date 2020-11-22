use anyhow::Result;
use clap::Clap;

use crate::intcode::io::BasicProgramIO;
use crate::intcode::Program;
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

fn part_one() -> Result<String> {
    let mut io = BasicProgramIO::new(&[1]);
    let mut program = Program::from_str("Test Mode", &input("day09")?);
    program.expand();

    program.run(&mut io);

    Ok(format!("{:?}", io.outputs()))
}

fn part_two() -> Result<String> {
    let mut io = BasicProgramIO::new(&[2]);
    let mut program = Program::from_str("Boost Mode", &input("day09")?);
    program.expand();

    program.run(&mut io);

    Ok(format!("{:?}", io.outputs()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_problem1_examples() {}

    #[test]
    fn test_16_digits() {
        let code = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let mut io = BasicProgramIO::new(&[]);
        let mut program = Program::new("Test 16 Digits", &code);
        program.expand();

        program.run(&mut io);

        assert_eq!(io.outputs()[0].to_string().len(), 16);
    }

    #[test]
    fn test_64bit_output() {
        let code = [104, 1125899906842624, 99];
        let mut io = BasicProgramIO::new(&[]);
        let mut program = Program::new("Test 64 bit output", &code);
        program.expand();

        program.run(&mut io);

        assert_eq!(io.outputs()[0], 1125899906842624);
    }

    #[test]
    fn test_quine() {
        let code = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut io = BasicProgramIO::new(&[]);
        let mut program = Program::new("Test Quine", &code);
        program.expand();

        program.run(&mut io);

        assert_eq!(io.outputs(), code);
    }

    #[test]
    fn test_diagnostic() {
        let mut io = BasicProgramIO::new(&[1]);
        let mut program = Program::from_str("Test Run Diagnostic", &crate::inputs::load_input(9));
        program.expand();

        program.run(&mut io);

        assert_eq!(io.outputs(), [3241900951]);
    }
}
