use std::cmp::Reverse;

#[derive(Debug)]
struct Elf {
    pub snack_list: Vec<u32>,
}

impl Elf {
    pub fn carrying_total(&self) -> u32 {
        self.snack_list.iter().sum()
    }
}

fn parse_elfes(input: String) -> Vec<Elf> {
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

    if !snacks.is_empty() {
        let elf = Elf { snack_list: snacks };
        elfs.push(elf);
    }
    elfs
}

pub fn most_total_calories(input: String) -> u32 {
    let elfs = parse_elfes(input);
    // use the max function to find the elf with the most calories
    let max_elf = elfs.iter().max_by_key(|elf| elf.carrying_total());
    max_elf.expect("No Elfs in the list").carrying_total()
}

pub fn most_three_elfes_calories(input: String) -> u32 {
    let mut elfes = parse_elfes(input);

    elfes.sort_by_key(|elf| Reverse(elf.carrying_total()));

    let first_three = &elfes[..3];
    dbg!(first_three);
    first_three.iter().map(|elf| elf.carrying_total()).sum()
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const BASE_PATH: &str = "src/test_files/";

    fn star_one_base_test(path: &str, expected: u32) {
        let input: String = fs::read_to_string(path).expect("Test file cannot be opened");
        let result = most_total_calories(input);
        assert_eq!(result, expected);
    }

    fn star_two_base_test(path: &str, expected: u32) {
        let input: String = fs::read_to_string(path).expect("Test file cannot be opened");
        let result = most_three_elfes_calories(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn star_one_example_test() {
        let path = format!("{}{}", BASE_PATH, "day01_example.txt");
        star_one_base_test(&path, 24000);
    }

    #[test]
    fn star_one_main() {
        let path = format!("{}{}", BASE_PATH, "day01_input.txt");
        star_one_base_test(&path, 68292); // 68292 is the right answer
    }

    #[test]
    fn star_two_example() {
        let path = format!("{}{}", BASE_PATH, "day01_example.txt");
        star_two_base_test(&path, 45000); // 68292 is the right answer
    }
}
