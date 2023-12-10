#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::{AdventOfCode, Solution};

pub struct CubeConundrum {
    day: i32,
    // We store the color limit for the first part of the problem
    // It cannot be a const because Rust does not (yet) support compile time evaluation of code for HashMap initialization
    color_limit: HashMap<String, i32>,
}

impl AdventOfCode for CubeConundrum {
    fn new() -> Self {
        CubeConundrum {
            day: 2,
            color_limit: HashMap::from([
                ("red".to_string(), 12),
                ("green".to_string(), 13),
                ("blue".to_string(), 14),
            ]),
        }
    }

    fn solve(&self) -> Solution {
        // Open input file for second day
        let file = File::open("src/solution/inputs/input-02").expect("Error opening file");

        // Regex to find the color info in the line
        let per_color_regex = Regex::new("([0-9]+ [a-z]+)").unwrap();
        // HashMap to store the current count for each color
        let count_per_color = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        let mut part_one: i64 = 0;
        let mut part_two = 0;
        // Read file line by line
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            // Parse the game number from the line
            let game_num: i64 = CubeConundrum::get_game_number(&line);

            // Setting up a flag to check if any color had more than the allowed number of cubes for the first part
            let mut any_impossible = false;

            // Clone the count_per_color HashMap to create a mutable version for the current loop
            let mut max_count_per_color = count_per_color.clone();

            // Iterate over the color info in the line
            for color_info in per_color_regex.captures_iter(&line) {
                let color_info = color_info.get(0).unwrap().as_str();
                // Split the color info into count and color by whitespace
                let split = color_info.split_whitespace().collect::<Vec<&str>>();
                let count = split[0].parse::<i32>().unwrap();
                let color = split[1];

                // Check if the current count for the color is less than the limit
                // If not, set the flag, because we'll have to add the game num to the part one sum
                let is_color_possible =
                    count <= *self.color_limit.get(color).expect("Color not found");
                // println!("Color [{}] is possible: {}", color_info, is_color_possible);
                if !is_color_possible && !any_impossible {
                    any_impossible = true;
                }

                // Replace the current max count for the color with the current count if it is bigger
                let current_max = max_count_per_color.get(color).unwrap();
                if current_max < &count {
                    max_count_per_color.insert(color.to_string(), count);
                }
            }
            // Calculate the product of the max counts for each color and add it to the part two sum
            part_two += max_count_per_color.values().product::<i32>() as i64;
            // If there was any color with more than the allowed number of cubes, add the game num to the part one sum
            if !any_impossible {
                part_one += game_num;
            }
        }
        Solution {
            day: self.day,
            part_one,
            part_two,
        }
    }
}

impl CubeConundrum {
    fn get_game_number(line: &String) -> i64 {
        Regex::new("Game ([0-9]+):")
            .unwrap() // Regex to find the game number in the line
            .captures(&line)
            .unwrap()
            .get(1) // Get the first capture group
            .unwrap()
            .as_str()
            .parse::<i64>() // Parse the capture group to i32
            .unwrap()
    }
}
