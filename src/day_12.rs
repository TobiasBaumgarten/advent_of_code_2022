#![doc = include_str!("descriptions/day_12.md")]

use std::collections::{HashMap, VecDeque};
use std::{thread, usize};
use std::sync::{Arc, Mutex};

pub const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut y_vec: Vec<Vec<u8>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    let mut c: char;

    for (y, line) in input.lines().enumerate() {
        let mut x_vec = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            c = ch;
            if c == 'S' {
                c = 'a';
                start = Some((x, y));
            } else if c == 'E' {
                c = 'z';
                end = Some((x, y));
            }

            x_vec.push(c as u8);
        }
        y_vec.push(x_vec);
    }
    let start = start.expect("the Start has to be defined by an 'S'");
    let end = end.expect("the End has to be defined by an 'E'");
    (y_vec, start, end)
}

fn get_path(
    map: &Vec<Vec<u8>>,
    start: &(usize, usize),
    end: &(usize, usize),
) -> Result<Vec<(usize, usize)>, ()> {
    let height = map.len();
    let width = map[0].len();
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut dequeue: VecDeque<(usize, usize)> = VecDeque::new();
    dequeue.push_back(start.clone());
    let mut visitied: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    visitied.insert(start.clone(), None);

    let mut cur_height: u8;
    let mut neig_heigt: u8;
    let mut next_x: i32;
    let mut next_y: i32;
    let mut next_pos: (usize, usize);

    while let Some((cur_x, cur_y)) = dequeue.pop_front() {
        cur_height = map[cur_y as usize][cur_x as usize];

        // target reached?
        if cur_x == end.0 && cur_y == end.1 {
            let mut pre_node = (cur_x, cur_y);
            let mut path: Vec<(usize, usize)> = Vec::new();
            while let Some(next_node) = visitied.get(&pre_node) {
                path.push(pre_node);
                match next_node {
                    Some(next) => pre_node = *next,
                    None => break,
                }
            }
            return Ok(path);
        }

        // looking for neighbors
        for (dir_x, dir_y) in directions {
            next_x = cur_x as i32 + dir_x;
            next_y = cur_y as i32 + dir_y;

            if next_x < 0 || next_y < 0 || next_x as usize >= width || next_y as usize >= height {
                continue;
            }

            next_pos = (next_x as usize, next_y as usize);

            if visitied.contains_key(&next_pos) {
                continue;
            }

            neig_heigt = map[next_pos.1][next_pos.0];

            if neig_heigt <= cur_height || neig_heigt == cur_height + 1 {
                dequeue.push_back(next_pos);
                visitied.insert(next_pos, Some((cur_x, cur_y)));
            }
        }
    }

    Err(())
}

pub fn solve_star_one(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    let path = get_path(&map, &start, &end);
    if let Ok(path) = path {
        return path.len() - 1;
    }
    return 0;
}

pub fn solve_star_two(input: &str) -> usize {
    let (map, _, end) = parse_input(input);
    // get all a starter positions
    let mut starters: Vec<(usize, usize)> = Vec::new();

    // get all starter positions
    for (y, x_layer) in map.iter().enumerate() {
        for (x, height) in x_layer.iter().enumerate() {
            if *height == b'a' {
                starters.push((x, y));
            }
        }
    }

    // setup for concurrency
    let results = Arc::new(Mutex::new(Vec::new()));
    let map = Arc::new(map); 
    let end = Arc::new(end);
    let mut handles = vec![];
    
    // go threw every starter position
    for start in starters {
        let map_clone = Arc::clone(&map);
        let end_clone = Arc::clone(&end);
        let results_clone = Arc::clone(&results);
        
        // alulate the path in the new thread
        let handle = thread::spawn(move || {
            if let Ok(path) = get_path(&map_clone, &start, &end_clone) {
                let path_len = path.len() - 1;
                let mut results = results_clone.lock().unwrap();
                results.push(path_len);
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    *results.iter().min().unwrap()
}

#[cfg(test)]
mod tests_day_12 {
    use super::*;
    use crate::load_input;

    #[test]
    fn test_star_one_example() {
        let result = solve_star_one(&EXAMPLE);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_star_one_input() {
        let result = solve_star_one(&load_input(12));
        assert_eq!(result, 472);
    }

    #[test]
    fn test_star_two_example() {
        let result = solve_star_two(&EXAMPLE);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_star_two_input() {
        let result = solve_star_two(&load_input(12));
        assert_eq!(result, 465);
    }
}
