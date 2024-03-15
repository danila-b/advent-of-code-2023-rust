mod card;
mod hand;

use crate::hand::Hand;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let file = File::open("input/input_day_7.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);

    let mut hands: Vec<Hand> = vec![];
    let mut total_winnings: usize = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("{}", line);

        let new_hand = Hand::from_str(&line).unwrap();
        let mut new_index: Option<usize> = None;

        // This is akin to bubble sort of array with O(N) complexity on search and insert with shift
        // To optimize it we can use data structures to store values of Hand, one of the most optimal
        // structure for this will be Balanced Binary Trees, especially RB-Tree with amortization
        // because of good average insert complexity of at least O(log N)
        for (index, hand) in hands.iter().enumerate() {
            if new_hand < *hand {
                new_index = Some(index);
                break;
            }
        }

        match new_index {
            Some(index) => hands.insert(index, new_hand),
            None => hands.push(new_hand),
        }
    }

    for (rank, hand) in hands.iter().enumerate() {
        total_winnings += (rank + 1) * hand.bid;
    }

    println!("Total winnings {}", total_winnings);
}
