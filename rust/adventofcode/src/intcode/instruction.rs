use std::fmt;

use super::io::ProgramIO;
use super::memory::ProgramMemory;
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
    pub fn new(address: i64, memory: &ProgramMemory) -> (Instruction, i64) {
        let opcode = memory[address];

        match opcode % 100 {
            1 => (Self::binary(OpCode::Add, address, opcode, memory), 4),
            2 => (Self::binary(OpCode::Multiply, address, opcode, memory), 4),
            3 => (Self::nullary(OpCode::Input, address, opcode, memory), 2),
            4 => (Self::nullary(OpCode::Output, address, opcode, memory), 2),
            5 => (Self::unary(OpCode::JumpIfTrue, address, opcode, memory), 3),
            6 => (Self::unary(OpCode::JumpIfFalse, address, opcode, memory), 3),
            7 => (Self::binary(OpCode::LessThan, address, opcode, memory), 4),
            8 => (Self::binary(OpCode::Equals, address, opcode, memory), 4),
            9 => (
                Self::nullary(OpCode::AdjustRelative, address, opcode, memory),
                2,
            ),
            99 => (
                Self {
                    operation: OpCode::Halt,
                    param1: None,
                    param2: None,
                    param3: None,
                },
                1,
            ),
            _ => {
                panic!("Unknown opcode");
            }
        }
    }

    pub fn execute(&self, memory: &mut ProgramMemory, io: &mut impl ProgramIO) -> ExecutionState {
        match self.operation {
            OpCode::Add => {
                *memory.address_mut(self.param3) =
                    memory.value(self.param1) + memory.value(self.param2);
                ExecutionState::Continue
            }
            OpCode::Multiply => {
                *memory.address_mut(self.param3) =
                    memory.value(self.param1) * memory.value(self.param2);
                ExecutionState::Continue
            }
            OpCode::Input => {
                *memory.address_mut(self.param1) = io.get_next_input();
                ExecutionState::Continue
            }
            OpCode::Output => {
                io.receive_output(memory.value(self.param1));
                ExecutionState::Continue
            }
            OpCode::JumpIfTrue => {
                if memory.value(self.param1) != 0 {
                    ExecutionState::Jump(memory.value(self.param2))
                } else {
                    ExecutionState::Continue
                }
            }
            OpCode::JumpIfFalse => {
                if memory.value(self.param1) == 0 {
                    ExecutionState::Jump(memory.value(self.param2))
                } else {
                    ExecutionState::Continue
                }
            }
            OpCode::LessThan => {
                *memory.address_mut(self.param3) =
                    if memory.value(self.param1) < memory.value(self.param2) {
                        1
                    } else {
                        0
                    };
                ExecutionState::Continue
            }
            OpCode::Equals => {
                *memory.address_mut(self.param3) =
                    if memory.value(self.param1) == memory.value(self.param2) {
                        1
                    } else {
                        0
                    };
                ExecutionState::Continue
            }
            OpCode::AdjustRelative => ExecutionState::AdjustRelative(memory.value(self.param1)),
            OpCode::Halt => ExecutionState::Halt,
        }
    }

    fn create_parameter(
        index: i64,
        address: i64,
        opcode: i64,
        memory: &ProgramMemory,
    ) -> Parameter {
        Parameter::new(
            ParameterMode::from_opcode(opcode as i64, index),
            memory[(address + index + 1)],
        )
    }

    fn nullary(
        operation: OpCode,
        address: i64,
        opcode: i64,
        memory: &ProgramMemory,
    ) -> Instruction {
        Instruction {
            operation,
            param1: Some(Instruction::create_parameter(0, address, opcode, memory)),
            param2: None,
            param3: None,
        }
    }

    fn unary(operation: OpCode, address: i64, opcode: i64, memory: &ProgramMemory) -> Instruction {
        Instruction {
            operation,
            param1: Some(Instruction::create_parameter(0, address, opcode, memory)),
            param2: Some(Instruction::create_parameter(1, address, opcode, memory)),
            param3: None,
        }
    }

    fn binary(operation: OpCode, address: i64, opcode: i64, memory: &ProgramMemory) -> Instruction {
        Instruction {
            operation,
            param1: Some(Instruction::create_parameter(0, address, opcode, memory)),
            param2: Some(Instruction::create_parameter(1, address, opcode, memory)),
            param3: Some(Instruction::create_parameter(2, address, opcode, memory)),
        }
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
            OpCode::AdjustRelative => write!(f, "(AdjR => {})", self.param1.unwrap(),),
        }
    }
}
