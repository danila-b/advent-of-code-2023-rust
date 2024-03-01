use std::collections::VecDeque;

use crate::definitions::part_number::PartNumber;
use crate::definitions::symbol::{Symbol, SymbolType};

pub fn count_gear_ratio(gear_index: usize, number_block: &VecDeque<Vec<PartNumber>>) -> usize {
    let mut gear_numbers: Vec<usize> = vec![];

    for number_row in number_block {
        for number in number_row {
            if (number.start_idx <= gear_index + 1) && (number.end_idx >= gear_index - 1) {
                gear_numbers.push(number.value.parse::<usize>().unwrap());
            }
        }

        if gear_numbers.len() > 2 {
            return 0;
        }
    }

    if gear_numbers.len() == 2 {
        return gear_numbers[0] * gear_numbers[1];
    }

    0
}

pub fn count_gear_ration_in_row(
    symbol_row: &Vec<Symbol>,
    number_block: &VecDeque<Vec<PartNumber>>,
) -> usize {
    let mut gear_ration_sum: usize = 0;

    for symbol in symbol_row {
        match symbol.variant {
            SymbolType::Gear => gear_ration_sum += count_gear_ratio(symbol.index, number_block),
            SymbolType::Usual => (),
        }
    }

    gear_ration_sum
}
