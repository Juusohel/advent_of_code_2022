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

fn calculate_game_score(opponent_move: char, outcome: char) -> i32 {
    let mut total: i32 = 0;
    match outcome {
        'X' => total = total + 0,
        'Y' => total = total + 3,
        'Z' => total = total + 6,
        _ => panic!("invalid input")
    }

    total = total + calculate_shape_score(opponent_move, outcome);
    return total
}

fn calculate_shape_score(opponent_move: char, outcome: char) -> i32 {
    let shape_score;
    match opponent_move {
        'A' => {
            match outcome {
                'X' => shape_score = 3,
                'Y' => shape_score = 1,
                'Z' => shape_score = 2,
                _ => panic!("invalid input")
            }
        }
        'B' => {
            match outcome {
                'X' => shape_score = 1,
                'Y' => shape_score = 2,
                'Z' => shape_score = 3,
                _ => panic!("invalid input")
            }
        }
        'C' => {
            match outcome {
            'X' => shape_score = 2,
            'Y' => shape_score = 3,
            'Z' => shape_score = 1,
            _ => panic!("invalid input")
            }
        }

        _ => panic!("invalid input")

    }
    shape_score
}