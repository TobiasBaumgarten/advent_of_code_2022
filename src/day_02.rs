fn evaluate_tool_score(letter: &str) -> u32 {
    match letter {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Can't evaluate a score from the letter: {letter}"),
    }
}

fn convert_opponent(letter: &str) -> u32 {
    match letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Can't evaluate opponent from the letter: {letter}"),
    }
}

fn choose_tool_score(opponent: &str, win_condition: &str) -> u32 {
    match win_condition {
        // lose
        "X" => {
            let score = convert_opponent(&opponent);
            if score == 1 {
                3
            } else {
                score - 1
            }
        }
        // draw
        "Y" => convert_opponent(&opponent),
        // win
        "Z" => {
            let score = convert_opponent(&opponent);
            if score == 3 {
                1
            } else {
                score + 1
            }
        }
        _ => panic!("The input letter {} isn't in the ruleset", win_condition),
    }
}

fn evaluate_round(me: &str, opponent: &str, strat: Option<bool>) -> u32 {
    let strat = strat.unwrap_or(false);
    let own_score = match strat {
        true => choose_tool_score(&opponent, &me),
        _ => evaluate_tool_score(&me),
    };
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

pub fn get_score(input: &str, strat: Option<bool>) -> u32 {
    let mut scores: Vec<u32> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        // check all needed letters are there
        if parts.len() < 2 {
            panic!("There are to less values in the line {}", index + 1)
        }

        scores.push(evaluate_round(&parts[1], &parts[0], strat));
    }

    scores.iter().sum()
}

#[cfg(test)]
mod tests_day_02 {

    use crate::load_test_file;

    use super::*;

    #[test]
    fn star_one_example() {
        let input = "\
A Y
B X
C Z";
        let result = get_score(&input, None);
        assert_eq!(result, 15);
    }

    #[test]
    fn star_one_input() {
        let input = load_test_file(2);
        let result = get_score(&input, None);

        assert_eq!(result, 11767); // 11767 is the right answer
    }

    #[test]
    fn star_two_input() {
        let input = load_test_file(2);
        let result = get_score(&input, Some(true));

        assert_eq!(result, 13886); // 13886 is the right answer
    }
}
