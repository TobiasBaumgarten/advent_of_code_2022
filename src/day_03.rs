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

fn sum_pirorities(input: &str) -> u32 {
    input.lines().map(|l| get_priority_rucksack(l)).sum()
}

#[cfg(test)]
mod tests_day_03 {
    use super::*;
    use std::fs;
    const BASE_PATH: &str = "src/test_files/";

    #[test]
    fn star_one_example() {
        let input = "\
        vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let sum = sum_pirorities(input);
        assert_eq!(sum, 157);
    }

    #[test]
    fn star_one_input() {
        let input = fs::read_to_string(format!("{BASE_PATH}day03_input.txt"))
            .expect("Test file cannot be opened");

        let sum = sum_pirorities(&input);
        assert_eq!(sum, 8252); // is the right answer
    }
}
