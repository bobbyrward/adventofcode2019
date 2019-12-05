use super::movement::Movement;

#[derive(Debug, Clone)]
pub struct Wire {
    movements: Vec<Movement>,
}

impl Wire {
    pub fn new(s: &str) -> Wire {
        Wire {
            movements: s
                .split(',')
                .map(|m| m.parse::<Movement>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    pub fn to_coords(&self) -> Vec<(i32, i32)> {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut retvalue = Vec::new();

        for (x, y) in self.movements.iter().map(|m| m.to_coords()).flatten() {
            current_x += x;
            current_y += y;

            retvalue.push((current_x, current_y));
        }

        retvalue
    }

    pub fn to_coords_with_steps(&self) -> Vec<((i32, i32), i32)> {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut current_steps = 0;
        let mut retvalue = Vec::new();

        for ((x, y), steps) in self
            .movements
            .iter()
            .map(|m| m.to_coords_with_steps())
            .flatten()
        {
            current_x += x;
            current_y += y;
            current_steps += steps;

            retvalue.push(((current_x, current_y), current_steps));
        }

        retvalue
    }
}
