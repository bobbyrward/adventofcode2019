use crate::intcode::io::BasicProgramIO;
use crate::intcode::Program;
use crate::solution::Solution;

#[derive(Debug, Clone)]
pub struct Day09 {}

pub fn create_solution() -> Day09 {
    Day09 {}
}

fn load_program(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

impl Solution for Day09 {
    fn problem1(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[1]);
        let mut program =
            Program::with_capacity("Test Mode", &load_program(input), 1024 * 1024 * 4);

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }

    fn problem2(&self, input: &str) -> String {
        let mut io = BasicProgramIO::new(&[2]);
        let mut program =
            Program::with_capacity("Boost Mode", &load_program(input), 1024 * 1024 * 4);

        program.run(&mut io);

        format!("{:?}", io.outputs())
    }
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
        let mut program = Program::with_capacity("Test 16 Digits", &code, 1024 * 1024 * 4);

        program.run(&mut io);

        assert_eq!(io.outputs()[0].to_string().len(), 16);
    }

    #[test]
    fn test_64bit_output() {
        let code = [104, 1125899906842624, 99];
        let mut io = BasicProgramIO::new(&[]);
        let mut program = Program::with_capacity("Test 64 bit output", &code, 1024 * 1024 * 4);

        program.run(&mut io);

        assert_eq!(io.outputs()[0], 1125899906842624);
    }

    #[test]
    fn test_quine() {
        let code = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut io = BasicProgramIO::new(&[]);
        let mut program = Program::with_capacity("Test Quine", &code, 1024 * 1024 * 4);

        program.run(&mut io);

        assert_eq!(io.outputs(), code);
    }

    #[test]
    fn test_diagnostic() {
        let code = load_program(&crate::inputs::load_input(9));
        let mut io = BasicProgramIO::new(&[1]);
        let mut program = Program::with_capacity("Test Run Diagnostic", &code, 1024 * 1024 * 4);

        program.run(&mut io);

        assert_eq!(io.outputs(), [3241900951]);
    }
}
