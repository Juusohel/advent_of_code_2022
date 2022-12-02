use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn find_fattest_elf(filepath: &str) -> i32 {
    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };

    // Reading file contents
    let lines = io::BufReader::new(file).lines();
    let mut elves = Vec::new();
    let mut holding_vec = Vec::new();
    for line in lines {
        let line_string = line.unwrap(); // trusting input file
        if !line_string.is_empty() {
            let item_calories: i32 = line_string.parse().unwrap(); // trusting input again
            holding_vec.push(item_calories)
        } else {
            // Found empty line, divider
            // Pushing contents of holding vec into elves and clearing it for reuse
            elves.push(holding_vec.clone());
            holding_vec.clear();
        }
    }

    // Find the top 3 elves with most calories and and their total
    let mut third_elf;
    let mut top_three = Vec::new();
    for elf in elves {
        let elf_total: i32 = elf.iter().sum();
        if top_three.len() < 3 {
            // Less than three elves
            top_three.push(elf_total)
        } else {
            // Sorting top 3
            top_three.sort();
            // getting lowest total elf
            third_elf = top_three.get(0).unwrap();
            // checking new value vs lowest in array
            if &elf_total > third_elf {
                // replace third elf with new one
                top_three[0] = elf_total
            }
        }
    }
    let final_calories = top_three.iter().sum();
    return final_calories;
}
