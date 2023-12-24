use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize)]
struct Input {
    seeds: Vec<String>,
    seed_to_soil_map: Vec<String>,
    soil_to_fertilizer_map: Vec<String>,
    fertilizer_to_water_map: Vec<String>,
    water_to_light_map: Vec<String>,
    light_to_temperature_map: Vec<String>,
    temperature_to_humidity_map: Vec<String>,
    humidity_to_location_map: Vec<String>,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>
}

impl Map {
    fn from(map_input: Vec<String>) -> Map {
        let ranges: Vec<Range> = map_input
            .iter()
            .map(|input| Range::from(input))
            .collect();

        Map {
            ranges
        }
    }
}

#[derive(Debug)]
struct Range {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl Range {
    fn from(range_input: &String) -> Range {
        let values: Vec<i64> = range_input
            .split(' ')
            .map(|val| match val.parse::<i64>() {
                Ok(int) => int,
                Err(err) => {
                    println!("Error converting {} to integer", val);
                    panic!("Error! {}", err);
                }
            })
            .collect();
        Range {
            destination_start: *values.get(0).unwrap(),
            source_start: *values.get(1).unwrap(),
            range: *values.get(2).unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("./input.json");
    let result = part_1(input);
    println!("Part 1: {}", result);
    let input = include_str!("./input.json");
    let result = part_2(input);
    println!("Part 2: {}", result);
}


fn part_1(input: &str) -> i64 {
    let input: Input = serde_json::from_str(input).unwrap();

    fn convert_source_using_range(source_number: i64, map: &Map) -> i64 {
        for range in &map.ranges {
            let source_range = range.source_start..(range.source_start + range.range);
    
            if source_range.contains(&source_number) {
                let index_offset = source_number - range.source_start;
                return range.destination_start + index_offset
            }
        }
        source_number
    }

    let mut seedResults: Vec<i64> = vec![];
    for seed in input.seeds {
        let seed = seed.parse::<i64>().unwrap();
        let value = convert_source_using_range(seed, &Map::from(input.seed_to_soil_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.soil_to_fertilizer_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.fertilizer_to_water_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.water_to_light_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.light_to_temperature_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.temperature_to_humidity_map.clone()));
        let value = convert_source_using_range(value, &Map::from(input.humidity_to_location_map.clone()));
        println!("Seed {}'s final location is {}", seed, value);
        seedResults.push(value);
    }

    let lowest_location_value = *seedResults.iter().min().unwrap();
    println!("Lowest seed location value: {}", &lowest_location_value);
    lowest_location_value
}

fn part_2(input: &str) -> i64 {
    1
}


#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = include_str!("./part_1_test_input.json");
        let result = part_1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("./part_2_test_input.txt");
        let result = part_2(input);
        assert_eq!(result, 1);
    }
}