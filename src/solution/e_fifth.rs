use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::{AdventOfCode, Solution};

#[derive(Debug)]
pub struct GiveASeedAFertilizer {
    day: i32,
    part_one_seeds: Vec<i64>,
    part_two_seeds: Vec<SeedRange>,
    seed_to_soil: Vec<HowTo>,
    soil_to_fertilizer: Vec<HowTo>,
    fertilizer_to_water: Vec<HowTo>,
    water_to_light: Vec<HowTo>,
    light_to_temperature: Vec<HowTo>,
    temperature_to_humidity: Vec<HowTo>,
    humidity_to_location: Vec<HowTo>,
}

#[derive(Debug)]
struct HowTo {
    destination: i64,
    source: i64,
    range: i64,
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64, // exclusive
}

// seed -> soil -> fertilizer -> water -> light -> temperature -> humidity -> location
const INPUT_TYPES: [&str; 8] = [
    "seeds",
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

impl AdventOfCode for GiveASeedAFertilizer {
    fn new() -> Self {
        let file = File::open("src/solution/inputs/input-05").expect("Error opening file");

        let category_regex = Regex::new("(\\d+) (\\d+) (\\d+)").expect("Invalid regex");

        let mut part_one_seeds = Vec::new();
        let mut part_two_seeds = Vec::new();

        let mut seed_to_soil = Vec::new();
        let mut soil_to_fertilizer = Vec::new();
        let mut fertilizer_to_water = Vec::new();
        let mut water_to_light = Vec::new();
        let mut light_to_temperature = Vec::new();
        let mut temperature_to_humidity = Vec::new();
        let mut humidity_to_location = Vec::new();

        let mut current_type: &str;
        let mut current_vec = &mut seed_to_soil;
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            // Skipping empty lines
            if line.len() == 0 {
                continue;
            }

            // Checking if the first character is a digit or a letter
            if line.chars().next().unwrap().is_ascii_digit() {
                // If it's a digit, we parse the source, destination and range
                for capture in category_regex.captures_iter(&line) {
                    let destination = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
                    let source = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
                    let range = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();

                    // println!("{} {} {}", destination, source, range);

                    current_vec.push(HowTo {
                        destination,
                        source,
                        range,
                    });
                }
            } else {
                // If it's a letter, we parse the type and set up the storage
                current_type = INPUT_TYPES
                    .iter()
                    .filter(|&t| line.starts_with(t))
                    .next()
                    .expect("Invalid input type in file");

                // If the current type is seeds, we read its numbers from the same line
                if current_type == "seeds" {
                    line[line.find(':').unwrap() + 1..]
                        .split_whitespace()
                        .for_each(|seed_number| {
                            part_one_seeds.push(
                                seed_number
                                    .parse::<i64>()
                                    .expect("Invalid seed number in file"),
                            );
                        });

                    let seed_list = line[line.find(':').unwrap() + 1..]
                        .split_whitespace()
                        .map(|s| s.parse::<i64>().expect("Invalid seed number in file"))
                        .collect::<Vec<i64>>();

                    for x in (0..seed_list.len()).step_by(2) {
                        let first = seed_list[x];
                        let second = seed_list[x + 1];

                        part_two_seeds.push(SeedRange {
                            start: first,
                            end: first + second,
                        });
                    }
                } else {
                    // If the current type is not seeds, we set up the storage for the current type in the iteration when we read numbers
                    current_vec = match current_type {
                        "seed-to-soil" => &mut seed_to_soil,
                        "soil-to-fertilizer" => &mut soil_to_fertilizer,
                        "fertilizer-to-water" => &mut fertilizer_to_water,
                        "water-to-light" => &mut water_to_light,
                        "light-to-temperature" => &mut light_to_temperature,
                        "temperature-to-humidity" => &mut temperature_to_humidity,
                        "humidity-to-location" => &mut humidity_to_location,
                        _ => {
                            panic!("Invalid input type in file")
                        }
                    };
                }
            }
        }

        GiveASeedAFertilizer {
            day: 5,
            part_one_seeds,
            part_two_seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn solve(&self) -> Solution {
        Solution {
            day: self.day,
            part_one: self.part_one(),
            part_two: 0,
        }
    }
}

impl GiveASeedAFertilizer {
    pub fn part_one(&self) -> i64 {
        let mut seed_to_location: HashMap<i64, i64> = HashMap::new();

        for seed in &self.part_one_seeds {
            let mut soil = 0;
            for to_soil in &self.seed_to_soil {
                if seed >= &to_soil.source && seed < &(to_soil.source + to_soil.range) {
                    soil = seed - to_soil.source + to_soil.destination;
                    // println!("Seed {} -> Soil {}", seed, soil);
                    break;
                }
            }
            if soil == 0 {
                soil = *seed;
            }

            let mut fertilizer = 0;
            for to_fertilizer in &self.soil_to_fertilizer {
                if soil >= to_fertilizer.source && soil < to_fertilizer.source + to_fertilizer.range
                {
                    fertilizer = soil - to_fertilizer.source + to_fertilizer.destination;
                    // println!("Soil {} -> Fertilizer {}", soil, fertilizer);
                    break;
                }
            }
            if fertilizer == 0 {
                fertilizer = soil
            }

            let mut water = 0;
            for to_water in &self.fertilizer_to_water {
                if fertilizer >= to_water.source && fertilizer < to_water.source + to_water.range {
                    water = fertilizer - to_water.source + to_water.destination;
                    // println!("Fertilizer {} -> Water {}", fertilizer, water);
                    break;
                }
            }
            if water == 0 {
                water = fertilizer;
            }

            let mut light = 0;
            for to_light in &self.water_to_light {
                if water >= to_light.source && water < to_light.source + to_light.range {
                    light = water - to_light.source + to_light.destination;
                    // println!("Water {} -> Light {}", water, light);
                    break;
                }
            }
            if light == 0 {
                light = water;
            }

            let mut temperature = 0;
            for to_temperature in &self.light_to_temperature {
                if light >= to_temperature.source
                    && light < to_temperature.source + to_temperature.range
                {
                    temperature = light - to_temperature.source + to_temperature.destination;
                    // println!("Light {} -> Temperature {}", light, temperature);
                    break;
                }
            }
            if temperature == 0 {
                temperature = light;
            }

            let mut humidity = 0;
            for to_humidity in &self.temperature_to_humidity {
                if temperature >= to_humidity.source
                    && temperature < to_humidity.source + to_humidity.range
                {
                    humidity = temperature - to_humidity.source + to_humidity.destination;
                    // println!("Temperature {} -> Humidity {}", temperature, humidity);
                    break;
                }
            }
            if humidity == 0 {
                humidity = temperature;
            }

            let mut location = 0;
            for to_location in &self.humidity_to_location {
                if humidity >= to_location.source
                    && humidity < to_location.source + to_location.range
                {
                    location = humidity - to_location.source + to_location.destination;
                    // println!("Humidity {} -> Location {}", humidity, location);
                    break;
                }
            }
            if location == 0 {
                location = humidity;
            }
            seed_to_location.insert(*seed, location);
        }
        *seed_to_location.values().min().unwrap()
    }
}
