#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

pub enum RPSStrategy {
    Move,
    WinLoseTie,
}

fn str_to_rps(round: &str, strategy: &RPSStrategy) -> (RPS, RPS) {
    let opponent_char = round.as_bytes()[0] as char;
    let player_char = round.as_bytes()[2] as char;
    let opponent = match opponent_char {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissor,
        _ => panic!("unsupported char {}", opponent_char),
    };

    let player = match player_char {
        // Want to lose
        'X' => match strategy {
            RPSStrategy::Move => RPS::Rock,
            RPSStrategy::WinLoseTie => match opponent {
                RPS::Rock => RPS::Scissor,
                RPS::Paper => RPS::Rock,
                RPS::Scissor => RPS::Paper,
            },
        },
        // Want to tie
        'Y' => match strategy {
            RPSStrategy::Move => RPS::Paper,
            RPSStrategy::WinLoseTie => opponent,
        },
        // Want to win
        'Z' => match strategy {
            RPSStrategy::Move => RPS::Scissor,
            RPSStrategy::WinLoseTie => match opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissor,
                RPS::Scissor => RPS::Rock,
            },
        },
        _ => panic!("unsupported char {}", player_char),
    };

    (opponent, player)
}

fn play_rps(opponent: RPS, player: RPS) -> u32 {
    let play_score = match player {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    };

    let result_score = match opponent {
        RPS::Rock => match player {
            RPS::Rock => 3,
            RPS::Paper => 6,
            RPS::Scissor => 0,
        },
        RPS::Paper => match player {
            RPS::Rock => 0,
            RPS::Paper => 3,
            RPS::Scissor => 6,
        },
        RPS::Scissor => match player {
            RPS::Rock => 6,
            RPS::Paper => 0,
            RPS::Scissor => 3,
        },
    };

    play_score + result_score
}

pub fn rps_tournament(
    path: &str,
    strategy: &RPSStrategy,
) -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(path)?;
    let result: u32 = input
        .lines()
        .into_iter()
        .map(|s| str_to_rps(s, strategy))
        .map(|(opponent, player)| play_rps(opponent, player))
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_move() {
        let result = rps_tournament("example.txt", &RPSStrategy::Move).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn example_win_lose_tie() {
        let result = rps_tournament("example.txt", &RPSStrategy::WinLoseTie).unwrap();
        assert_eq!(result, 12);
    }
}
