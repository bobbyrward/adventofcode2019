pub trait ProgramIO {
    fn get_next_input(&mut self) -> i32;
    fn receive_output(&mut self, output: i32);
}

pub struct BasicProgramIO {
    inputs: Vec<i32>,
    current_input: usize,
    outputs: Vec<i32>,
}

impl BasicProgramIO {
    pub fn new(inputs: &[i32]) -> BasicProgramIO {
        BasicProgramIO {
            inputs: inputs.iter().copied().collect(),
            current_input: 0,
            outputs: Vec::new(),
        }
    }

    pub fn outputs(&self) -> &[i32] {
        &self.outputs
    }
}

impl ProgramIO for BasicProgramIO {
    fn get_next_input(&mut self) -> i32 {
        self.current_input += 1;
        self.inputs[self.current_input - 1]
    }

    fn receive_output(&mut self, output: i32) {
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
