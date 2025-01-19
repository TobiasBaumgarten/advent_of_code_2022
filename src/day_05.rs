//! # Day 5: Supply Stacks
//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//!
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
//!
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
//!
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
//!
//! After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was PSNRGBTFT.
//!
//! # Part Two
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
//!
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//! Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
//!
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
//!
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was BNTZFPMMW.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **
//!

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

    pub fn top_string(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().expect("msg"))
            .collect()
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
        assert_eq!(ship.top_string(), "CMZ")
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
