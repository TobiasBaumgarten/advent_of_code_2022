#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(Option<i32>),
    Multiply(Option<i32>),
}

impl Operation {
    fn from_str(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        let value = match parts.last() {
            Some(&"old") => None,
            Some(value) => Some(value.parse::<i32>().unwrap()),
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
    items: Vec<i32>,
    operation: Operation,
    test: i32,
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
        let starting_items: Vec<i32> = cleared_spaces
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        // get the operation
        let operation = Operation::from_str(parts[5]);

        // get the test
        let test = parts[7].split_whitespace().collect::<Vec<&str>>();
        let test = test.last().unwrap().parse::<i32>().unwrap();

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
    fn inspect_item(&mut self, item: i32) -> (u8, i32) {
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

        item /= 3;

        if item % self.test == 0 {
            return (self.throw_monkey.0, item);
        }
        return (self.throw_monkey.1, item);
    }

    fn inspect_items(&mut self) -> Vec<(u8, i32)> {
        let items = std::mem::take(&mut self.items);

        // Process all items
        items
            .into_iter()
            .map(|item| self.inspect_item(item))
            .collect()
    }

    fn push_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

pub fn solve_star_one(input: &str, rounds: usize) -> u32 {
    let blocks = split_blocks(&input);
    let mut monkeys: Vec<Monkey> = Vec::new();

    dbg!(&blocks);

    for block in blocks {
        let monkey = Monkey::from_str(&block);
        monkeys.push(monkey);
    }

    monkeys.sort_by_key(|m| m.name);

    dbg!(&monkeys);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let changes = {
                let monk = &mut monkeys[i];
                monk.inspect_items()
            };

            for (target, item) in changes {
                monkeys[target as usize].push_item(item);
            }
        }
    }

    monkeys.sort_by_key(|m| -(m.inspected_count as i32));

    dbg!(&monkeys);

    monkeys[0].inspected_count * monkeys[1].inspected_count
}

const EXAMPLE: &str = "\
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
        let result = solve_star_one(EXAMPLE, 20);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_star_one() {
        let input = load_input(11);
        let result = solve_star_one(&input, 20);

        assert_eq!(result, 90294);
    }
}
