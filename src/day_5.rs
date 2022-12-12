use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn rearrange_crates(filepath: &str) -> Vec<&str> {
    // Hard-coding initial stacks from puzzle input
    let one: Vec<&str> = vec!["Z", "J", "G"];
    let two: Vec<&str> = vec!["Q", "L", "R", "P", "W", "F", "V", "C"];
    let three: Vec<&str> = vec!["F", "P", "M", "C", "L", "G", "R"];
    let four: Vec<&str> = vec!["L", "F", "B", "W", "P", "H", "M"];
    let five: Vec<&str> = vec!["G", "C", "F", "S", "V", "Q"];
    let six: Vec<&str> = vec!["W", "H", "J", "Z", "M", "Q", "T", "L"];
    let seven: Vec<&str> = vec!["H", "F", "S", "B", "V"];
    let eight: Vec<&str> = vec!["F", "J", "Z", "S"];
    let nine: Vec<&str> = vec!["M", "C", "D", "P", "F", "H", "B", "T"];

    let mut crates = vec![one, two, three, four, five, six, seven, eight, nine];
    let mut holding_vec: Vec<&str> = Vec::new();

    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        // Getting parameters
        // Little bit scuffed since no multiple delimiter split, hence we're just going to split the string on " " and grab the numbers we need from that

        let line_opened: String = line.unwrap();
        let split: Vec<&str> = line_opened.split(" ").collect();
        let amount = split[1];
        let from = split[3];
        let target = split[5];

        let target: usize = target.to_string().parse().unwrap();
        let from: usize = from.to_string().parse().unwrap();

        let amount_int: usize = amount.to_string().parse().unwrap();

        // grab the crates from stack
        for _ in 0..amount_int {
            holding_vec.push(crates[from - 1].pop().unwrap());
        }
        // put them into the the new one in the correct order, clears holding vec
        // println!("Moving amount: {} {:?} from {} to {}", amount, holding_vec, from-1, target-1);
        holding_vec.reverse();
        crates[target - 1].append(&mut holding_vec)
    }

    // print "top" of the stack. Because our vecs are reversed, it's the first entry, not last
    let mut tops_of_stacks: Vec<&str> = Vec::new();
    for stack in crates {
        tops_of_stacks.push(stack[stack.len() - 1])
    }

    return tops_of_stacks;
}
