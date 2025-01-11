use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;

/// used to load the test files easily
fn load_test_file(day: u32) -> String {
    const BASE_PATH: &str = "src/test_files/";
    fs::read_to_string(format!("{BASE_PATH}day_{:02}.txt", day))
        .expect("Test file cannot be opened")
}
