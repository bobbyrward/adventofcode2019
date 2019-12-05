use log::debug;
use std::fmt;

use super::io::ProgramIO;
use super::opcode::{ExecutionState, OpCode};
use super::parameters::{Parameter, ParameterMode};

#[derive(Debug, Clone)]
pub struct Instruction {
    operation: OpCode,
    param1: Option<Parameter>,
    param2: Option<Parameter>,
    param3: Option<Parameter>,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            operation: OpCode::Halt,
            param1: None,
            param2: None,
            param3: None,
        }
    }
}

impl Instruction {
    fn add(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::Add,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 2),
                memory[(address + 3) as usize],
            )),
        }
    }

    fn mul(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::Multiply,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 2),
                memory[(address + 3) as usize],
            )),
        }
    }

    fn inp(address: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::Input,
            param1: Some(Parameter::new(
                ParameterMode::Address,
                memory[(address + 1) as usize],
            )),
            param2: None,
            param3: None,
        }
    }

    fn outp(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::Output,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: None,
            param3: None,
        }
    }

    fn halt() -> Instruction {
        Instruction {
            operation: OpCode::Halt,
            param1: None,
            param2: None,
            param3: None,
        }
    }

    fn jit(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::JumpIfTrue,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: None,
        }
    }

    fn jif(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::JumpIfFalse,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: None,
        }
    }

    fn lt(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::LessThan,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 2),
                memory[(address + 3) as usize],
            )),
        }
    }

    fn eq(address: i32, opcode: i32, memory: &[i32]) -> Instruction {
        Instruction {
            operation: OpCode::Equals,
            param1: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 0),
                memory[(address + 1) as usize],
            )),
            param2: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 1),
                memory[(address + 2) as usize],
            )),
            param3: Some(Parameter::new(
                ParameterMode::from_opcode(opcode, 2),
                memory[(address + 3) as usize],
            )),
        }
    }

    pub fn new(address: i32, memory: &[i32]) -> (Instruction, i32) {
        let opcode = memory[address as usize];

        match opcode % 100 {
            1 => (Instruction::add(address, opcode, memory), 4),
            2 => (Instruction::mul(address, opcode, memory), 4),
            3 => (Instruction::inp(address, memory), 2),
            4 => {
                let i = Instruction::outp(address, opcode, memory);
                debug!("output = {:?}", i);
                (i, 2)
            }
            5 => (Instruction::jit(address, opcode, memory), 3),
            6 => (Instruction::jif(address, opcode, memory), 3),
            7 => (Instruction::lt(address, opcode, memory), 4),
            8 => (Instruction::eq(address, opcode, memory), 4),
            99 => (Instruction::halt(), 1),
            _ => {
                panic!("Unknown opcode");
            }
        }
    }

    pub fn target_value(&self, memory: &[i32]) -> i32 {
        match self.operation {
            OpCode::Add => *self.param3.unwrap().address(memory),
            OpCode::Multiply => *self.param3.unwrap().address(memory),
            OpCode::Input => *self.param1.unwrap().address(memory),
            OpCode::Output => -1,
            OpCode::Halt => -1,
            OpCode::JumpIfTrue => *self.param2.unwrap().address(memory),
            OpCode::JumpIfFalse => *self.param2.unwrap().address(memory),
            OpCode::LessThan => *self.param3.unwrap().address(memory),
            OpCode::Equals => *self.param3.unwrap().address(memory),
        }
    }

    pub fn parameter_values(&self, memory: &[i32]) -> Vec<i32> {
        match self.operation {
            OpCode::Add => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
            OpCode::Multiply => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
            OpCode::Input => vec![self.param1.unwrap().value(memory)],
            OpCode::Output => vec![self.param1.unwrap().value(memory)],
            OpCode::Halt => vec![],
            OpCode::JumpIfTrue => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
            OpCode::JumpIfFalse => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
            OpCode::LessThan => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
            OpCode::Equals => vec![
                self.param1.unwrap().value(memory),
                self.param2.unwrap().value(memory),
                self.param2.unwrap().value(memory),
            ],
        }
    }

    pub fn execute(&self, memory: &mut [i32], io: &mut impl ProgramIO) -> ExecutionState {
        self.operation
            .execute(memory, io, self.param1, self.param2, self.param3)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.operation {
            OpCode::Add => write!(
                f,
                "({} + {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap(),
                self.param3.unwrap()
            ),
            OpCode::Multiply => write!(
                f,
                "({} * {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap(),
                self.param3.unwrap()
            ),
            OpCode::Input => write!(f, "(Input => {})", self.param1.unwrap(),),
            OpCode::Output => write!(f, "(Output => {})", self.param1.unwrap(),),
            OpCode::Halt => write!(f, "(Halt!)"),
            OpCode::JumpIfTrue => write!(
                f,
                "(JIT {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap()
            ),
            OpCode::JumpIfFalse => write!(
                f,
                "(JIF {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap()
            ),
            OpCode::LessThan => write!(
                f,
                "({} < {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap(),
                self.param3.unwrap()
            ),
            OpCode::Equals => write!(
                f,
                "({} = {} => {})",
                self.param1.unwrap(),
                self.param2.unwrap(),
                self.param3.unwrap()
            ),
        }
    }
}
