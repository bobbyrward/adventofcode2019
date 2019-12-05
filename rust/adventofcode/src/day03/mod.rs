use crate::solution::Solution;
use std::collections::HashMap;

type WireGrid = HashMap<(i32, i32), i32>;
type WireGridWithSteps = HashMap<(i32, i32), HashMap<i32, i32>>;

mod movement;
mod wire;

use wire::Wire;

#[derive(Debug, Clone)]
pub struct Day03 {}

pub fn create_solution() -> Day03 {
    Day03 {}
}

impl Solution for Day03 {
    fn problem1(&self, input: &str) -> String {
        let wires: Vec<_> = input.lines().map(|l| Wire::new(l)).collect();

        let (grid, intersection_value) = build_grid(&wires);
        let closest = find_closest_intersection(grid, intersection_value);

        closest.unwrap().to_string()
    }

    fn problem2(&self, input: &str) -> String {
        let wires: Vec<_> = input.lines().map(|l| Wire::new(l)).collect();

        let grid = build_grid_with_steps(&wires);
        let closest = find_closest_intersection_by_steps(grid);

        closest.unwrap().to_string()
    }
}

fn build_grid(wires: &[Wire]) -> (WireGrid, i32) {
    let mut grid = WireGrid::new();
    let mut intersection_value = 0;

    for (idx, wire) in wires.iter().enumerate() {
        intersection_value |= 1 << idx;

        for (x, y) in wire.to_coords() {
            *grid.entry((x, y)).or_insert(0) |= 1 << idx;
        }
    }

    (grid, intersection_value)
}

fn build_grid_with_steps(wires: &[Wire]) -> WireGridWithSteps {
    let mut grid = WireGridWithSteps::new();

    for (idx, wire) in wires.iter().enumerate() {
        for ((x, y), steps) in wire.to_coords_with_steps() {
            *grid
                .entry((x, y))
                .or_insert_with(HashMap::new)
                .entry(idx as i32)
                .or_default() = steps;
        }
    }

    grid
}

fn find_closest_intersection_by_steps(grid: WireGridWithSteps) -> Option<i32> {
    let mut closest: Option<i32> = None;

    for ((_x, _y), wires) in &grid {
        if wires.len() > 1 {
            let total_steps = wires.values().sum();
            if let Some(n) = closest {
                if n > total_steps {
                    closest = Some(total_steps);
                }
            } else {
                closest = Some(total_steps);
            }
        }
    }

    closest
}

fn find_closest_intersection(grid: WireGrid, intersection_value: i32) -> Option<i32> {
    let mut closest: Option<i32> = None;

    for ((x, y), value) in &grid {
        if *value == intersection_value {
            let abs_x = x.abs();
            let abs_y = y.abs();

            if let Some(n) = closest {
                if n > (abs_x + abs_y) {
                    closest = Some(abs_x + abs_y);
                }
            } else {
                closest = Some(abs_x + abs_y);
            }
        }
    }

    closest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples() {
        let test_case = |wires: Vec<Wire>, expected| {
            let (g, v) = build_grid(&wires);

            assert_eq!(find_closest_intersection(g, v), expected);
        };

        test_case(
            vec![
                Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                Wire::new("U62,R66,U55,R34,D71,R55,D58,R83"),
            ],
            Some(159),
        );
        test_case(
            vec![
                Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
            ],
            Some(135),
        );
    }

    #[test]
    fn part_two_examples() {
        let test_case = |wires: Vec<Wire>, expected| {
            assert_eq!(
                find_closest_intersection_by_steps(build_grid_with_steps(&wires)),
                expected
            );
        };

        test_case(
            vec![
                Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                Wire::new("U62,R66,U55,R34,D71,R55,D58,R83"),
            ],
            Some(610),
        );
        test_case(
            vec![
                Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
            ],
            Some(410),
        );
    }
}
