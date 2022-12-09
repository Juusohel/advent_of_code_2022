use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;


pub fn sum_priorities(filepath: &str) -> i32 {
    // creating priorities map
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priorities: HashMap<char, i32>= HashMap::new();

    for (i, character) in alpha.chars().enumerate() {
        priorities.insert(character, i as i32 + 1);
    }



    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };

    let mut total_priority: i32 = 0;

    // Reading file contents
    let lines = io::BufReader::new(file).lines();
    let mut sets_of_three: Vec<Vec<String>> = Vec::new();
    let mut count = 0;
    let mut holding_vec: Vec<String> = Vec::new();
    for line in lines {
        holding_vec.push(line.unwrap());


        if count >= 2 {
            count = 0;
            sets_of_three.push(holding_vec.clone());
            holding_vec.clear();
            continue
        } else {
            count = count +1;
        }

    }
    let mut comp_1: Vec<char> = Vec::new();
    let mut comp_2: Vec<char> = Vec::new();
    let mut comp_3: Vec<char> = Vec::new();

    for set in sets_of_three {
        for (i, line) in set.into_iter().enumerate() {
            match i {
                0 => {
                   for char in line.chars() {
                       comp_1.push(char)
                   }
               }
                1 => {
                    for char in line.chars() {
                        comp_2.push(char)
                    }
                }
                2 => {
                    for char in line.chars() {
                        comp_3.push(char)
                    }
                }
                _ => panic!("broke")
            }
        }

        let mut duplicates_vec: Vec<char> = Vec::new();
        // adding number 1 to map
        let mut comp_1_char_map: HashSet<char> = HashSet::new();
        for character in comp_1.clone() {
            comp_1_char_map.insert(character);
        }

        // checking duplicates from number 2 and adding to duplicates
        for character in comp_2.clone() {
            if comp_1_char_map.contains(&character) {
                duplicates_vec.push(character)
            }
        }

        for character in comp_3.clone() {
            if duplicates_vec.contains(&character){
                let added_prio = priorities.get(&character).unwrap();
                total_priority = total_priority + added_prio;
                break
            }
        }
        comp_1.clear();
        comp_2.clear();
        comp_3.clear();
    }


    total_priority
}
