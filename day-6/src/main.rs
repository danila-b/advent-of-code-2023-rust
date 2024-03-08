mod race;

use crate::race::Race;
use std::fs::File;

fn main() {
    // Part 1
    let file = File::open("input/input_day_6.txt").expect("Failed to open file");
    let races: Vec<Race> = Race::parse_rases_from_file(&file, false).unwrap();
    let mut margin_of_error: usize = 1;

    for race in &races {
        let race_wins = race.find_race_wins();
        margin_of_error *= race_wins;
    }

    println!("Margin of error {:?}", margin_of_error);

    // Part 2
    let file = File::open("input/input_day_6.txt").expect("Failed to open file");
    let races: Vec<Race> = Race::parse_rases_from_file(&file, true).unwrap();
    let mut margin_of_error: usize = 1;

    for race in &races {
        let race_wins = race.find_race_wins();
        margin_of_error *= race_wins;
    }

    println!("Margin of error {:?}", margin_of_error);
}
