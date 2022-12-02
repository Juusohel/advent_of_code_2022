mod day_1;
mod day_2;

fn main() {
    // Day 1
    println!(
        "DAY 1: Top 3 elves are carrying {} calories",
        day_1::find_fattest_elf("day1_input1.txt")
    );

    // Day 2

    println!(
        "DAY 2: Total score: {}",
        day_2::rps_score("day2_input.txt")
    );

}
