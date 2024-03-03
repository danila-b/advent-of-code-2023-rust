use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct ScratchCard {
    pub id: usize,
    pub winning_numbers: HashSet<usize>,
    pub card_numbers: Vec<usize>,
}

impl ScratchCard {
    pub fn find_winning_numbers(&self) -> Vec<usize> {
        let mut winning_numbers: Vec<usize> = vec![];

        for number in &self.card_numbers {
            if self.winning_numbers.contains(number) {
                winning_numbers.push(*number);
            }
        }

        winning_numbers
    }

    pub fn calculate_points(&self) -> usize {
        let number_of_winning_numbers = self.find_winning_numbers().len();

        match number_of_winning_numbers {
            0 => 0,
            number => 2usize.pow(number as u32 - 1),
        }
    }
}

#[derive(Debug)]
pub struct ScratchCardParsingError;

impl FromStr for ScratchCard {
    type Err = ScratchCardParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(ScratchCardParsingError);
        }

        let id = parts[0]
            .trim_start_matches("Card ")
            .trim()
            .parse::<usize>()
            .unwrap();

        let numbers_parts: Vec<&str> = parts[1].split("|").collect();

        if numbers_parts.len() != 2 {
            return Err(ScratchCardParsingError);
        }

        let winning_numbers: HashSet<usize> = numbers_parts[0]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let card_numbers: Vec<usize> = numbers_parts[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        Ok(ScratchCard {
            id,
            winning_numbers,
            card_numbers,
        })
    }
}
