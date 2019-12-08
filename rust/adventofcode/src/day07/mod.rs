use crate::intcode::io::{BasicProgramIO, ProgramIO};
use crate::intcode::Program;
use crate::solution::Solution;
use itertools::Itertools;
use log::info;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

struct ChannelIO {
    index: i32,
    input: Receiver<i32>,
    first_input: Option<i32>,
    outputs: Vec<i32>,
    output: Sender<i32>,
}

impl ChannelIO {
    fn new(index: i32, first_input: i32, input: Receiver<i32>) -> (ChannelIO, Receiver<i32>) {
        let (output, next_input) = channel();

        (
            ChannelIO {
                index,
                input,
                first_input: Some(first_input),
                outputs: Vec::new(),
                output,
            },
            next_input,
        )
    }
}

impl ProgramIO for ChannelIO {
    fn get_next_input(&mut self) -> i32 {
        if let Some(first) = self.first_input {
            info!("{} - Sending first input {}", self.index, first);
            self.first_input = None;
            first
        } else {
            info!("{} - Waiting on input", self.index);
            let next_input = self.input.recv().unwrap();
            info!("{} - Received input {}", self.index, next_input);
            next_input
        }
    }

    fn receive_output(&mut self, output: i32) {
        self.outputs.push(output);
        info!("{} - Sending output {}", self.index, output);
        self.output.send(output).unwrap()
    }
}

struct Amplifier {
    code: Vec<i32>,
}

impl Amplifier {
    fn new(code: &[i32]) -> Amplifier {
        Amplifier {
            code: code.iter().copied().collect(),
        }
    }

    fn run(&self, phase_settings: &[i32]) -> i32 {
        let mut io = BasicProgramIO::new(&[phase_settings[0], 0]);
        let mut program = Program::new("0", &self.code);

        program.run(&mut io);

        io = BasicProgramIO::new(&[phase_settings[1], io.outputs()[0]]);
        program = Program::new("1", &self.code);

        program.run(&mut io);

        io = BasicProgramIO::new(&[phase_settings[2], io.outputs()[0]]);
        program = Program::new("2", &self.code);

        program.run(&mut io);

        io = BasicProgramIO::new(&[phase_settings[3], io.outputs()[0]]);
        program = Program::new("3", &self.code);

        program.run(&mut io);

        io = BasicProgramIO::new(&[phase_settings[4], io.outputs()[0]]);
        program = Program::new("4", &self.code);

        program.run(&mut io);

        io.outputs()[0]
    }

    fn spawn_thread(
        &self,
        index: i32,
        phase_setting: i32,
        input: Receiver<i32>,
    ) -> (thread::JoinHandle<()>, Receiver<i32>) {
        let local_code = self.code.clone();
        let (mut io, output) = ChannelIO::new(index, phase_setting, input);

        (
            thread::spawn(move || {
                // &[settings[0], 0]);
                let mut program = Program::new(&format!("Program {}", index), &local_code);

                program.run(&mut io);
            }),
            output,
        )
    }

    fn feedback_loop(&self, phase_settings: &[i32]) -> i32 {
        let (sender, receiver) = channel();
        let (_program1, output1) = self.spawn_thread(0, phase_settings[0], receiver);

        sender.send(0).unwrap();

        let (_program2, output2) = self.spawn_thread(1, phase_settings[1], output1);
        let (_program3, output3) = self.spawn_thread(2, phase_settings[2], output2);
        let (_program4, output4) = self.spawn_thread(3, phase_settings[3], output3);
        let (_program5, output5) = self.spawn_thread(4, phase_settings[4], output4);

        let mut outputs: Vec<i32> = Vec::new();

        loop {
            let out = output5.recv().unwrap();

            if out == std::i32::MIN {
                return *outputs.last().unwrap();
            } else {
                outputs.push(out);

                if sender.send(out).is_err() {
                    return out;
                }

                info!("Output = {:?}", outputs);
            }
        }
    }
}

fn find_highest_output(code: &[i32]) -> i32 {
    let mut current_max = 0;
    let inputs = [0, 1, 2, 3, 4];

    for permutation in inputs.iter().copied().permutations(5) {
        let result = Amplifier::new(code).run(&permutation);

        if result > current_max {
            current_max = result;
        }
    }
    current_max
}

fn find_highest_output_from_feedback(code: &[i32]) -> i32 {
    let mut current_max = 0;
    let inputs = [5, 6, 7, 8, 9];

    for permutation in inputs.iter().copied().permutations(5) {
        let result = Amplifier::new(code).feedback_loop(&permutation);

        if result > current_max {
            current_max = result;
        }
    }
    current_max
}

#[derive(Debug, Clone)]
pub struct Day07 {}

pub fn create_solution() -> Day07 {
    Day07 {}
}

fn load_program(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}
impl Solution for Day07 {
    fn problem1(&self, input: &str) -> String {
        find_highest_output(&load_program(input)).to_string()
    }

    fn problem2(&self, input: &str) -> String {
        find_highest_output_from_feedback(&load_program(input)).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_amp(code: Vec<i32>, inputs: Vec<i32>) -> i32 {
        Amplifier::new(&code).run(&inputs)
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            run_amp(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,],
                vec![4, 3, 2, 1, 0]
            ),
            43210
        );

        assert_eq!(
            run_amp(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                vec![0, 1, 2, 3, 4],
            ),
            54321,
        );
        assert_eq!(
            run_amp(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                vec![1, 0, 4, 3, 2],
            ),
            65210,
        );
    }

    #[test]
    fn test_example_permutations() {
        assert_eq!(
            find_highest_output(&vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ]),
            43210,
        );

        assert_eq!(
            find_highest_output(&vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],),
            54321,
        );
        assert_eq!(
            find_highest_output(&vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],),
            65210,
        );
    }

    #[test]
    fn test_feedback_examples() {
        assert_eq!(
            Amplifier::new(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ])
            .feedback_loop(&[9, 8, 7, 6, 5]),
            139629729
        );
        assert_eq!(
            Amplifier::new(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ])
            .feedback_loop(&[9, 7, 8, 5, 6]),
            18216
        );
    }

    #[test]
    fn test_feedback_example_permutations() {
        assert_eq!(
            find_highest_output_from_feedback(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
        assert_eq!(
            find_highest_output_from_feedback(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }
}