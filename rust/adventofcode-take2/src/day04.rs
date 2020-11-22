use anyhow::Result;
use clap::Clap;

use crate::Command;

const INPUT: &str = "206938-679128";

#[derive(Debug, Clap)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

fn password_to_digits(n: i32) -> Vec<i32> {
    let mut output = Vec::with_capacity(6);
    let mut n = n;

    for idx in 0..6 {
        let digit = n / (10i32.pow(5 - idx)) % 10;

        n %= 10i32.pow(5 - idx);

        output.push(digit);
    }

    output
}

fn parse_input() -> (i32, i32) {
    let values: Vec<_> = INPUT
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    (values[0], values[1])
}

#[derive(Debug, Clone, Copy)]
struct Run {
    digit: i32,
    run_length: i32,
}

fn run_length_encode(digits: &[i32]) -> Vec<Run> {
    let mut result = Vec::new();
    let mut current_run: Option<Run> = None;

    for digit in digits {
        if let Some(ref mut current) = current_run {
            if current.digit != *digit {
                result.push(*current);
                current_run = Some(Run {
                    digit: *digit,
                    run_length: 1,
                });
            } else {
                current.run_length += 1;
            }
        } else {
            current_run = Some(Run {
                digit: *digit,
                run_length: 1,
            });
        }
    }

    result.push(current_run.unwrap());

    result
}

fn password_validates(password: i32) -> bool {
    let digits = password_to_digits(password);

    let has_double = run_length_encode(&digits).iter().any(|x| x.run_length > 1);

    let is_rev_sorted = digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(x, y)| y >= x);

    has_double && is_rev_sorted
}

fn password_validates_problem2(password: i32) -> bool {
    let digits = password_to_digits(password);

    let has_double = run_length_encode(&digits).iter().any(|x| x.run_length == 2);

    let is_rev_sorted = digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(x, y)| y >= x);

    has_double && is_rev_sorted
}

fn valid_passwords_in_range(min: i32, max: i32) -> i32 {
    (min..=max).filter(|x| password_validates(*x)).count() as i32
}

fn valid_passwords_in_range_problem2(min: i32, max: i32) -> i32 {
    (min..=max)
        .filter(|x| password_validates_problem2(*x))
        .count() as i32
}

fn part_one() -> Result<String> {
    let (min, max) = parse_input();

    Ok(valid_passwords_in_range(min, max).to_string())
}

fn part_two() -> Result<String> {
    let (min, max) = parse_input();

    Ok(valid_passwords_in_range_problem2(min, max).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_to_digits() {
        assert_eq!(password_to_digits(123456), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_problem_one_examples() {
        assert!(password_validates(111111));
        assert!(!password_validates(223450));
        assert!(!password_validates(123789));
    }

    #[test]
    fn test_problem_two_examples() {
        assert!(password_validates_problem2(112345));
        assert!(password_validates_problem2(112233));
        assert!(!password_validates_problem2(123444));
        assert!(password_validates_problem2(111122));
    }
}
