use regex::Regex;
use std::collections::HashSet;

const FREQUENZY_FACTOR: i64 = 4000000;

pub fn solve_puzzle_one(input: impl Into<String>, y: i32) -> i32 {
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
    count_none_positions(&sensors, y, min_x, max_x)
}

pub fn solve_puzzle_two(input: impl Into<String>, max: i32) -> i64 {
    let sensors = parse_input(input);

    for sensor in &sensors {
        // only seek the points around the sensor range
        let r = sensor.distance() + 1; // increase range by 1
        let x = sensor.position.0;
        let y = sensor.position.1;

        for point in get_manhatten_border(x, y, r) {
            if !(point.0 >= 0 && point.0 <= max && point.1 >= 0 && point.1 <= max) {
                continue;
            }
            if sensors.iter().all(|s| !s.in_range_point(&point)) {
                return point.0 as i64 * FREQUENZY_FACTOR + point.1 as i64;
            }
        }
    }

    panic!("Ther isn't any position");
}

fn get_manhatten_border(x: i32, y: i32, r: i32) -> HashSet<Point> {
    let mut border = HashSet::new();
    for i in 0..=r {
        // math rotation
        border.insert(Point(x + (r - i), y - i));
        border.insert(Point(x + i, y + (r - i)));
        border.insert(Point(x - i, y - (r - i)));
        border.insert(Point(x - (r - i), y + i));
    }
    border
}

fn count_none_positions(sensors: &Vec<Sensor>, y: i32, min_x: i32, max_x: i32) -> i32 {
    // check beacons on y
    let beacons_on_y: HashSet<i32> = sensors
        .iter()
        .filter(|s| s.beacon.1 == y)
        .map(|s| s.beacon.0)
        .collect();

    // remove beacon count
    let mut count = -(beacons_on_y.len() as i32);

    for x in min_x..=max_x {
        for sensor in sensors {
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

    /// Returns the manhatten distance between the sensor positon and the beacon position.
    fn distance(&self) -> i32 {
        (self.position.0 - self.beacon.0).abs() + (self.position.1 - self.beacon.1).abs()
    }

    /// Checks if the distance between the sensor and the beacon is
    /// smaller then the distance between the sensor and the given point.
    fn in_range(&self, x: impl Into<i32>, y: impl Into<i32>) -> bool {
        let squared = (self.position.0 - x.into()).abs() + (self.position.1 - y.into()).abs();
        squared <= self.distance()
    }

    fn in_range_point(&self, point: &Point) -> bool {
        self.in_range(point.0, point.1)
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
        day_15::{parse_input, solve_puzzle_one, solve_puzzle_two},
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
    fn star_two_example() {
        let sum = solve_puzzle_two(EXAMPLE, 20);
        assert_eq!(sum, 56000011);
    }

    #[test]
    fn star_one_input() {
        let input = load_input(15);
        let count = solve_puzzle_one(input, 2000000);

        assert_eq!(count, 6078701);
    }

    #[test]
    fn star_two_input() {
        let input = load_input(15);
        let sum = solve_puzzle_two(input, 4000000);
        assert_eq!(sum, 12567351400528);
    }
}
