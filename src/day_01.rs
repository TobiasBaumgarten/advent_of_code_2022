#![doc = include_str!("descriptions/day_01.md")]

use std::cmp::Reverse;

/// calculates the most total calories
pub fn solve_star_one(input: &str) -> u32 {
    let elfs = Elf::build(input);
    // use the max function to find the elf with the most calories
    let max_elf = elfs.iter().max_by_key(|elf| elf.carrying_total());
    max_elf.expect("No Elfs in the list").carrying_total()
}

/// find the three elfes with the most calories and returns the sum of the calories
pub fn solve_star_two(input: &str) -> u32 {
    let mut elfes = Elf::build(&input);

    elfes.sort_by_key(|elf| Reverse(elf.carrying_total()));

    let first_three = &elfes[..3];
    dbg!(first_three);
    first_three.iter().map(|elf| elf.carrying_total()).sum()
}

/// Elfs with a snacklist
#[derive(Debug)]
struct Elf {
    pub snack_list: Vec<u32>,
}

impl Elf {

    /// Returns the total amount of calories of this elf
    pub fn carrying_total(&self) -> u32 {
        self.snack_list.iter().sum()
    }

    /// Build a new elfels with the full input
    pub fn build(input: &str) -> Vec<Elf> {
        let mut snacks: Vec<u32> = Vec::new();
        let mut elfs: Vec<Elf> = Vec::new();

        // go threw all lines
        for line in input.lines() {
            if !line.is_empty() {
                // parse the string to an unsignt int
                let snack_calories: u32 = line.parse().expect("Number can't be parsed");
                snacks.push(snack_calories); // collect the calories as u32
                continue;
            }

            // if the line is empty create a new elf and collect
            let elf = Elf { snack_list: snacks };
            elfs.push(elf);
            // the snack collection has to be emtied to have room for a new elf
            snacks = Vec::new();
        }

        // don't forgett the last snack to be add to the last elf
        if !snacks.is_empty() {
            let elf = Elf { snack_list: snacks };
            elfs.push(elf);
        }
        elfs
    }
}




#[cfg(test)]
mod tests_day_01 {
    use crate::load_input;

    use super::*;

    const EXAMPLE: &str = "\
    1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    fn star_one_base_test(input: &str, expected: u32) {
        let result = solve_star_one(input);
        assert_eq!(result, expected);
    }

    fn star_two_base_test(input: &str, expected: u32) {
        let result = solve_star_two(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn star_one_example_test() {
        star_one_base_test(&EXAMPLE, 24000);
    }

    #[test]
    fn star_one_main() {
        let input = load_input(1);
        star_one_base_test(&input, 68292); // 68292 is the right answer
    }

    #[test]
    fn star_two_example() {
        star_two_base_test(&EXAMPLE, 45000); // 68292 is the right answer
    }

    #[test]
    fn star_two_main() {
        let input = load_input(1);
        star_two_base_test(&input, 203203); // 203203 is the right answer
    }
}
