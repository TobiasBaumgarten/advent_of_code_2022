#![doc = include_str!("descriptions/day_09.md")]
use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

impl Direction {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err("Could not wrap Direction"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn build(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn movement(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::None => {}
        };
    }

    fn is_far_away(&self, other: &Position) -> bool {
        let x_dis = (self.x - other.x).abs();
        let y_dis = (self.y - other.y).abs();
        y_dis > 1 || x_dis > 1
    }

    fn get_directions(&self, other: &Position) -> (Direction, Direction) {
        let x_dis = self.x - other.x;
        let y_dis = self.y - other.y;
        let x_dir = match x_dis {
            v if v > 0 => Direction::Right,
            v if v < 0 => Direction::Left,
            _ => Direction::None,
        };

        let y_dir = match y_dis {
            v if v < 0 => Direction::Down,
            v if v > 0 => Direction::Up,
            _ => Direction::None,
        };

        (x_dir, y_dir)
    }
}

pub fn solve_star_one(input: &str) -> u32 {
    // setup the positions
    let mut head_position = Position::build(0, 0);
    let mut tail_position = Position::build(0, 0);
    let mut path: HashSet<Position> = HashSet::new();
    // collect instructions
    let instructions: Vec<(Direction, u32)> = input.lines().map(|line| parse_line(line)).collect();

    // save the initial position
    path.insert(tail_position.clone());

    for (direction, times) in &instructions {
        for _ in 0..*times {
            head_position.movement(&direction);

            tail_movement(&head_position, &mut tail_position);
            // save the tail position in the set (unique)
            path.insert(tail_position.clone());
        }
    }
    path.len() as u32
}

pub fn solve_star_two(input: &str) -> u32 {
    // setup positions
    let mut head_position = Position::build(0, 0);
    let mut rope: Vec<Position> = vec![head_position.clone(); 9];
    let mut path: HashSet<Position> = HashSet::new();

    // collect instructions
    let instructions: Vec<(Direction, u32)> = input.lines().map(|line| parse_line(line)).collect();

    // save the initial position
    path.insert(head_position.clone());

    for (direction, times) in &instructions {
        for _ in 0..*times {
            head_position.movement(&direction); // move the head as before

            let mut prev: Option<&Position> = Some(&head_position); // set initial the previev knot as head

            // loop threw the tails/knots of the rope
            for knot in &mut rope {
                if let Some(prev) = prev {
                    tail_movement(prev, knot);
                }
                prev = Some(knot)
            }
            // save the tail of the rope in the path
            path.insert(rope[8].clone());
        }
    }
    path.len() as u32
}

/// Handles the tail movement
fn tail_movement(head_position: &Position, tail_position: &mut Position) {
    // if isn't far away just skip
    if !head_position.is_far_away(tail_position) {
        return;
    }
    // get the directions and move the tail
    let (direction_1, direction_2) = head_position.get_directions(&*tail_position);
    tail_position.movement(&direction_1);
    tail_position.movement(&direction_2);
}

/// Parse the line in the direction and times
fn parse_line(line: &str) -> (Direction, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let direction = Direction::from_str(parts[0]).unwrap();
    let times: u32 = parts.get(1).unwrap().parse().unwrap();
    (direction, times)
}

pub const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[cfg(test)]
mod tests_day_09 {
    use crate::load_input;

    use super::*;

    #[test]
    fn test_example_star_one() {
        let r = solve_star_one(EXAMPLE);
        assert_eq!(r, 13);
    }

    #[test]
    fn test_star_one() {
        let input = load_input(9);
        let r = solve_star_one(&input);
        assert_eq!(r, 6563);
    }

    #[test]
    fn test_star_two() {
        let input = load_input(9);
        let r = solve_star_two(&input);
        assert_eq!(r, 2653);
    }
}
