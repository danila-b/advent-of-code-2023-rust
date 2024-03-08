use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn parse_line_to_numbers(line: &str) -> Vec<usize> {
        line.split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect()
    }

    fn parse_line_to_long_number(line: &str) -> Vec<usize> {
        let number_string: String = line.chars().filter(|c| c.is_digit(10)).collect();
        vec![number_string.parse::<usize>().unwrap()]
    }

    pub fn parse_rases_from_file(file: &File, to_long_number: bool) -> Result<Vec<Race>> {
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();

        let times_line = lines
            .next()
            .ok_or(Error::new(ErrorKind::Other, "Missing time line"))??;

        let distances_line = lines
            .next()
            .ok_or(Error::new(ErrorKind::Other, "Missing distance line"))??;

        let times = match to_long_number {
            true => Self::parse_line_to_long_number(&times_line),
            false => Self::parse_line_to_numbers(&times_line),
        };
        let distances = match to_long_number {
            true => Self::parse_line_to_long_number(&distances_line),
            false => Self::parse_line_to_numbers(&distances_line),
        };

        Ok(times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect())
    }

    pub fn find_race_wins(&self) -> usize {
        let mut record_charges: usize = 0;

        for seconds_to_charge in 1..self.time / 2 {
            if seconds_to_charge * (self.time - seconds_to_charge) > self.distance {
                record_charges = self.time - seconds_to_charge * 2 + 1;
                break;
            }
        }

        record_charges
    }
}
