#![doc = include_str!("descriptions/day_05.md")]

pub fn solve_star_one(input: &str) -> String {
    solve(input, false)
}

pub fn solve_star_two(input: &str) -> String {
    solve(input, true)
}

fn solve(input: &str, crane_9001: bool) -> String {
    let mut ship = Ship::build(&input, crane_9001);
    ship.run_orders();

    let result: String = ship
        .stacks
        .iter()
        .map(|stack| stack.last().expect("msg"))
        .collect();
    result
}

/// The Ship with the stacks
struct Ship {
    stacks: Vec<Vec<char>>,
    orders: Vec<String>,
    crane_9001: bool,
}

impl Ship {
    /// Build the ship
    pub fn build(input: &str, crane_9001: bool) -> Ship {
        let splitted_lines: Vec<&str> = input.lines().collect();

        let parts: Vec<&[&str]> = splitted_lines.rsplit(|frame| frame.is_empty()).collect();
        if parts.len() != 2 {
            panic!("The build input hasn't the right format");
        }
        // save the orders
        let orders: Vec<String> = parts[0].iter().map(|line| line.to_string()).collect();
        // call init_stacks to build the starting stacks
        let stacks = Ship::init_stacks(parts);

        Ship {
            stacks,
            orders,
            crane_9001,
        }
    }

    // build the initial stacks of the ship
    fn init_stacks(parts: Vec<&[&str]>) -> Vec<Vec<char>> {
        // get the stack info from parts in index 1 in reverse
        let stack_info = parts[1];
        let stack_info: Vec<&str> = stack_info.iter().rev().cloned().collect();
        let mut stack_iter = stack_info.iter();

        // get the info of the length and remvoe it from the iter
        let pos_info = stack_iter
            .next()
            .expect("The build input hasn't the right format");

        // convert the pos_info to numbers, due to better looping
        let pos_info: Vec<i32> = pos_info
            .chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_digit(10).unwrap() as i32
                } else {
                    0
                }
            })
            .collect();

        // get the length of the ship
        let length = *pos_info.iter().max().expect("Something is strange");

        // init the ship with the given lenght as stacks that holds crates (chars)
        let mut stacks = vec![Vec::new(); length as usize];

        // put the infos on the considered stack
        for level_input in stack_iter {
            for (level_part, info) in level_input.chars().zip(pos_info.iter()) {
                // if the info is zero or the level_part is empty - there isn't anythin to put on the stack
                if *info == 0 || level_part == ' ' {
                    continue;
                }

                let stack_ind = info - 1; // the index of the stack
                stacks[stack_ind as usize].push(level_part);
            }
        }

        stacks
    }

    /// Runs all orders from the crane
    pub fn run_orders(&mut self) {
        let orders = self.orders.clone();

        for order in orders {
            self.execute_order_line(&order);
        }
    }

    /// Takes one order input line
    fn execute_order_line(&mut self, input: &str) {
        let parts: Vec<&str> = input.split(" ").collect();
        if parts.len() != 6 {
            panic!("That isn't the correct input line");
        }

        let quantity: u8 = parts[1].parse().expect("Cannot read how many creates");
        let from: u8 = parts[3]
            .parse()
            .expect("Cannot read where to get the creates");
        let to: u8 = parts[5]
            .parse()
            .expect("Cannot read where to set the creates");

        self.moving(from, to, quantity);
    }

    /// Moves a `quantity` of creates `from` a stack `to` a stack
    fn moving(&mut self, from: u8, to: u8, quantity: u8) {
        if from as usize > self.stacks.len() || to as usize > self.stacks.len() {
            panic!("This stack isn't available from: {} | to: {}", from, to);
        }

        let from_stack = &mut self.stacks[(from - 1) as usize];
        if quantity as usize > from_stack.len() {
            panic!("Not enough crates to move from stack {}", from);
        }

        // split crates directly from the source stack
        let creates = from_stack.split_off(from_stack.len() - quantity as usize);

        // push crates to the destination stack
        let to_stack = &mut self.stacks[(to - 1) as usize];
        if self.crane_9001 {
            to_stack.extend(creates);
        } else {
            to_stack.extend(creates.into_iter().rev()); // reverse order
        }
    }
}

#[cfg(test)]
mod tests_day_05 {
    use crate::load_input;

    use super::*;

    const EXAMPLE: &str = "/
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn top_string(ship: Ship) -> String {
        ship.stacks
            .iter()
            .map(|stack| stack.last().expect("msg"))
            .collect()
    }

    #[test]
    fn example_ship_init() {
        let input = EXAMPLE;

        let ship = Ship::build(input, false);
        let result: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(ship.stacks, result)
    }

    #[test]
    fn exemple_execute_first_order() {
        let mut ship = Ship::build(&EXAMPLE, false);
        let line = ship.orders[0].clone();

        let result: Vec<Vec<char>> = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];

        ship.execute_order_line(&line);

        assert_eq!(ship.stacks, result);
    }

    #[test]
    fn example_first_star() {
        let mut ship = Ship::build(EXAMPLE, false);

        ship.run_orders();
        assert_eq!(top_string(ship), "CMZ")
    }

    #[test]
    fn test_star_one_input() {
        let input = load_input(5);

        let result = solve_star_one(&input);

        assert_eq!("PSNRGBTFT", result);
    }

    #[test]
    fn test_star_two_input() {
        let input = load_input(5);

        let result = solve_star_two(&input);

        assert_eq!("BNTZFPMMW", result);
    }
}
