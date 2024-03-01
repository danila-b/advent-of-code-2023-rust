use crate::cube;
use std::str::FromStr;

#[derive(Debug)]
/// A game represented as game ID and string of cube rolls.
/// Can be improved to store Vec<CubeThrow> instead of plain string.
pub struct Game {
    pub id: usize,
    pub cube_throws: String,
}

fn get_game_id_from_string(game_str: &str) -> usize {
    let digits: String = game_str.chars().filter(|c| c.is_digit(10)).collect();

    if digits.is_empty() {
        panic!("Can't find game number ID in the string.")
    } else {
        digits.parse::<usize>().ok().unwrap()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let parts: Vec<&str> = input.split(":").collect();

        match parts.len() {
            2 => Ok(Game {
                id: get_game_id_from_string(parts[0]),
                cube_throws: parts[1].to_string(),
            }),
            _ => {
                println!("Couldn't parse string to games because of incorrect colon delimeters.");
                Err(())
            }
        }
    }
}

impl Game {
    pub fn check_game(&self) -> bool {
        for cube_throw in self.cube_throws.split([',', ';']) {
            let cube_throw =
                cube::CubeThrow::from_str(cube_throw).expect("Cube throw is not valid.");

            if !cube_throw.check_cube_throw() {
                return false;
            }
        }

        true
    }

    pub fn find_minimum_cubes_power(&self) -> usize {
        let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);

        for cube_throw in self.cube_throws.split([',', ';']) {
            let cube_throw =
                cube::CubeThrow::from_str(cube_throw).expect("Cube throw is not valid.");

            match cube_throw {
                cube::CubeThrow::Red(number) => {
                    if min_red < number {
                        min_red = number;
                    };
                }
                cube::CubeThrow::Blue(number) => {
                    if min_blue < number {
                        min_blue = number;
                    };
                }
                cube::CubeThrow::Green(number) => {
                    if min_green < number {
                        min_green = number;
                    };
                }
            }
        }

        min_red * min_blue * min_green
    }
}
