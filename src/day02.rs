use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<String> {
    input.split("\n").map(|s| s.replace(" ", "")).collect()
}

pub fn solve(input: &str, f: fn(String) -> u32) -> u32 {
    parse_input(input)
        .iter()
        .fold(0, |acc, round| acc + f(round.to_string()))
}

pub fn get_round_score_1(round: String) -> u32 {
    let scores = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let mut score = 0;

    let opponent_play = round.chars().nth(0).unwrap();
    let player_play = round.chars().nth(1).unwrap();

    score += scores.get(&player_play).unwrap();
    match opponent_play {
        'A' => {
            // opponent plays rock
            if player_play == 'X' {
                score += 3
            } else if player_play == 'Y' {
                score += 6
            } else {
                score += 0
            }
        }
        'B' => {
            // opponent plays paper
            if player_play == 'X' {
                score += 0
            } else if player_play == 'Y' {
                score += 3
            } else {
                score += 6
            }
        }
        'C' => {
            // opponent plays scissors
            if player_play == 'X' {
                score += 6
            } else if player_play == 'Y' {
                score += 0
            } else {
                score += 3
            }
        }
        _ => panic!("Invalid opponent play"),
    }

    score
}

pub fn get_round_score_2(round: String) -> u32 {
    let round_scores = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let mut score = 0;

    let opponent_play = round.chars().nth(0).unwrap();
    let player_play = round.chars().nth(1).unwrap();

    score += round_scores.get(&player_play).unwrap();

    match opponent_play {
        'A' => {
            // opponent plays rock
            if player_play == 'X' {
                score += 3
            } else if player_play == 'Y' {
                score += 1
            } else {
                score += 2
            }
        }
        'B' => {
            // opponent plays paper
            if player_play == 'X' {
                score += 1
            } else if player_play == 'Y' {
                score += 2
            } else {
                score += 3
            }
        }
        'C' => {
            // opponent plays scissors
            if player_play == 'X' {
                score += 2
            } else if player_play == 'Y' {
                score += 3
            } else {
                score += 1
            }
        }
        _ => panic!("Invalid opponent play"),
    }
    score
}
