#![doc = include_str!("descriptions/day_11.md")]
#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(Option<u64>),
    Multiply(Option<u64>),
}

impl Operation {
    fn from_str(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        let value: Option<u64> = match parts.last() {
            Some(&"old") => None,
            Some(value) => Some(value.parse().unwrap()),
            None => None,
        };

        match parts.iter().nth_back(1) {
            Some(&"*") => Operation::Multiply(value),
            Some(&"+") => Operation::Add(value),
            Some(_) | None => panic!("The format isnt correct"),
        }
    }
}

fn split_blocks(input: &str) -> Vec<&str> {
    if input.contains("\r\n\r\n") {
        input.split("\r\n\r\n").collect()
    } else {
        input.split("\n\n").collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    name: u8,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    throw_monkey: (u8, u8),
    inspected_count: u32,
}

/// a monkey to handle the data
impl Monkey {
    /// converts a `Monkey` from a string
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(|d| d == ':' || d == '\n').collect();

        // get the name
        let name: Vec<&str> = parts[0].split_whitespace().collect();
        let name = name[1].parse::<u8>().unwrap();

        // get the Starting Items
        let cleared_spaces = parts[3].replace(" ", "").replace("\r", "");
        let starting_items: Vec<u64> = cleared_spaces
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        // get the operation
        let operation = Operation::from_str(parts[5]);

        // get the test
        let test = parts[7].split_whitespace().collect::<Vec<&str>>();
        let test: u64 = test.last().unwrap().parse().unwrap();

        // get monkey options
        let test_true = parts[9]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let test_false = parts[11]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();

        Self {
            name,
            items: starting_items,
            operation,
            test,
            throw_monkey: (test_true, test_false),
            inspected_count: 0,
        }
    }

    // increase the inspection count
    fn is_inspecting(&mut self) {
        self.inspected_count += 1;
    }

    /// Proceeds a item with a given worry level
    fn inspect_item(&mut self, item: u64, worry_behavior: fn(u64) -> u64) -> (u8, u64) {
        let mut item = item;
        self.is_inspecting();

        match self.operation {
            Operation::Add(option) => match option {
                Some(value) => item += value,
                None => item += item,
            },
            Operation::Multiply(option) => match option {
                Some(value) => item *= value,
                None => item *= item,
            },
        };

        item = worry_behavior(item);

        if item % self.test == 0 {
            return (self.throw_monkey.0, item);
        }
        return (self.throw_monkey.1, item);
    }

    fn inspect_items(&mut self, worry_behavior: fn(u64) -> u64) -> Vec<(u8, u64)> {
        let items = std::mem::take(&mut self.items);

        // Process all items
        items
            .into_iter()
            .map(|item| self.inspect_item(item, worry_behavior))
            .collect()
    }

    fn push_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

pub fn worry_behavior_div_3(item: u64) -> u64 {
    item / 3
}

pub fn worry_behavior_none(item: u64) -> u64 {
    item
}

pub fn solve_stars(input: &str, rounds: usize, worry_behavior: fn(u64) -> u64) -> u64 {
    let blocks = split_blocks(&input);
    let mut monkeys: Vec<Monkey> = Vec::new();

    // parse the monkeys
    for block in blocks {
        let monkey = Monkey::from_str(&block);
        monkeys.push(monkey);
    }

    // calculate the product of all test divisors
    let modulo: u64 = monkeys.iter().map(|m| m.test).product();

    // sort monkey, so that the index equal to the name
    monkeys.sort_by_key(|m| m.name);

    // lets start
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let changes = {
                // get the current monkey
                let monk = &mut monkeys[i];
                monk.inspect_items(worry_behavior)
            };

            // Apply modulo to each item before passing it on
            let changes = changes
                .into_iter()
                .map(|(target, item)| (target, item % modulo))
                .collect::<Vec<_>>();

            for (target, item) in changes {
                monkeys[target as usize].push_item(item);
            }
        }
    }

    monkeys.sort_by_key(|m| -(m.inspected_count as i32));

    // Cast to u64 for the final result
    (monkeys[0].inspected_count as u64) * (monkeys[1].inspected_count as u64)
}

pub const EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[cfg(test)]
mod tests_day_11 {
    use crate::load_input;

    use super::*;

    #[test]
    fn test_operation_from_str() {
        // first test with int
        let input = "new = old * 19";
        let result = Operation::from_str(&input);
        let expected = Operation::Multiply(Some(19));
        assert_eq!(result, expected);

        let result = Operation::from_str("new = old * old");
        let expected = Operation::Multiply(None);
        assert_eq!(result, expected);

        let result = Operation::from_str("new = old + 3");
        let expected = Operation::Add(Some(3));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_monkey_from_str() {
        let input = "\
Monkey 0:
  Starting items: 66, 59, 64, 51
  Operation: new = old * 3
  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 4";

        let result = Monkey::from_str(&input);
        let expected = Monkey {
            name: 0,
            items: [66, 59, 64, 51].to_vec(),
            operation: Operation::Multiply(Some(3)),
            test: 2,
            inspected_count: 0,
            throw_monkey: (1, 4),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_star_one() {
        let result = solve_stars(EXAMPLE, 20, worry_behavior_div_3);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_star_one() {
        let input = load_input(11);
        let result = solve_stars(&input, 20, worry_behavior_div_3);

        assert_eq!(result, 90294);
    }

    #[test]
    fn test_star_two() {
        let input = load_input(11);
        let result = solve_stars(&input, 10000, worry_behavior_none);

        assert_eq!(result, 18170818354);
    }
}
