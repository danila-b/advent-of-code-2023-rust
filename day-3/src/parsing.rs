use crate::definitions::part_number::PartNumber;
use crate::definitions::symbol::{Symbol, SymbolType};

pub fn parse_line(line: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let line_len = line.len();

    let mut row_numbers: Vec<PartNumber> = vec![];
    let mut row_symbols: Vec<Symbol> = vec![];
    let mut part_number: Option<PartNumber> = None;

    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            match &mut part_number {
                Some(x) => x.add_value_char(char),
                None => {
                    part_number = Some(PartNumber {
                        value: char.to_string(),
                        start_idx: index,
                        end_idx: 0,
                    })
                }
            }
        } else {
            match part_number {
                Some(mut x) => {
                    x.set_end_idx(index - 1);
                    row_numbers.push(x);
                    part_number = None;
                }
                None => (),
            }

            if char != '.' {
                if char == '*' {
                    row_symbols.push(Symbol {
                        index: index,
                        variant: SymbolType::Gear,
                    });
                } else {
                    row_symbols.push(Symbol {
                        index: index,
                        variant: SymbolType::Usual,
                    });
                }
            }
        }
    }

    // Catching a case if line is ending with number
    match part_number {
        Some(mut x) => {
            x.set_end_idx(line_len - 1);
            row_numbers.push(x);
        }
        None => (),
    }

    (row_numbers, row_symbols)
}
