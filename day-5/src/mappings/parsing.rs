use crate::mappings::Mappings;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn parse_tuple(line: &str) -> (usize, usize, usize) {
    let parts: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}

pub fn parse_file_to_mappings(file: File) -> Mappings {
    let mut mappings: Mappings = Mappings {
        seeds: Vec::new(),
        seed_to_soil_map: Vec::new(),
        soil_to_fertilizer_map: Vec::new(),
        fertilizer_to_water_map: Vec::new(),
        water_to_light_map: Vec::new(),
        light_to_temperature_map: Vec::new(),
        temperature_to_humidity_map: Vec::new(),
        humidity_to_location_map: Vec::new(),
    };

    let reader = BufReader::new(file);

    let mut current_section = String::new();

    for line in reader.lines() {
        let line = line.expect("Failed to parse the file line");

        if line.is_empty() {
            continue;
        }

        let line_parts = line.split_once(':');

        match line_parts {
            Some(section) => {
                current_section = line_parts.unwrap().0.to_string();

                if current_section.as_str() == "seeds" {
                    mappings.seeds = section
                        .1
                        .split_whitespace()
                        .map(|s| usize::from_str(s).unwrap())
                        .collect();
                }
            }
            None => {
                match current_section.as_str() {
                    "seed-to-soil map" => mappings.seed_to_soil_map.push(parse_tuple(&line)),
                    "soil-to-fertilizer map" => {
                        mappings.soil_to_fertilizer_map.push(parse_tuple(&line))
                    }
                    "fertilizer-to-water map" => {
                        mappings.fertilizer_to_water_map.push(parse_tuple(&line))
                    }
                    "water-to-light map" => mappings.water_to_light_map.push(parse_tuple(&line)),
                    "light-to-temperature map" => {
                        mappings.light_to_temperature_map.push(parse_tuple(&line))
                    }
                    "temperature-to-humidity map" => mappings
                        .temperature_to_humidity_map
                        .push(parse_tuple(&line)),
                    "humidity-to-location map" => {
                        mappings.humidity_to_location_map.push(parse_tuple(&line))
                    }
                    _ => continue,
                };
            }
        }
    }

    mappings
}
