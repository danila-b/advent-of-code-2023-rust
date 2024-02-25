use std::str::FromStr;

/// CubeThrow represent one roll of a dice within a game, with cube colour and rolled number
pub enum CubeThrow {
    Blue(usize),
    Red(usize),
    Green(usize),
}

impl CubeThrow {
    const BLUE_LIMIT: usize = 14;
    const RED_LIMIT: usize = 12;
    const GREEN_LIMIT: usize = 13;

    pub fn check_cube_throw(&self) -> bool {
        match *self {
            CubeThrow::Blue(number) => number <= CubeThrow::BLUE_LIMIT,
            CubeThrow::Red(number) => number <= CubeThrow::RED_LIMIT,
            CubeThrow::Green(number) => number <= CubeThrow::GREEN_LIMIT,
            _ => panic!("Couldn't find enum"),
        }
    }
}

#[derive(Debug)]

pub struct CubeThrowParsingError;

impl FromStr for CubeThrow {
    type Err = CubeThrowParsingError;

    fn from_str(input: &str) -> Result<CubeThrow, Self::Err> {
        let parts: Vec<&str> = input.trim().split(" ").collect();

        match parts.len() {
            2 => {
                let number = parts[0]
                    .parse()
                    .expect("Expected a number as cube throw part.");
                let colour = parts[1];

                match colour {
                    "blue" => Ok(CubeThrow::Blue(number)),
                    "red" => Ok(CubeThrow::Red(number)),
                    "green" => Ok(CubeThrow::Green(number)),
                    _ => Err(CubeThrowParsingError),
                }
            }
            _ => {
                println!("Fail to parse cube throw '{}'.", input);
                Err(CubeThrowParsingError)
            }
        }
    }
}
