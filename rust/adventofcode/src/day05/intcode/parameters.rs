use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ParameterMode {
    Immediate,
    Address,
}

impl ParameterMode {
    pub fn from_opcode(opcode: i32, index: i32) -> ParameterMode {
        (opcode / 10i32.pow((2 + index) as u32) % 10).into()
    }
}

impl From<i32> for ParameterMode {
    fn from(x: i32) -> ParameterMode {
        match x {
            0 => ParameterMode::Address,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    pub fn new(mode: ParameterMode, value: i32) -> Parameter {
        match mode {
            ParameterMode::Immediate => Parameter { mode, value },
            ParameterMode::Address => {
                if value < 0 {
                    panic!("Negative memory address");
                }
                Parameter { mode, value }
            }
        }
    }

    pub fn value(self, memory: &[i32]) -> i32 {
        match self.mode {
            ParameterMode::Immediate => self.value,
            ParameterMode::Address => memory[self.value as usize],
        }
    }

    pub fn address<'a>(&self, memory: &'a [i32]) -> &'a i32 {
        &memory[self.value as usize]
    }

    pub fn address_mut<'a>(&self, memory: &'a mut [i32]) -> &'a mut i32 {
        &mut memory[self.value as usize]
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            mode: ParameterMode::Immediate,
            value: 0,
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.mode {
            ParameterMode::Immediate => write!(f, "\"{}\"", self.value),
            ParameterMode::Address => write!(f, "#{}", self.value),
        }
    }
}
