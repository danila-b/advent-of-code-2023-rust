use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod definitions;
mod gear_ratio;
mod parsing;
mod part_numbers_op;

use crate::definitions::part_number::PartNumber;
use crate::definitions::symbol::Symbol;

fn main() {
    let file_path: &str = "./input/input_day_3.txt";

    let file: File = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut symbol_block: VecDeque<Vec<Symbol>> = VecDeque::with_capacity(3);
    let mut numbers_block: VecDeque<Vec<PartNumber>> = VecDeque::with_capacity(3);

    let mut part_number_sum: usize = 0;
    let mut gear_ratio_sum: usize = 0;

    let mut counter: usize = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line of file");

        let (row_numbers, row_symbols) = parsing::parse_line(&line);

        if counter >= 3 {
            symbol_block.pop_front();
            numbers_block.pop_front();
        }

        symbol_block.push_back(row_symbols);
        numbers_block.push_back(row_numbers);

        // Can be optimized to use only one block and one loop iterating by symbols,
        // But oh well, who knew part 2 will be like that ¯\_(ツ)_/¯
        match counter {
            0 => (),
            1 => {
                part_number_sum +=
                    part_numbers_op::sum_part_numbers_in_line(&numbers_block[0], &symbol_block);
                gear_ratio_sum +=
                    gear_ratio::count_gear_ration_in_row(&symbol_block[0], &numbers_block);
            }
            _ => {
                part_number_sum +=
                    part_numbers_op::sum_part_numbers_in_line(&numbers_block[1], &symbol_block);
                gear_ratio_sum +=
                    gear_ratio::count_gear_ration_in_row(&symbol_block[1], &numbers_block);
            }
        }

        counter += 1;
    }

    // Account for the last line
    part_number_sum += part_numbers_op::sum_part_numbers_in_line(&numbers_block[2], &symbol_block);
    gear_ratio_sum += gear_ratio::count_gear_ration_in_row(&symbol_block[0], &numbers_block);

    println!("part_number_sum: {:?}", part_number_sum);
    println!("gear_ratio_sum: {:?}", gear_ratio_sum);
}
