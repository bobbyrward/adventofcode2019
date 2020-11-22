use anyhow::Result;
use clap::Clap;

use crate::intcode::io::ProgramIO;
use crate::intcode::Program;
use crate::point::Point;
use crate::{input, Command};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use tracing::info;

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

#[derive(Debug, Clone, Copy)]
/// Generic error just to have an error type to use
struct GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generic error of some kind, stuff broke")
    }
}

impl Error for GenericError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Available colors
enum Color {
    Black,
    White,
    Unknown,
}

/// Convert i64 to Color
impl TryFrom<i64> for Color {
    type Error = GenericError;

    fn try_from(val: i64) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            -1 => Ok(Color::Unknown),
            _ => Err(GenericError {}),
        }
    }
}

/// Convert color to i64
impl From<Color> for i64 {
    fn from(val: Color) -> i64 {
        match val {
            Color::Black => 0,
            Color::White => 1,
            // Color::Unknown => -1,
            Color::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Available directions for the robot
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Available turn directions for the roboto
enum Rotation {
    Clockwise,
    CounterClockwise,
}

/// Convert i64 to Rotation
impl TryFrom<i64> for Rotation {
    type Error = GenericError;

    fn try_from(val: i64) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Rotation::CounterClockwise),
            1 => Ok(Rotation::Clockwise),
            _ => Err(GenericError {}),
        }
    }
}

