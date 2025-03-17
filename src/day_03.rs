#![doc = include_str!("descriptions/day_03.md")]

/// Solves star one
/// Sums the priorities
pub fn solve_star_one(input: &str) -> u32 {
    input.lines().map(|l| get_priority_rucksack(l)).sum()
}

/// Solves star two
/// find the equal items in both compartments and give the group the batch
/// then sums the priorities
pub fn solve_star_two(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    // all groups have to consist of the rucksacks
    if lines.len() % 3 != 0 {
        panic!("The input lines have to be a multiple of 3");
    }
    // create chunks that are same as a group
    let group_size = 3;
    let chunks: Vec<Vec<&str>> = lines.chunks(group_size).map(|c| c.to_vec()).collect();

    // loop threw the group
    for group in chunks {
        // find the badges
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                sum += get_priority_item(&c) as u32;
                break;
            }
        }
    }
    sum
}

fn get_priority_item(letter: &char) -> u8 {
    // b'A' -> 65, b'Z' -> 90, b'a' -> 97, b'z' -> 122
    let value = *letter as u8;
    if value >= b'a' && value <= b'z' {
        // Lowercase letters: 'a' (97) -> 1, ..., 'z' (122) -> 26
        value - b'a' + 1
    } else if value >= b'A' && value <= b'Z' {
        // Uppercase letters: 'A' (65) -> 27, ..., 'Z' (90) -> 52
        value - b'A' + 27
    } else {
        panic!("Input must be an alphabetic character");
    }
}

/// Input is a line
fn get_priority_rucksack(input: &str) -> u32 {
    // slice line
    let a = &input[..input.len() / 2];
    let b: &str = &input[input.len() / 2..];
    let mut sum: u32 = 0;

    // check if a item type is in both compartments
    for c in a.chars() {
        if b.contains(c) {
            sum += get_priority_item(&c) as u32;
            break;
        }
    }
    sum
}



#[cfg(test)]
mod tests_day_03 {
    use crate::load_input;

    use super::*;

    #[test]
    fn star_one_example() {
        let input = "\
        vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let sum = solve_star_one(input);
        assert_eq!(sum, 157);
    }

    #[test]
    fn star_one_input() {
        let input = load_input(3);

        let sum = solve_star_one(&input);
        assert_eq!(sum, 8252); // 8252 is the right answer
    }

    #[test]
    fn star_two_example() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

        let sum = solve_star_two(input);
        assert_eq!(sum, 18);
    }

    #[test]
    fn star_two_input() {
        let input = load_input(3);
            

        let sum = solve_star_two(&input);
        assert_eq!(sum, 2828); // 2828 is the right answer
    }
}
