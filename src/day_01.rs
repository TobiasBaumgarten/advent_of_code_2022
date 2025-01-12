//! # Day 1: Calorie Counting
//! Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
//! 
//! To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
//! 
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//! 
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
//! 
//! The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
//! 
//! For example, suppose the Elves finish writing their items' Calories and end up with the following list:
//! 
//! ```txt
//! 1000
//! 2000
//! 3000
//! 
//! 4000
//! 
//! 5000
//! 6000
//! 
//! 7000
//! 8000
//! 9000
//! 
//! 10000
//! ```
//! This list represents the Calories of the food carried by five Elves:
//! 
//! The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
//! The second Elf is carrying one food item with 4000 Calories.
//! The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
//! The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
//! The fifth Elf is carrying one food item with 10000 Calories.
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
//! 
//! Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
//! 
//! Your puzzle answer was 68292. &#9989;
//! 
//! ## Part Two 
//! By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
//! 
//! To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
//! 
//! In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.
//! 
//! Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
//! 
//! Your puzzle answer was 203203. &#9989;
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **

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
