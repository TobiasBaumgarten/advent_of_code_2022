fn evaluate_tool_score(latter: &str) -> u32 {
    match latter {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Can't evaluate a score from the latter: {latter}"),
    }
}

fn convert_opponent(latter: &str) -> u32 {
    match latter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Can't evaluate opponent from the latter: {latter}"),
    }
}

fn evaluate_round(me: &str, opponent: &str) -> u32 {
    let own_score = evaluate_tool_score(&me);
    let opp_score = convert_opponent(&opponent);

    // draw
    if own_score == opp_score {
        return 3 + own_score;
    }

    // lose
    if own_score < 3 && opp_score == own_score + 1 || own_score == 3 && opp_score == 1 {
        return own_score;
    }
    //  win
    own_score + 6
}

pub fn get_score(input: &str) -> u32 {
    let mut scores: Vec<u32> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        // check all needed latters are there
        if parts.len() < 2 {
            panic!("There are to less values in the line {}", index + 1)
        }

        scores.push(evaluate_round(&parts[1], &parts[0]));
    }

    scores.iter().sum()
}

#[cfg(test)]
mod tests_day_02 {

    use super::*;
    use std::fs;

    const BASE_PATH: &str = "src/test_files/";

    #[test]
    fn star_one_example() {
        let input = "\
A Y
B X
C Z";
        let result = get_score(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn star_one_input() {
        let input = fs::read_to_string(format!("{BASE_PATH}day02_input.txt"))
            .expect("Test file cannot be opend");
        let result = get_score(&input);

        assert_eq!(result, 11767); // 11767 is the right answer
    }
}
