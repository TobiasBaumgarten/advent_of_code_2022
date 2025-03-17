#![doc = include_str!("descriptions/day_07.md")]

use std::{collections::HashMap, vec};

pub fn solve_star_one(input: &str) -> u32 {
    let sys = handle_file_system(&input);
    let max: u32 = 100000;
    let sum: u32 = sys
        .iter()
        .map(|(_, &size)| if size <= max { size } else { 0 })
        .sum();
    sum
}

pub fn solve_star_two(input: &str) -> u32 {
    let needed: i32 = 30_000_000;
    let total_space: i32 = 70_000_000;
    let sys = handle_file_system(input); // Assume this returns a HashMap<String, u32> or similar
    let least_freed = needed - (total_space - *sys.get("/").unwrap() as i32);

    if least_freed < 0 {
        panic!("The calculation of the needed space failed - maybe there is already enought space for the update?")
    }
    let least_freed = least_freed as u32;

    // find the smalest possible folder to delete for the update
    let big_enough = sys
        .iter()
        .filter(|(_, &size)| size > least_freed)
        .map(|(_, &size)| size)
        .min();


    match big_enough {
        Some(size) => size,
        None => panic!("There is something wrong!"),
    }
}

fn handle_file_system(input: &str) -> HashMap<String, u32> {
    let mut current_pos: Vec<&str> = vec!["/"];
    let mut file_system: HashMap<String, u32> = HashMap::new();
    for line in input.lines() {
        // handle the commands

        if line.starts_with("$ ls") || line.starts_with("dir") {
            // these cases haven't any impact
            continue;
        }

        // if the cd commands come up, change the current_pos
        if line.starts_with("$ cd") {
            let dir = &line["& cd ".len()..];
            match dir {
                ".." => {
                    current_pos.pop();
                }
                "/" => {
                    current_pos = vec!["/"];
                }
                other => current_pos.push(other),
            }
            continue;
        }

        // handle the file size stuff

        let file_parts: Vec<&str> = line.split(" ").collect();
        // get the size - the file name isn't important
        let filesize = *file_parts.get(0).expect("The input is wrong");

        let filesize: u32 = filesize.parse().unwrap();

        let mut path = String::new();
        // add to every folder the filesize
        for part in &current_pos {
            // handle the path
            if !path.ends_with('/') && !path.is_empty() {
                path.push('/');
            }
            path.push_str(part);

            match file_system.get_mut(&path) {
                Some(dir) => *dir += filesize, // add the size to the path
                None => { // if the folder path not exists add the path and the size
                    file_system.insert(path.clone(), filesize);
                }
            }
        }
    }
    file_system
}

#[cfg(test)]
mod tests_day_07 {

    use crate::load_input;

    use super::*;

    #[test]
    fn example_star_one() {
        let sum = solve_star_one(EXAMPLE);
        assert_eq!(sum, 95437);
    }

    #[test]
    fn test_star_one() {
        let input = load_input(7);
        let sum = solve_star_one(&input);
        assert_eq!(
            sum, 1443806,
            "The result should be 1443806 but the answer is {:?}",
            sum
        );
    }

    #[test]
    fn test_star_two() {
        let input = load_input(7);
        let sum = solve_star_two(&input);
        assert_eq!(
            sum, 942298,
            "The result should be 942298 but the answer is {:?}",
            sum
        );
    }
}

pub const EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
