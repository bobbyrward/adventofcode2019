use crate::solution::Solution;
use std::iter::{IntoIterator, Iterator};

#[derive(Debug, Clone)]
pub struct Day01 {}

pub fn create_solution() -> Day01 {
    Day01 {}
}

impl Solution for Day01 {
    fn problem1(&self, input: &str) -> String {
        let module_masses: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

        format!(
            "{}",
            total_fuel_required_without_fuel_for_fuel(module_masses.clone())
        )
    }

    fn problem2(&self, input: &str) -> String {
        let module_masses: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

        format!(
            "{}",
            total_fuel_required_with_fuel_for_fuel(module_masses.clone())
        )
    }
}

fn mass_fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn module_fuel_required(mass: i32) -> i32 {
    let mut required = 0;
    let mut current_mass = mass;

    loop {
        let iteration_required = mass_fuel_required(current_mass);

        if iteration_required < 0 {
            return required;
        }

        required += iteration_required;
        current_mass = iteration_required;
    }
}

/// The total fuel required for all modules without taking into account how much
/// fuel the added fuel requires
fn total_fuel_required_without_fuel_for_fuel(module_masses: impl IntoIterator<Item = i32>) -> i32 {
    module_masses.into_iter().map(mass_fuel_required).sum()
}

/// The total fuel required for all modules while taking into account how much
/// fuel the added fuel requires
fn total_fuel_required_with_fuel_for_fuel(module_masses: impl IntoIterator<Item = i32>) -> i32 {
    module_masses.into_iter().map(module_fuel_required).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(mass_fuel_required(12), 2);
        assert_eq!(mass_fuel_required(14), 2);
        assert_eq!(mass_fuel_required(1969), 654);
        assert_eq!(mass_fuel_required(100756), 33583);
    }

    #[test]
    fn test_total_fuel_required() {
        let module_masses: Vec<i32> = vec![12, 14, 1969, 100756];

        assert_eq!(
            total_fuel_required_without_fuel_for_fuel(module_masses),
            2 + 2 + 654 + 33583
        );
    }

    #[test]
    fn test_fuel_required_problem_2() {
        assert_eq!(module_fuel_required(12), 2);
        assert_eq!(module_fuel_required(14), 2);
        assert_eq!(module_fuel_required(1969), 966);
        assert_eq!(module_fuel_required(100756), 50346);
    }

    #[test]
    fn test_total_fuel_required_problem_2() {
        let module_masses: Vec<i32> = vec![12, 14, 1969, 100756];

        assert_eq!(
            total_fuel_required_with_fuel_for_fuel(module_masses),
            2 + 2 + 966 + 50346
        );
    }
}
