pub mod parsing;

#[derive(Debug)]
pub struct Mappings {
    pub seeds: Vec<usize>,
    seed_to_soil_map: Vec<(usize, usize, usize)>,
    soil_to_fertilizer_map: Vec<(usize, usize, usize)>,
    fertilizer_to_water_map: Vec<(usize, usize, usize)>,
    water_to_light_map: Vec<(usize, usize, usize)>,
    light_to_temperature_map: Vec<(usize, usize, usize)>,
    temperature_to_humidity_map: Vec<(usize, usize, usize)>,
    humidity_to_location_map: Vec<(usize, usize, usize)>,
}

impl Mappings {
    fn connect_value_to_mapping(value: usize, mapping: &Vec<(usize, usize, usize)>) -> usize {
        for (dest_range_start, source_range_start, range_lenght) in mapping {
            if value >= *source_range_start && value < source_range_start + range_lenght {
                return dest_range_start + (value - source_range_start);
            }
        }

        value
    }

    pub fn find_seed_location(&self, seed: usize) -> usize {
        let soil = Self::connect_value_to_mapping(seed, &self.seed_to_soil_map);
        let fertilizer = Self::connect_value_to_mapping(soil, &self.soil_to_fertilizer_map);
        let water = Self::connect_value_to_mapping(fertilizer, &self.fertilizer_to_water_map);
        let light = Self::connect_value_to_mapping(water, &self.water_to_light_map);
        let temperature = Self::connect_value_to_mapping(light, &self.light_to_temperature_map);
        let humidity =
            Self::connect_value_to_mapping(temperature, &self.temperature_to_humidity_map);
        let location = Self::connect_value_to_mapping(humidity, &self.humidity_to_location_map);

        location
    }
}
