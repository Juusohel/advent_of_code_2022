use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn count_overlap_pairs(filepath: &str) -> i32 {

    // Open file
    let file = match File::open(filepath) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(file) => file,
    };

    let mut total_overlap_pairs = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let lineunwrap = line.unwrap();
        let split: Vec<&str> = lineunwrap.split(',').collect();
        let mut pair1 = String::from("");
        let mut pair2 = String::from("");



        let mut count = 0;
        for part in split {
            if count == 0 {
                pair1 = String::from(part)
            } else {
                pair2 = String::from(part)
            }
            count = count + 1;
        }


        let pair1: Vec<&str> = pair1.split('-').collect();


        let pair1high :i32 = pair1[0].parse().unwrap();
        let pair1low :i32 = pair1[1].parse().unwrap();

        let pair2: Vec<&str> = pair2.split('-').collect();

        let pair2high :i32 = pair2[0].parse().unwrap();
        let pair2low :i32 = pair2[1].parse().unwrap();

        // if second set is in first set
        if pair1low <= pair2low && pair1high >= pair2high {
            total_overlap_pairs = total_overlap_pairs + 1;
            continue
        }
        if pair2low <= pair1low && pair2high >= pair1high {
            total_overlap_pairs = total_overlap_pairs + 1;
            continue
        }

    }

    total_overlap_pairs
}