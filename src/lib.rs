#![doc = include_str!("../README.md")]

use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;

/// Loads the input by a given day
/// # Example
/// ```
/// use advent_of_code_2022::day_01;
/// use advent_of_code_2022::load_input;
///
/// let input = load_input(1);
/// day_01::solve_star_one(&input);
/// ```
pub fn load_input(day: u32) -> String {
    const BASE_PATH: &str = "src/test_files/";
    fs::read_to_string(format!("{BASE_PATH}day_{:02}.txt", day))
        .expect("Test file cannot be opened")
}
