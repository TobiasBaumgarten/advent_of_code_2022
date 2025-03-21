#![doc = include_str!("descriptions/day_10.md")]

use std::collections::HashMap;
use std::str::FromStr;

const CRT_DIMENSIONS: (usize, usize) = (40, 6);

enum Instruction {
    Add(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "addx" => Ok(Instruction::Add(
                parts[1].parse::<i32>().expect("Invalid number"),
            )),
            "noop" => Ok(Instruction::Noop),
            _ => Err("Invalid format"),
        }
    }
}

struct CPU {
    register: i32,
}

impl CPU {
    fn new() -> Self {
        Self { register: 1 }
    }

    fn compute(&mut self, instruction: Option<&Instruction>) {
        if let Some(la_instruction) = instruction {
            match la_instruction {
                Instruction::Add(value) => self.register += value,
                Instruction::Noop => (),
            }
        }
    }
}

pub fn solve_star_two(input: &str) -> Vec<String> {
    let instructions = parse_input(input);
    let mut cpu = CPU::new();
    let max = instructions.iter().map(|(i, _)| i).max().unwrap();
    let mut crt: Vec<String> = Vec::new();

    for cycle in 1..max + 1 {
        // get the current position
        let position = ((cycle - 1) % CRT_DIMENSIONS.0) as i32;
        if position == 0 {
            // create a new line if we the position is 0
            crt.push(String::new());
        }
        // get the current line
        let line = crt.last_mut().unwrap();

        // draw the sprite
        line.push(draw_sprite(position, &cpu));

        // handle register
        let instruction = instructions.get(&cycle);
        cpu.compute(instruction);
    }
    crt
}

fn draw_sprite(position: i32, cpu: &CPU) -> char {
    if (position - cpu.register).abs() < 2 {
        '#'
    } else {
        '.'
    }
}

pub fn solve_star_one(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut cpu = CPU::new();

    let mut milestones: HashMap<i32, i32> =
        HashMap::from([(20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)]);

    for cycle in 1..221 {
        if milestones.contains_key(&cycle) {
            milestones.insert(cycle, cpu.register);
        }

        // handle register
        let instruction = instructions.get(&(cycle as usize));
        cpu.compute(instruction);
    }

    milestones
        .iter()
        .map(|(&key, &value)| (key as i32) * value)
        .sum()
}

fn parse_input(input: &str) -> HashMap<usize, Instruction> {
    let mut cycle = 0;
    let mut instruction_map: HashMap<usize, Instruction> = HashMap::new();
    for line in input.lines() {
        let instruction = Instruction::from_str(line).unwrap();
        match instruction {
            Instruction::Add(_) => cycle += 2,
            Instruction::Noop => cycle += 1,
        };
        instruction_map.insert(cycle, instruction);
    }
    instruction_map
}

#[cfg(test)]
mod test_day_10 {
    use super::*;
    use crate::load_input;

    #[test]
    fn test_example_star_one() {
        let r = solve_star_one(EXAMPLE);
        assert_eq!(r, 13140);
    }

    #[test]
    fn test_star_one() {
        let input = load_input(10);
        let r = solve_star_one(&input);
        assert_eq!(r, 14760);
    }

    #[test]
    fn test_example_star_two() {
        let expected = "##..##..##..##..##..##..##..##..##..##..";
        let result = solve_star_two(EXAMPLE);
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn test_star_two() {
        let expected = "\
####.####..##..####.###..#..#.###..####.
#....#....#..#.#....#..#.#..#.#..#.#....
###..###..#....###..#..#.#..#.#..#.###..
#....#....#.##.#....###..#..#.###..#....
#....#....#..#.#....#.#..#..#.#.#..#....
####.#.....###.####.#..#..##..#..#.####.";
        let input = load_input(10);
        let result = solve_star_two(&input);
        assert_eq!(expected, result.join("\n"));
    }
}

pub const EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
