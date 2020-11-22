pub trait ProgramIO {
    fn get_next_input(&mut self) -> i64;
    fn receive_output(&mut self, output: i64);
}

pub struct BasicProgramIO {
    inputs: Vec<i64>,
    current_input: usize,
    outputs: Vec<i64>,
}

impl BasicProgramIO {
    pub fn new(inputs: &[i64]) -> BasicProgramIO {
        BasicProgramIO {
            inputs: inputs.iter().copied().collect(),
            current_input: 0,
            outputs: Vec::new(),
        }
    }

    pub fn outputs(&self) -> &[i64] {
        &self.outputs
    }
}

impl ProgramIO for BasicProgramIO {
    fn get_next_input(&mut self) -> i64 {
        self.current_input += 1;
        self.inputs[self.current_input - 1]
    }

    fn receive_output(&mut self, output: i64) {
        self.outputs.push(output);
    }
}

impl Default for BasicProgramIO {
    fn default() -> BasicProgramIO {
        BasicProgramIO {
            inputs: Vec::new(),
            current_input: 0,
            outputs: Vec::new(),
        }
    }
}
