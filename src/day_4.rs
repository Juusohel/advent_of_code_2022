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


        // var names are reversed oops
        let pair1high :i32 = pair1[0].parse().unwrap();
        let pair1low :i32 = pair1[1].parse().unwrap();

        // fill up array with first section
        let mut first_sections: Vec<i32> = vec!();
        for i in pair1high..=pair1low {
            first_sections.push(i);
        }

        let pair2: Vec<&str> = pair2.split('-').collect();

        let pair2high :i32 = pair2[0].parse().unwrap();
        let pair2low :i32 = pair2[1].parse().unwrap();

        // if any overlap
        for i in pair2high..=pair2low {
            if first_sections.contains(&i) {
                total_overlap_pairs = total_overlap_pairs + 1;
                break
            }
        }


    }

    total_overlap_pairs
}