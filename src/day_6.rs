use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn processed_characters_count(filepath: &str) -> i32 {
    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        // just one line but eh
        let chars = line.chars();
        let mut checkstring: Vec<char> = Vec::new();

        for (i, char) in chars.enumerate() {
            // if less than 4, still in the beginning
            if i < 13 {
                checkstring.push(char);
                continue;
            }
            checkstring.push(char);
            let unique = check_if_unique_combo(&checkstring);
            if unique {
                return i as i32 + 1;
            }
            let _ = checkstring.remove(0);
        }
    }

    return 0;
}

fn check_if_unique_combo(vec: &Vec<char>) -> bool {
    let mut map: HashSet<&char> = HashSet::new();
    for char in vec {
        if map.contains(&char) {
            // found duplicate char, not unique, returning false
            return false;
        }
        let _ = map.insert(char);
    }

    println!("Found true string {:?}", map);
    true
}
