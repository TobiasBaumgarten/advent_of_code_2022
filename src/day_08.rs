#![doc = include_str!("descriptions/day_08.md")]

pub fn solve_star_one(input: &str) -> u32 {
    let trees = parse_trees(input);
    let height = trees.len();
    let width = trees[0].len();
    let mut count: u32 = (2 * height + 2 * width - 4) as u32;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let tree_height = trees[y][x];

            // we assume the tree is visible from all sides and correct that in this interation
            let mut visible_from_left = true;
            let mut visible_from_right = true;
            let mut visible_from_top = true;
            let mut visible_from_bottom = true;

            for y2 in 0..y {
                if trees[y2][x] >= tree_height {
                    visible_from_top = false;
                    break;
                }
            }

            for y2 in y + 1..height {
                if trees[y2][x] >= tree_height {
                    visible_from_bottom = false;
                    break;
                }
            }

            for x2 in 0..x {
                if trees[y][x2] >= tree_height {
                    visible_from_left = false;
                    break;
                }
            }

            for x2 in x + 1..width {
                if trees[y][x2] >= tree_height {
                    visible_from_right = false;
                    break;
                }
            }

            if visible_from_left || visible_from_right || visible_from_top || visible_from_bottom {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_star_two(input: &str) -> u32 {
    let trees = parse_trees(input);
    let height = trees.len();
    let width = trees[0].len();

    let mut max_distance = 0;

    // leave the edges becouse there are anyway zero
    for y in 1..width - 1 {
        for x in 1..height - 1 {
            let mut tree_distances: u32 = 1;
            let tree = trees[y][x];
            let mut distance: u32;

            // see to the top
            distance = 0;
            for y_top in (0..y).rev() {
                distance += 1;
                let line_tree = trees[y_top][x];
                if line_tree >= tree {
                    break;
                }
            }
            tree_distances *= distance;

            // see to the left
            distance = 0;
            for x_left in (0..x).rev() {
                distance += 1;
                let line_tree = trees[y][x_left];
                if line_tree >= tree {
                    break;
                }
            }
            tree_distances *= distance;

            // see to the bottom
            distance = 0;
            for y_bot in y + 1..height {
                distance += 1;
                let line_tree = trees[y_bot][x];
                if line_tree >= tree {
                    break;
                }
            }
            tree_distances *= distance;

            // see to the right
            distance = 0;
            for x_right in x + 1..width {
                distance += 1;
                let line_tree = trees[y][x_right];
                if line_tree >= tree {
                    break;
                }
            }
            tree_distances *= distance;

            max_distance = max_distance.max(tree_distances);
        }
    }
    max_distance
}

fn parse_trees(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests_day_08 {

    use crate::load_input;

    use super::*;

    #[test]
    fn test_example_solve_star_one() {
        let result = solve_star_one(EXAMPLE);
        assert_eq!(result, 21)
    }

    #[test]
    fn test_solve_star_one() {
        let result = solve_star_one(&load_input(8));
        assert_eq!(result, 1779);
    }

    #[test]
    fn test_example_solve_star_two() {
        let result = solve_star_two(EXAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve_star_two() {
        let result = solve_star_two(&load_input(8));
        assert_eq!(result, 172224);
    }
}

pub const EXAMPLE: &str = "\
30373
25512
65332
33549
35390";
