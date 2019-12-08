use log::debug;

use super::io::ProgramIO;
use super::parameters::Parameter;

#[derive(Debug, Clone)]
pub enum ExecutionState {
    Continue,
    Halt,
    Jump(i32),
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl OpCode {
    pub fn execute(
        self,
        memory: &mut [i32],
        io: &mut impl ProgramIO,
        param1: Option<Parameter>,
        param2: Option<Parameter>,
        param3: Option<Parameter>,
    ) -> ExecutionState {
        match self {
            OpCode::Add => {
                *param3.unwrap().address_mut(memory) =
                    param1.unwrap().value(memory) + param2.unwrap().value(memory);
                ExecutionState::Continue
            }
            OpCode::Multiply => {
                *param3.unwrap().address_mut(memory) =
                    param1.unwrap().value(memory) * param2.unwrap().value(memory);
                ExecutionState::Continue
            }
            OpCode::Input => {
                *param1.unwrap().address_mut(memory) = io.get_next_input();
                ExecutionState::Continue
            }
            OpCode::Output => {
                debug!(
                    "Output: self={:?} param1={:?} param1.value={:?}",
                    self,
                    param1,
                    param1.unwrap().value(memory)
                );
                io.receive_output(param1.unwrap().value(memory));
                ExecutionState::Continue
            }
            OpCode::JumpIfTrue => {
                if param1.unwrap().value(memory) != 0 {
                    ExecutionState::Jump(param2.unwrap().value(memory))
                } else {
                    ExecutionState::Continue
                }
            }
            OpCode::JumpIfFalse => {
                if param1.unwrap().value(memory) == 0 {
                    ExecutionState::Jump(param2.unwrap().value(memory))
                } else {
                    ExecutionState::Continue
                }
            }
            OpCode::LessThan => {
                debug!(
                    "{} < {}",
                    param1.unwrap().value(memory),
                    param2.unwrap().value(memory)
                );
                if param1.unwrap().value(memory) < param2.unwrap().value(memory) {
                    *param3.unwrap().address_mut(memory) = 1;
                } else {
                    *param3.unwrap().address_mut(memory) = 0;
                }

                ExecutionState::Continue
            }
            OpCode::Equals => {
                if param1.unwrap().value(memory) == param2.unwrap().value(memory) {
                    *param3.unwrap().address_mut(memory) = 1;
                } else {
                    *param3.unwrap().address_mut(memory) = 0;
                }

                ExecutionState::Continue
            }
            OpCode::Halt => ExecutionState::Halt,
        }
    }
}
