mod cube;
mod game;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file_path: &str = "./input/input.txt";

    let file: File = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut valid_games: usize = 0;

    let mut sum_min_ubes_powers: usize = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game: game::Game = game::Game::from_str(&line).unwrap();

                // Part one solution
                if game.check_game() {
                    valid_games += game.id;
                    println!("Valid {:?}", game);
                }

                // Part two solution
                sum_min_ubes_powers += game.find_minimum_cubes_power();
            }
            Err(err) => panic!("Cannot read the line. Error {}", err),
        }
    }

    println!("Sum of valid game ids: {}", valid_games);
    println!("Min cubes power sum: {}", sum_min_ubes_powers);
}