#[derive(Debug, Clone)]
/// The canvas(hull) to draw on)
struct Canvas {
    width: i64,
    height: i64,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: i64, height: i64) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::Unknown; (width * height) as usize],
        }
    }

    #[allow(dead_code)]
    fn width(&self) -> i64 {
        self.width
    }

    #[allow(dead_code)]
    fn height(&self) -> i64 {
        self.height
    }

    fn paint(&mut self, pos: Point, color: Color) {
        self.pixels[(pos.y * self.width + pos.x) as usize] = match color {
            Color::Unknown => panic!("Trying to paint with Unknown"),
            x => x,
        };
    }

    fn get_pixel(&self, pos: Point) -> Color {
        self.pixels[(pos.y * self.width + pos.x) as usize]
    }

    #[allow(dead_code)]
    fn render(&self) {
        print!("\n\n");
        for y in 0..self.height {
            print!("  ");
            for x in 0..self.width {
                let pixel = self.pixels[(y * self.width + x) as usize];
                print!(
                    "{}",
                    match pixel {
                        Color::Black => " ",
                        Color::White => "#",
                        Color::Unknown => ".",
                    }
                );
            }
            println!();
        }
        println!("\n");
    }

    fn dump(&self) -> &[Color] {
        &self.pixels
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A robit
struct Robot {
    position: Point,
    direction: Direction,
}

impl Robot {
    fn new(starting_position: Point) -> Self {
        Self {
            position: starting_position,
            direction: Direction::Up,
        }
    }

    fn position(&self) -> Point {
        self.position
    }

    #[allow(dead_code)]
    fn direction(&self) -> Direction {
        self.direction
    }

    fn turn(&mut self, rotation: Rotation) {
        self.direction = match self.direction {
            Direction::Up => match rotation {
                Rotation::Clockwise => Direction::Right,
                Rotation::CounterClockwise => Direction::Left,
            },
            Direction::Left => match rotation {
                Rotation::Clockwise => Direction::Up,
                Rotation::CounterClockwise => Direction::Down,
            },
            Direction::Down => match rotation {
                Rotation::Clockwise => Direction::Left,
                Rotation::CounterClockwise => Direction::Right,
            },
            Direction::Right => match rotation {
                Rotation::Clockwise => Direction::Down,
                Rotation::CounterClockwise => Direction::Up,
            },
        }
    }

    fn advance(&mut self) {
        let delta = match self.direction {
            Direction::Up => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Right => Point::new(1, 0),
        };
        info!(
            "Robot => advancing, delta={:?} newpos={:?}",
            delta,
            delta + self.position
        );
        self.position += delta;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// What input is the program expecting
enum IOState {
    WaitingForColor,
    WaitingForDirection,
}

#[derive(Debug)]
/// IO for the robot
struct RobotProgramIO<'a> {
    state: IOState,
    canvas: &'a mut Canvas,
    robot: &'a mut Robot,
}

impl<'a> RobotProgramIO<'a> {
    fn new(canvas: &'a mut Canvas, robot: &'a mut Robot) -> RobotProgramIO<'a> {
        RobotProgramIO {
            state: IOState::WaitingForColor,
            canvas,
            robot,
        }
    }
}
impl<'a> ProgramIO for RobotProgramIO<'a> {
    fn get_next_input(&mut self) -> i64 {
        let position = self.robot.position();
        info!("Input -> current_pos={:?}", position);

        let pixel = self.canvas.get_pixel(position);
        info!("Input -> pixel={:?}", pixel);

        pixel.into()
    }
    fn receive_output(&mut self, output: i64) {
        match self.state {
            IOState::WaitingForColor => {
                info!("Output -> color={:?}", Color::try_from(output).unwrap());
                self.state = IOState::WaitingForDirection;
                self.canvas
                    .paint(self.robot.position(), Color::try_from(output).unwrap());
                self.state = IOState::WaitingForDirection;
                /*
                self.canvas.render();
                std::thread::sleep(std::time::Duration::from_millis(50));
                */
            }
            IOState::WaitingForDirection => {
                info!(
                    "Output -> rotation={:?}",
                    Rotation::try_from(output).unwrap()
                );
                self.state = IOState::WaitingForColor;
                self.robot.turn(Rotation::try_from(output).unwrap());
                info!("Output -> direction={:?}", self.robot.direction(),);
                self.robot.advance();
                info!("Output -> new position={:?}", self.robot.position());
            }
        };
    }
}

fn paint_the_hull(name: &str, code: &str, canvas: &mut Canvas, starting_point: Point) {
    let mut program = Program::from_str(name, code);
    let mut robot = Robot::new(starting_point);
    let mut io = RobotProgramIO::new(canvas, &mut robot);

    program.expand();
    program.run(&mut io);
}

fn part_one() -> Result<String> {
    let mut canvas = Canvas::new(80, 70);
    let start = Point::new(canvas.width(), canvas.height()) / 2;

    paint_the_hull("testing", &input("day11")?, &mut canvas, start);

    canvas.render();

    let painted_count: usize = canvas
        .dump()
        .iter()
        .filter(|x| **x != Color::Unknown)
        .count();

    Ok(format!("Number of painted pixels = {}", painted_count))
}

fn part_two() -> Result<String> {
    let mut canvas = Canvas::new(43, 6);
    canvas.paint(Point::default(), Color::White);
    paint_the_hull("testing", &input("day11")?, &mut canvas, Point::default());

    canvas.render();

    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11_color() {
        assert_eq!(Color::try_from(0).unwrap(), Color::Black);
        assert_eq!(Color::try_from(1).unwrap(), Color::White);

        let black: i64 = Color::Black.into();
        assert_eq!(black, 0);

        let white: i64 = Color::White.into();
        assert_eq!(white, 1);
    }

    #[test]
    fn test_day11_rotation() {
        assert_eq!(Rotation::try_from(0).unwrap(), Rotation::CounterClockwise);
        assert_eq!(Rotation::try_from(1).unwrap(), Rotation::Clockwise);
    }

    #[test]
    fn test_day11_canvas() {
        assert_eq!(Canvas::new(9, 10).width(), 9);
        assert_eq!(Canvas::new(9, 10).height(), 10);
        assert_eq!(Canvas::new(9, 10).dump().len(), 90);

        let mut canvas = Canvas::new(2, 2);
        canvas.paint(Point::new(1, 1), Color::White);
        canvas.paint(Point::new(0, 0), Color::White);
        assert_eq!(
            canvas.dump(),
            &[Color::White, Color::Unknown, Color::Unknown, Color::White]
        );
        assert_eq!(canvas.get_pixel(Point::new(0, 0)), Color::White);
        assert_eq!(canvas.get_pixel(Point::new(1, 0)), Color::Unknown);
        assert_eq!(canvas.get_pixel(Point::new(0, 1)), Color::Unknown);
        assert_eq!(canvas.get_pixel(Point::new(1, 1)), Color::White);

        canvas.paint(Point::new(0, 1), Color::Black);
        canvas.paint(Point::new(1, 0), Color::Black);
        assert_eq!(
            canvas.dump(),
            &[Color::White, Color::Black, Color::Black, Color::White]
        );
        assert_eq!(canvas.get_pixel(Point::new(0, 0)), Color::White);
        assert_eq!(canvas.get_pixel(Point::new(1, 0)), Color::Black);
        assert_eq!(canvas.get_pixel(Point::new(0, 1)), Color::Black);
        assert_eq!(canvas.get_pixel(Point::new(1, 1)), Color::White);
    }

    #[test]
    fn test_day11_robot_turning() {
        let mut robot = Robot::new(Point::default());
        assert_eq!(robot.direction(), Direction::Up);

        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Right);
        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Down);
        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Left);
        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Up);

        robot.turn(Rotation::CounterClockwise);
        assert_eq!(robot.direction(), Direction::Left);
        robot.turn(Rotation::CounterClockwise);
        assert_eq!(robot.direction(), Direction::Down);
        robot.turn(Rotation::CounterClockwise);
        assert_eq!(robot.direction(), Direction::Right);
        robot.turn(Rotation::CounterClockwise);
        assert_eq!(robot.direction(), Direction::Up);
    }

    #[test]
    fn test_day11_robot_advancing() {
        let mut robot = Robot::new(Point::default());
        assert_eq!(robot.direction(), Direction::Up);
        assert_eq!(robot.position(), Point::new(0, 0));

        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Right);
        robot.advance();
        assert_eq!(robot.position(), Point::new(1, 0));
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 0));

        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Down);
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 1));
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 2));

        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Left);
        robot.advance();
        assert_eq!(robot.position(), Point::new(1, 2));
        robot.advance();
        assert_eq!(robot.position(), Point::new(0, 2));

        robot.turn(Rotation::Clockwise);
        assert_eq!(robot.direction(), Direction::Up);
        robot.advance();
        assert_eq!(robot.position(), Point::new(0, 1));
        robot.advance();
        assert_eq!(robot.position(), Point::new(0, 0));
    }

    #[test]
    fn test_day11_robot() {
        let mut robot = Robot::new(Point::new(2, 2));
        assert_eq!(robot.position(), Point::new(2, 2));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(1, 2));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(1, 3));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 3));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 2));

        robot.turn(Rotation::Clockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(3, 2));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(3, 1));

        robot.turn(Rotation::CounterClockwise);
        robot.advance();
        assert_eq!(robot.position(), Point::new(2, 1));
    }
}
