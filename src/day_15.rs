use std::collections::HashSet;

use regex::Regex;

fn solve_puzzle_one(input: impl Into<String>, y: i32) -> i32 {
    let sensors = parse_input(input);
    let max_x = sensors
        .iter()
        .map(|s| s.position.0 + s.distance() as i32)
        .max()
        .unwrap();
    let min_x = sensors
        .iter()
        .map(|s| s.position.0 - s.distance() as i32)
        .min()
        .unwrap();

    // check beacons on y
    let beacons_on_y: HashSet<i32> = sensors
        .iter()
        .filter(|s| s.beacon.1 == y)
        .map(|s| s.beacon.0)
        .collect();

    // remove beacon count
    let mut count = -(beacons_on_y.len() as i32);

    for x in min_x..=max_x {
        for sensor in &sensors {
            if sensor.in_range(x, y) {
                count += 1;
                break;
            }
        }
    }
    count
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32);

struct Sensor {
    position: Point,
    beacon: Point,
}

impl Sensor {
    fn new(position: Point, beacon: Point) -> Self {
        Self { position, beacon }
    }

    fn distance(&self) -> i64 {
        (self.position.0 as i64 - self.beacon.0 as i64).abs()
            + (self.position.1 as i64 - self.beacon.1 as i64).abs()
    }

    fn in_range(&self, x: impl Into<i64>, y: impl Into<i64>) -> bool {
        let squared =
            (self.position.0 as i64 - x.into()).abs() + (self.position.1 as i64 - y.into()).abs();
        squared <= self.distance()
    }
}

fn parse_i32(hay: Option<regex::Match<'_>>) -> i32 {
    hay.unwrap().as_str().parse().unwrap()
}

fn parse_input(input: impl Into<String>) -> Vec<Sensor> {
    let input = input.into();
    let re = Regex::new("=(-?\\d*)").unwrap();
    let mut result: Vec<Sensor> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let nums: Vec<i32> = re
            .captures_iter(line)
            .map(|cap| parse_i32(cap.get(1)))
            .collect();

        if nums.len() != 4 {
            panic!(
                "The input of day 15 has some issues - the count should be 4 but is {}\nLine:'{}",
                nums.len(),
                line,
            );
        }

        let sensor = Point(nums[0], nums[1]);
        let beacon = Point(nums[2], nums[3]);
        result.push(Sensor::new(sensor, beacon));
    }
    result
}

#[cfg(test)]
mod tests_day_15 {
    use crate::{
        day_15::{parse_input, solve_puzzle_one},
        load_input,
    };

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_input_parser() {
        let result = parse_input(EXAMPLE);
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn star_one_example() {
        let count = solve_puzzle_one(EXAMPLE, 10);
        assert_eq!(count, 26);
    }

    #[test]
    fn star_one_input() {
        let input = load_input(15);
        let count = solve_puzzle_one(input, 2000000);

        assert_eq!(count, 6078701);
    }
}
