//! # Day 2: Rock Paper Scissors
//! The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
//! 
//! Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//! 
//! Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//! 
//! The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//! 
//! The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//! 
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//! 
//! For example, suppose you were given the following strategy guide:
//! ```txt
//! A Y
//! B X
//! C Z
//! ```
//! This strategy guide predicts and recommends the following:
//! 
//! In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//! In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//! The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
//! In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//! 
//! What would your total score be if everything goes exactly according to your strategy guide?
//! 
//! Your puzzle answer was 11767. &#9989;
//! 
//! ## Part Two
//! The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
//! 
//! The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//! 
//! In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
//! In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
//! In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
//! 
//! Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
//! 
//! Your puzzle answer was 13886. &#9989;
//! 
//! Both parts of this puzzle are complete! They provide two gold stars: **


/// Solves the first part of the puzzle if the strat is None or false. The first star will be solved, because there is no strategie to follow.
/// 
/// If u enable the strategie you can solve the second part
pub fn solve_stars(input: &str, strat: Option<bool>) -> u32 {
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

/// Converts a "Rock Paper Sissor"-Letter to a score
/// # Example
/// ```ignore
/// evaluate_tool_score("X") // returns 1
/// ``` 
fn evaluate_tool_score(letter: &str) -> u32 {
    match letter {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Can't evaluate a score from the letter: {letter}"),
    }
}

/// Chose the right tool depanding on the opponent and the win condition
fn choose_tool_score(opponent: &str, win_condition: &str) -> u32 {
    match win_condition {
        // lose
        "X" => {
            let score = evaluate_tool_score(&opponent);
            if score == 1 {
                3
            } else {
                score - 1
            }
        }
        // draw
        "Y" => evaluate_tool_score(&opponent),
        // win
        "Z" => {
            let score = evaluate_tool_score(&opponent);
            if score == 3 {
                1
            } else {
                score + 1
            }
        }
        _ => panic!("The input letter {} isn't in the ruleset", win_condition),
    }
}

/// Evaluate the your score in the round
/// Enable strat for the second puzzle
fn evaluate_round(me: &str, opponent: &str, strat: Option<bool>) -> u32 {
    let strat = strat.unwrap_or(false);
    let own_score = match strat {
        true => choose_tool_score(&opponent, &me),
        _ => evaluate_tool_score(&me),
    };
    let opp_score = evaluate_tool_score(&opponent);

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



#[cfg(test)]
mod tests_day_02 {

    use crate::load_input;

    use super::*;

    #[test]
    fn star_one_example() {
        let input = "\
A Y
B X
C Z";
        let result = solve_stars(&input, None);
        assert_eq!(result, 15);
    }

    #[test]
    fn star_one_input() {
        let input = load_input(2);
        let result = solve_stars(&input, None);

        assert_eq!(result, 11767); // 11767 is the right answer
    }

    #[test]
    fn star_two_input() {
        let input = load_input(2);
        let result = solve_stars(&input, Some(true));

        assert_eq!(result, 13886); // 13886 is the right answer
    }
}
