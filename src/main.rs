mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    // Day 1
    println!(
        "DAY 1: Top 3 elves are carrying {} calories",
        day_1::find_fattest_elf("day1_input1.txt")
    );

    // Day 2

    println!("DAY 2: Total score: {}", day_2::rps_score("day2_input.txt"));

    println!(
        "DAY 3: Total priority: {}",
        day_3::sum_priorities("day3_input.txt")
    );

    println!(
        "DAY 4: Total overlap pairs: {}",
        day_4::count_overlap_pairs("day4_input.txt")
    );

    println!(
        "DAY 5: Tops of stacks are: {:?}",
        day_5::rearrange_crates("day5_input.txt")
    )
}
