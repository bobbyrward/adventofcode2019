mod instruction;
pub mod io;
mod opcode;
mod parameters;

use log::debug;

use instruction::Instruction;
use io::ProgramIO;
use opcode::ExecutionState;

#[derive(Debug, Clone)]
pub struct Program {
    memory: Vec<i32>,
}

impl Program {
    pub fn new(memory: &[i32]) -> Program {
        Program {
            memory: memory.iter().copied().collect(),
        }
    }

    pub fn run(&mut self, io: &mut impl ProgramIO) {
        let mut address = 0;

        loop {
            let (instruction, size) = Instruction::new(address, &self.memory);

            debug!("Instruction(#{}): {}", address, instruction);
            debug!(
                "pre-execute: target={}, parameters={:?}",
                instruction.target_value(&self.memory),
                instruction.parameter_values(&self.memory),
            );

            let state = instruction.execute(&mut self.memory, io);

            debug!(
                "post-execute: target={}",
                instruction.target_value(&self.memory),
            );
            debug!("Memory: {:?}", self.memory);

            match state {
                ExecutionState::Halt => break,
                ExecutionState::Continue => {
                    address += size;
                }
                ExecutionState::Jump(n) => {
                    address = n;
                }
            };
        }
    }

    #[cfg(test)]
    pub fn memory(&self) -> &[i32] {
        &self.memory
    }
}

impl Default for Program {
    fn default() -> Self {
        Self { memory: Vec::new() }
    }
}

#[cfg(test)]
mod test {
    use super::io::BasicProgramIO;
    use super::*;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn run_program(name: &str, code: &[i32], inputs: &[i32], expected_output: &[i32]) {
        let mut io = BasicProgramIO::new(inputs);
        let mut program = Program::new(code);
        program.run(&mut io);
        assert_eq!(io.outputs(), expected_output, "{}", name);
    }

    #[test]
    fn test_basic_intcode() {
        let test_fn = |actual: &mut [i32], expected: &[i32]| {
            let mut io = BasicProgramIO::new(&[]);
            let mut program = Program::new(actual);
            program.run(&mut io);
            assert_eq!(program.memory(), expected);
        };

        test_fn(&mut [1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        test_fn(&mut [2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        test_fn(&mut [2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        test_fn(
            &mut [1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_intcode_io() {
        let mut io = BasicProgramIO::new(&[14]);
        let mut program = Program::new(&vec![3, 0, 4, 0, 99]);
        program.run(&mut io);

        assert_eq!(program.memory(), &[14, 0, 4, 0, 99]);
        assert_eq!(io.outputs(), &[14]);
    }

    #[test]
    fn test_jmps_0() {
        run_program(
            "jmp using position mode - 0",
            &vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
            &[0],
        );
        run_program(
            "jmp using immediate mode - 0",
            &vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[0],
            &[0],
        );
    }
    #[test]
    fn test_jmps_1() {
        run_program(
            "jmp using position mode - 1",
            &vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[1],
            &[1],
        );

        run_program(
            "jmp using immediate mode - 1",
            &vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[1],
            &[1],
        );
    }

    #[test]
    fn test_cmps() {
        // eq - position mode
        run_program(
            "eq pos mode - true",
            &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[8],
            &[1],
        );
        run_program(
            "eq pos mode - false",
            &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[7],
            &[0],
        );

        // lt - position mode
        run_program(
            "lt pos mode - true",
            &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[7],
            &[1],
        );
        run_program(
            "lt pos mode - false 1",
            &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[8],
            &[0],
        );
        run_program(
            "lt pos mode - false 2",
            &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[9],
            &[0],
        );

        // eq - immediate mode
        run_program(
            "eq immediate mode - true",
            &vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            &[8],
            &[1],
        );
        run_program(
            "eq immediate mode - false",
            &vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            &[7],
            &[0],
        );

        // lt - immediate mode
        run_program(
            "lt immediate mode - true",
            &vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[7],
            &[1],
        );
        run_program(
            "lt immediate mode - false 1",
            &vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[8],
            &[0],
        );
        run_program(
            "lt immediate mode - false 2",
            &vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[9],
            &[0],
        );
    }

    #[test]
    fn test_cmps_lt() {
        init_logging();

        run_program(
            "test_cmps - lt",
            &vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[2],
            &[999],
        );
    }

    #[test]
    fn test_cmps_eq() {
        init_logging();

        run_program(
            "test_cmps - eq",
            &vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[8],
            &[1000],
        );
    }

    #[test]
    fn test_cmps_gt() {
        init_logging();

        run_program(
            "test_cmps - gt",
            &vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[10],
            &[1001],
        );
    }
}
