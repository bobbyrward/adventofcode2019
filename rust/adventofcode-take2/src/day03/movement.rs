use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseMovementError;

impl fmt::Display for ParseMovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid movement string")
    }
}

impl Error for ParseMovementError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<ParseIntError> for ParseMovementError {
    fn from(_pie: ParseIntError) -> ParseMovementError {
        ParseMovementError {}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl Movement {
    pub fn to_coords(self) -> Vec<(i32, i32)> {
        match self {
            Movement::Left(n) => (1..=n).map(|_| (-1, 0)).collect(),
            Movement::Right(n) => (1..=n).map(|_| (1, 0)).collect(),
            Movement::Up(n) => (1..=n).map(|_| (0, 1)).collect(),
            Movement::Down(n) => (1..=n).map(|_| (0, -1)).collect(),
        }
    }
    pub fn to_coords_with_steps(self) -> Vec<((i32, i32), i32)> {
        match self {
            Movement::Left(n) => (1..=n).map(|_| ((-1, 0), 1)).collect(),
            Movement::Right(n) => (1..=n).map(|_| ((1, 0), 1)).collect(),
            Movement::Up(n) => (1..=n).map(|_| ((0, 1), 1)).collect(),
            Movement::Down(n) => (1..=n).map(|_| ((0, -1), 1)).collect(),
        }
    }
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseMovementError {});
        }

        let (direction, distance) = s.split_at(1);

        Ok(match direction {
            "L" => Movement::Left(distance.parse::<i32>()?),
            "R" => Movement::Right(distance.parse::<i32>()?),
            "U" => Movement::Up(distance.parse::<i32>()?),
            "D" => Movement::Down(distance.parse::<i32>()?),
            _ => {
                panic!("Not implemented");
            }
        })
    }
}
