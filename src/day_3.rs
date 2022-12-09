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
    let mut comp_1: Vec<char> = Vec::new();
    let mut comp_2: Vec<char> = Vec::new();
    for line in lines {
        let bags = line.unwrap();

        // Pushing into 2 half arrays
        let half_size = bags.len()/2;
        for (i, character) in bags.chars().enumerate() {
            if i+1 <= half_size {
                comp_1.push(character);
                // println!("adding to 1 {}", character);
            } else {
                // println!("adding to 2 {}", character);
                comp_2.push(character);
            }
        }

        // Adding first half to map
        let mut comp_1_char_map: HashSet<char> = HashSet::new();
        for character in comp_1.clone() {
            comp_1_char_map.insert(character);
        }
        // Checking if there's duplicates in second half add adding to priority total
        for character in comp_2.clone() {
            if comp_1_char_map.contains(&character) {

                let added_priority = priorities.get(&character).unwrap();
                comp_1_char_map.remove(&character);
                total_priority = total_priority + added_priority;
                // println!("Duplicate found: {}, adding value {}", character, added_priority);
            }
        }
        comp_1.clear();
        comp_2.clear();

    }
    total_priority
}
