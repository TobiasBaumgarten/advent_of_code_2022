#![doc = include_str!("descriptions/day_04.md")]

/// Represents a group of Elfes
struct Group(Range, Range);

// Represents the range of one elf
struct Range(u16, u16);

/// Solves the puzzle with the appropiate function
fn solve(input: &str, handle: fn(&Range, &Range) -> bool) -> usize {
    input
        .lines()
        .filter(|line| unzip_and_handle(line, handle))
        .count()
}

/// Solves the first star of day 4
pub fn solve_star_one(input: &str) -> usize {
    solve(&input, contains)
}

/// Solves the second star of day 4
pub fn solve_star_two(input: &str) -> usize {
    solve(&input, overlap)
}

/// Returns true if Range b is fully in Range a
fn contains(a: &Range, b: &Range) -> bool {
    a.0 <= b.0 && a.1 >= b.1
    //gr.0.0 <= gr.1.0 && gr.0.1 >= gr.1.1
}

fn overlap(a: &Range, b: &Range) -> bool {
    a.0 >= b.0 && a.0 <= b.1 || a.1 >= b.0 && a.1 <= b.1
}

/// Unzips the line and checks if one range contains the other
fn unzip_and_handle(line: &str, handle: fn(&Range, &Range) -> bool) -> bool {
    let gr = unzip_group(line).unwrap();

    handle(&gr.0, &gr.1) || handle(&gr.1, &gr.0)
}

/// Unzips the groups
fn unzip_group(line: &str) -> Result<Group, &'static str> {
    let replaced_input = line.replace(",", "-");
    let parts: Vec<&str> = replaced_input.split("-").collect();

    if parts.len() != 4 {
        return Err("The format of the group isn't correct.");
    }

    let parsed_parts: Result<Vec<u16>, _> = parts.iter().map(|p| p.parse::<u16>()).collect();

    match parsed_parts {
        Ok(parsed) => {
            // Retun the two ranges
            Ok(Group(
                Range(parsed[0], parsed[1]),
                Range(parsed[2], parsed[3]),
            ))
        }
        Err(_) => Err("The format of the group isn't correct."),
    }
}

#[cfg(test)]
mod tests_day_04 {
    use crate::load_input;

    use super::*;
    #[test]
    fn test_star_one_example() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_star_one(input), 2);
    }

    #[test]
    fn test_star_one_input() {
        let input = load_input(4);
        assert_eq!(solve_star_one(&input), 560);
    }

    #[test]
    fn test_star_two_input() {
        let input = load_input(4);
        assert_eq!(solve_star_two(&input), 839);
    }
}
