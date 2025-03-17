#![doc = include_str!("descriptions/day_06.md")]

use std::collections::HashSet;

pub fn solve_star_one(input: &str) -> Option<usize> {
    find_marker(input, 4 as usize)
}

pub fn solve_star_two(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

fn find_marker(input: &str, size: usize) -> Option<usize> {
    for index in size..=input.len() {
        let part = &input[index-size..index];
        let set: HashSet<char> = part.chars().collect();
        if set.len() == size {
            return Some(index);
        }
    }
    None
}


#[cfg(test)]
mod tests_day_06 {
    use crate::load_input;

    use super::*;

    #[test]
    fn example_star_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let result = solve_star_one(&input);
        assert_eq!(result, Some(7), "Expected 7 bit got {:?}", result);
    }

    #[test]
    fn example_star_two() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let result = solve_star_two(&input);
        assert_eq!(result, Some(19), "Expected 19 bit got {:?}", result);
    }
    
    #[test]
    fn test_star_one() {
        let input = load_input(6);

        let result = solve_star_one(&input);
        assert_eq!(result, Some(1794), "Expected 1794 bit got {:?}", result); 
    }

    #[test]
    fn test_star_two() {
        let input = load_input(6);

        let result = solve_star_two(&input);
        assert_eq!(result, Some(2851), "Expected 2851 bit got {:?}", result); 
    }
}