use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ParameterMode {
    Immediate,
    Address,
    Relative,
}

impl ParameterMode {
    pub fn from_opcode(opcode: i64, index: i64) -> ParameterMode {
        (opcode / 10i64.pow((2 + index) as u32) % 10).into()
    }
}

impl From<i64> for ParameterMode {
    fn from(x: i64) -> ParameterMode {
        match x {
            0 => ParameterMode::Address,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parameter {
    mode: ParameterMode,
    value: i64,
}

impl Parameter {
    pub fn new(mode: ParameterMode, value: i64) -> Parameter {
        match mode {
            ParameterMode::Immediate => Parameter { mode, value },
            ParameterMode::Address => {
                if value < 0 {
                    panic!("Negative memory address");
                }
                Parameter { mode, value }
            }
            ParameterMode::Relative => Parameter { mode, value },
        }
    }

    pub fn mode(&self) -> ParameterMode {
        self.mode
    }

    pub fn value(&self) -> i64 {
        self.value
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
            ParameterMode::Relative => write!(f, "=>{}", self.value),
        }
    }
}
