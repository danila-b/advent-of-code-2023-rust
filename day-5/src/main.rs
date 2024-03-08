mod mappings;
extern crate indicatif;
extern crate rayon;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::mappings::{parsing::parse_file_to_mappings, Mappings};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;

fn main() {
    let file = File::open("input/input_day_5.txt").expect("Failed to open file");
    let mappings: Mappings = parse_file_to_mappings(file);
    let mut lowest_location = usize::MAX;

    // Part 1

    for seed in &mappings.seeds {
        let seed_locations = mappings.find_seed_location(*seed);
        if seed_locations < lowest_location {
            lowest_location = seed_locations;
        }
    }

    println!("Lowest location: {}", lowest_location);

    // Part 2
    // A proper solution would be to map intervals instead of directly bruteforcing all the numbers.
    // But this is an excersice in parallel computing in Rust.

    let total_seeds = mappings.seeds.len() / 2;
    let progress_bar = ProgressBar::new(total_seeds as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let lowest_location = mappings
        .seeds
        .rchunks(2)
        .map(|seed_range| {
            let interval_start = seed_range[0];
            let interval_length = seed_range[1];

            progress_bar.inc(1);

            (0..interval_length)
                .into_par_iter()
                .map(|increment| mappings.find_seed_location(interval_start + increment))
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap_or(usize::MAX);

    println!("Lowest location: {}", lowest_location);
}
