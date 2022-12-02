use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn rps_score(filepath: &str) -> i32 {
    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };

    // Reading file contents
    let lines = io::BufReader::new(file).lines();

    let mut total: i32 = 0;
    for line in lines {
        let string = line.unwrap();
        let mut chars = string.chars();
        let opponent = chars.next().unwrap();

        // discarding whitespace
        chars.next();
        let player = chars.next().unwrap();
        total = total + calculate_game_score(opponent, player);
    }

    total
}

fn calculate_game_score(opponent_move: char, player_move: char) -> i32 {
    let mut total: i32 = 0;
    match player_move {
        'X' => total = total + 1,
        'Y' => total = total + 2,
        'Z' => total = total + 3,
        _ => panic!("invalid input")
    }

    total = total + calculate_outcome_score(opponent_move, player_move);
    return total
}

fn calculate_outcome_score(opponent_move: char, player_move: char) -> i32 {
    let outcome_score;
    match player_move {
        'X' => {
            match opponent_move {
                'A' => outcome_score = 3,
                'B' => outcome_score = 0,
                'C' => outcome_score = 6,
                _ => panic!("invalid input")
            }
        }
        'Y' => {
            match opponent_move {
                'A' => outcome_score = 6,
                'B' => outcome_score = 3,
                'C' => outcome_score = 0,
                _ => panic!("invalid input")
            }
        }
        'Z' => {
            match opponent_move {
            'A' => outcome_score = 0,
            'B' => outcome_score = 6,
            'C' => outcome_score = 3,
            _ => panic!("invalid input")
            }
        }

        _ => panic!("invalid input")

    }
    outcome_score
}