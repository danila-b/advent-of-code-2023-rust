use std::collections::{HashSet, VecDeque};

use crate::definitions::part_number::PartNumber;
use crate::definitions::symbol::Symbol;

pub fn get_all_symbol_idx(symbol_block: &VecDeque<Vec<Symbol>>) -> HashSet<usize> {
    let mut symbol_indexes: HashSet<usize> = HashSet::new();

    for symbol_line in symbol_block {
        for symbol in symbol_line {
            symbol_indexes.insert(symbol.index);
        }
    }

    symbol_indexes
}

pub fn sum_part_numbers_in_line(
    row_numbers: &Vec<PartNumber>,
    symbol_block: &VecDeque<Vec<Symbol>>,
) -> usize {
    let mut sum: usize = 0;
    let symbol_indexes = get_all_symbol_idx(&symbol_block);

    for number in row_numbers {
        for index in number.start_idx.saturating_sub(1)..number.end_idx + 2 {
            if symbol_indexes.contains(&index) {
                sum += number.value.parse::<usize>().unwrap();
                break;
            }
        }
    }

    sum
}
