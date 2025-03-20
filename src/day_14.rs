#![doc = include_str!("descriptions/day_14.md")]

use std::collections::HashSet;
use std::str::FromStr;

/// parses the input to coordinate tuples per line
fn parse_input(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            line.replace(" ", "") // replace all whitespaces
                .split("->") // split the coordinates
                .map(|s| parse_coord_string(s)) // convert to tuple
                .collect()
        })
        .collect()
}

/// parses a String with coordinates like ("123,4") to a truple ((123,4))
fn parse_coord_string(coord_str: &str) -> (i32, i32) {
    let splitted: Vec<&str> = coord_str.split(",").collect();

    if splitted.len() != 2 {
        panic!("The coordinates have to be exactly an x and y coordinate separated with a ','");
    }
    let x = splitted[0]
        .parse::<i32>()
        .expect("x cannot be converted to a int");
    let y = splitted[1]
        .parse::<i32>()
        .expect("y cannot be converted to a int");

    (x, y)
}
#[derive(Debug)]
struct Map {
    solid_blocks: HashSet<(i32, i32)>,
    max_y: i32,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths = parse_input(s);
        let mut solid_blocks: HashSet<(i32, i32)> = HashSet::new();

        for path in paths {
            for window in path.windows(2) {
                let (a, b) = (window[0], window[1]);

                // Create a range that works regardless of direction
                let (start, end, is_horizontal) = if a.0 == b.0 {
                    // Vertical line (x values equal)
                    let min_y = a.1.min(b.1);
                    let max_y = a.1.max(b.1);
                    (min_y, max_y, false)
                } else {
                    // Horizontal line (y values equal)
                    let min_x = a.0.min(b.0);
                    let max_x = a.0.max(b.0);
                    (min_x, max_x, true)
                };

                // Inclusive range
                for i in start..=end {
                    if is_horizontal {
                        solid_blocks.insert((i, a.1));
                    } else {
                        solid_blocks.insert((a.0, i));
                    }
                }
            }
        }

        let max_y = solid_blocks.iter().map(|(_, y)| *y).max().unwrap_or(0); // Provide a default in case collection is empty

        Ok(Self {
            solid_blocks,
            max_y,
        })
    }
}

enum FallingResult {
    Falling((i32, i32)),
    Rest,
    Abyss,
}

impl Map {
    #[inline]
    fn is_empty_slot(&self, coord: (i32, i32)) -> bool {
        !self.solid_blocks.contains(&coord)
    }

    fn add_solid(&mut self, coord: (i32, i32)) {
        self.solid_blocks.insert(coord);
    }

    /// tries to find the next falling position.
    /// if it doesn't find the next position it saves the block in the solid list
    fn next(&mut self, current: (i32, i32)) -> FallingResult {
        match current {
            (_, y) if self.max_y < y => FallingResult::Abyss,
            (x, y) if self.is_empty_slot((x, y + 1)) => FallingResult::Falling((x, y + 1)),
            (x, y) if self.is_empty_slot((x - 1, y + 1)) => FallingResult::Falling((x - 1, y + 1)),
            (x, y) if self.is_empty_slot((x + 1, y + 1)) => FallingResult::Falling((x + 1, y + 1)),
            _ => {
                self.add_solid(current);
                FallingResult::Rest
            }
        }
    }
}

pub fn solve_star_one(input: &str) -> usize {
    let mut solid_map = Map::from_str(&input).expect("Coulnd parse Map");
    let source: (i32, i32) = (500, 0);
    let mut count_sand = 0;
    let mut curr_sand = source;

    loop {
        match solid_map.next(curr_sand) {
            FallingResult::Abyss => break,
            FallingResult::Falling(curr) => curr_sand = curr,
            FallingResult::Rest => {
                count_sand += 1;
                curr_sand = source;
            }
        }
    }
    count_sand
}

pub fn solve_star_two(input: &str) -> usize {
    let mut solid_map = Map::from_str(&input).expect("Coulnd parse Map");
    solid_map.max_y += 1; // changed the max y value
    let source: (i32, i32) = (500, 0);
    let mut count_sand = 0;
    let mut curr_sand = source;

    loop {
        match solid_map.next(curr_sand) {
            FallingResult::Abyss => {
                solid_map.add_solid(curr_sand);
                curr_sand = source;
            }
            FallingResult::Falling(curr) => curr_sand = curr,
            FallingResult::Rest => {
                count_sand += 1;
                if curr_sand == source {
                    return count_sand;
                }
                curr_sand = source;
            }
        }
    }
}

pub const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[cfg(test)]
mod tests_day_14 {
    use super::*;
    use crate::load_input;

    #[test]
    fn simple_test_map() {
        let smap = Map::from_str("498,4 -> 498,6").unwrap();
        assert_eq!(smap.solid_blocks.len(), 3);
    }

    #[test]
    fn star_one_example() {
        let r = solve_star_one(EXAMPLE);
        assert_eq!(r, 24);
    }

    #[test]
    fn star_one_input() {
        let r = solve_star_one(&load_input(14));
        assert_eq!(r, 763);
    }

    #[test]
    fn star_two_example() {
        let r = solve_star_two(EXAMPLE);
        assert_eq!(r, 93);
    }

    #[test]
    fn star_two_input() {
        let r = solve_star_two(&load_input(14));
        assert_eq!(r, 23921);
    }
}
