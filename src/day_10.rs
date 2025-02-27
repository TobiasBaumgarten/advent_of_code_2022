use std::collections::HashMap;
use std::str::FromStr;

const CRT_DIMENSIONS: (u8, u8) = (40, 6);

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

struct LabeldInstruction {
    cycle: usize,
    instruction: Instruction,
}

pub fn solve_star_one(input: &str) -> i32 {
    let instructions = parse_input(input);

    let mut register: i32 = 1;
    // let mut cycle = 2;

    let mut milestones: HashMap<i32, i32> =
        HashMap::from([(20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)]);

    for cycle in 1..221 {
        let i_cycle = cycle as i32;
        if milestones.contains_key(&i_cycle) {
            milestones.insert(i_cycle, register);
        }

        let instruction = instructions.iter().find(|&i| i.cycle == cycle);

        if let Some(laInstruction) = instruction {
            match laInstruction.instruction {
                Instruction::Add(value) => register += value,
                Instruction::Noop => (),
            }
        }
    }
    dbg!(&milestones);

    milestones
        .iter()
        .map(|(&key, &value)| (key as i32) * value)
        .sum()
}

fn parse_input(input: &str) -> Vec<LabeldInstruction> {
    let mut cycle = 0;
    input
        .lines()
        .map(|l| {
            let instruction = Instruction::from_str(l).unwrap();
            match instruction {
                Instruction::Add(_) => cycle += 2,
                Instruction::Noop => cycle += 1,
            }
            LabeldInstruction {
                instruction: instruction,
                cycle: cycle,
            }
        })
        .collect()
}

pub fn solve_star_two() {
    todo!("A lot to do");
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
