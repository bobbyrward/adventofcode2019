use std::ops::{Index, IndexMut};

use super::parameters::{Parameter, ParameterMode};

#[derive(Debug, Clone)]
pub struct ProgramMemory {
    memory: Vec<i64>,
    relative_base: i64,
    current_address: i64,
}

impl ProgramMemory {
    pub fn new() -> ProgramMemory {
        ProgramMemory {
            memory: Vec::new(),
            relative_base: 0,
            current_address: 0,
        }
    }

    pub fn from_buffer(buffer: &[i64]) -> ProgramMemory {
        ProgramMemory {
            memory: buffer.iter().copied().collect(),
            relative_base: 0,
            current_address: 0,
        }
    }

    pub fn current_address(&self) -> i64 {
        self.current_address
    }

    pub fn jump(&mut self, value: i64) {
        self.current_address = value;
    }

    pub fn advance(&mut self, value: i64) {
        self.current_address += value;
    }

    pub fn adjust_relative(&mut self, value: i64) {
        self.relative_base += value;
    }

    pub fn value(&self, param: impl ParameterArgument) -> i64 {
        match param.get_parameter_argument_mode() {
            ParameterMode::Immediate => param.get_parameter_argument_value(),
            ParameterMode::Address => self[param.get_parameter_argument_value()],
            ParameterMode::Relative => {
                self[self.relative_base + param.get_parameter_argument_value()]
            }
        }
    }

    pub fn address_mut(&mut self, param: impl ParameterArgument) -> &mut i64 {
        match param.get_parameter_argument_mode() {
            ParameterMode::Immediate => panic!("ref_mut on immediate value"),
            ParameterMode::Address => &mut self[param.get_parameter_argument_value()],
            ParameterMode::Relative => {
                let address = self.relative_base + param.get_parameter_argument_value();
                &mut self[address]
            }
        }
    }

    pub fn expand(&mut self, size: i64) {
        self.memory.resize(size as usize, 0);
    }

    #[cfg(test)]
    pub fn dump(&self) -> &[i64] {
        &self.memory
    }
}

impl Index<i64> for ProgramMemory {
    type Output = i64;

    fn index(&self, idx: i64) -> &Self::Output {
        self.memory.index(idx as usize)
    }
}

impl IndexMut<i64> for ProgramMemory {
    fn index_mut(&mut self, idx: i64) -> &mut Self::Output {
        self.memory.index_mut(idx as usize)
    }
}

pub trait ParameterArgument {
    fn get_parameter_argument_mode(&self) -> ParameterMode;
    fn get_parameter_argument_value(&self) -> i64;
}

impl ParameterArgument for Parameter {
    fn get_parameter_argument_mode(&self) -> ParameterMode {
        self.mode()
    }
    fn get_parameter_argument_value(&self) -> i64 {
        self.value()
    }
}
impl ParameterArgument for Option<Parameter> {
    fn get_parameter_argument_mode(&self) -> ParameterMode {
        self.unwrap().mode()
    }
    fn get_parameter_argument_value(&self) -> i64 {
        self.unwrap().value()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memory_create() {
        let mem = ProgramMemory::new();
        assert_eq!(mem.memory, &[]);

        let mem = ProgramMemory::from_buffer(&[1, 2, 3, 4]);
        assert_eq!(mem.memory, &[1, 2, 3, 4]);

        let v = vec![1, 2, 3, 4];
        let mem = ProgramMemory::from_buffer(&v);
        assert_eq!(mem.memory, &[1, 2, 3, 4]);
    }
}
