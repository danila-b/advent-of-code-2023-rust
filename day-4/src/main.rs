use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod scratch_card;
use crate::scratch_card::ScratchCard;

fn main() {
    let file = File::open("input/input_day_4.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut cards: HashMap<usize, usize> = HashMap::new();
    let mut points: usize = 0;

    for line in reader.lines().flatten() {
        let scratch_card: ScratchCard =
            ScratchCard::from_str(&line).expect("Failed to parse ScratchCard from line.");

        // Part 1
        points += scratch_card.calculate_points();

        // Part 2
        let number_of_matches = scratch_card.find_winning_numbers().len();

        cards
            .entry(scratch_card.id)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        let current_card_counter = cards.get(&scratch_card.id).unwrap_or(&1).clone();

        for card_id in 1..number_of_matches + 1 {
            cards
                .entry(scratch_card.id + card_id)
                .and_modify(|counter| *counter += current_card_counter)
                .or_insert(current_card_counter);
        }
    }

    let total_number_of_cards: usize = cards.values().sum();

    println!("Number of points in the card pile: {}", points);
    println!("Total number of cards {}", total_number_of_cards);
}
