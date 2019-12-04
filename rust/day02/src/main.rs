fn run_program(program: &mut [i32]) -> &mut [i32] {
    let mut current_index = 0;

    // println!("pre-run: {:?}", program);
    while current_index < program.len() {
        let opcode = program[current_index];

        match opcode {
            1 => {
                let op1 = program[program[current_index + 1] as usize];
                let op2 = program[program[current_index + 2] as usize];

                program[program[current_index + 3] as usize] = op1 + op2;
            }
            2 => {
                let op1 = program[program[current_index + 1] as usize];
                let op2 = program[program[current_index + 2] as usize];

                program[program[current_index + 3] as usize] = op1 * op2;
            }
            99 => {
                // println!("Halting");
                break;
            }
            n => {
                panic!("Unknown opcode: {}", n);
            }
        }

        current_index += 4;
    }

    // println!("post-run: {:?}", program);
    program
}

fn run_gravity_assist(program: &[i32], noun: i32, verb: i32) -> i32 {
    let mut memory: Vec<i32> = program.iter().copied().collect();

    memory[1] = noun;
    memory[2] = verb;

    run_program(&mut memory);

    memory[0]
}

fn main() {
    let program: Vec<_> = include_str!("input.txt")
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    println!(
        "Gravity Assist 1202 program alarm state = {}",
        run_gravity_assist(&program, 12, 2)
    );

    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_gravity_assist(&program, noun, verb);
            if output == 19_690_720 {
                println!("Found 19690720. Noun = {}, Verb = {}", noun, verb);
                println!("100 * noun + verb = {}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("No answer found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        let test_fn = |actual: &mut [i32], expected: &[i32]| {
            assert_eq!(run_program(actual), expected);
        };

        test_fn(&mut [1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        test_fn(&mut [2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        test_fn(&mut [2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        test_fn(
            &mut [1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
