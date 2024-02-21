use std::fs::File;
use std::io::{BufRead, BufReader};

struct CalibrationNumber {
    first_digit: Option<char>,
    second_digit: Option<char>,
}

impl Into<usize> for CalibrationNumber {
    fn into(self) -> usize {
        let number: usize = match (self.first_digit, self.second_digit) {
            (Some(first_digit), Some(second_digit)) => {
                let concatenated_number: String = format!("{}{}", first_digit, second_digit);
                concatenated_number.parse::<usize>().unwrap()
            }
            (Some(first_digit), None) => {
                let concatenated_number: String = format!("{}{}", first_digit, first_digit);
                concatenated_number.parse::<usize>().unwrap()
            }
            _ => 0,
        };

        number
    }
}

fn get_calibration_value_from_line(line: &str) -> usize {
    let mut calibration_number: CalibrationNumber = CalibrationNumber {
        first_digit: None,
        second_digit: None,
    };

    for char in line.chars() {
        if char.is_digit(10) {
            if calibration_number.first_digit.is_none() {
                calibration_number.first_digit = Some(char);
            } else {
                calibration_number.second_digit = Some(char);
            }
        }
    }

    let calibration_number: usize = calibration_number.into();

    println!("Line: {}, Number: {}", line, calibration_number);

    calibration_number
}

fn main() {
    let file_path: &str = "./input/input.txt";

    let file: File = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut calibration_sum: usize = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => calibration_sum += get_calibration_value_from_line(&line),
            Err(err) => println!("Error line: {}", err),
        }
    }

    println!("Calibration sum: {}", calibration_sum)
}
