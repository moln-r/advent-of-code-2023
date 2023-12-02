use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::{AdventOfCode, Solution};

pub struct CubeConundrum {
    day: i32,
    color_limit: HashMap<String, i32>,
}

impl AdventOfCode for CubeConundrum {
    fn solve(&self) -> Solution {
        let file = File::open("src/solution/inputs/input-02")
            .expect("Error opening file");

        let per_color_regex = Regex::new("([0-9]+ [a-z]+)").unwrap();
        let count_per_color = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        let mut part_one = 0;
        let mut part_two = 0;
        for line in BufReader::new(file).lines() {
            if line.is_err() {
                println!("Error reading a line");
            } else {
                let line = line.unwrap();

                let game_num = Self::get_game_number(&line);
                // println!("Game number: {}", game_num);

                let mut any_impossible = false;
                let mut max_count_per_color = count_per_color.clone();

                for color_info in per_color_regex.captures_iter(&line) {
                    let color_info = color_info.get(0).unwrap().as_str();
                    let split = color_info.split_whitespace().collect::<Vec<&str>>();
                    let count = split[0].parse::<i32>().unwrap();
                    let color = split[1];

                    let is_color_possible = count <= *self.color_limit.get(color).expect("Color not found");
                    // println!("Color [{}] is possible: {}", color_info, is_color_possible);

                    if !is_color_possible && !any_impossible {
                        any_impossible = true;
                    }

                    // get max count for color and replace it if current count is bigger
                    let current_max = max_count_per_color.get(color).unwrap();
                    if current_max < &count {
                        max_count_per_color.insert(color.to_string(), count);
                    }
                }

                // println!("Max count per color: {:?}", max_count_per_color);
                part_two += max_count_per_color.values().product::<i32>();

                if !any_impossible {
                    part_one += game_num;
                }
            }
        }
        Solution { day: self.day, part_one, part_two }
    }
}

impl CubeConundrum {
    pub fn new() -> CubeConundrum {
        CubeConundrum {
            day: 2,
            color_limit: HashMap::from([
                ("red".to_string(), 12),
                ("green".to_string(), 13),
                ("blue".to_string(), 14),
            ]),
        }
    }

    fn get_game_number(line: &String) -> i32 {
        Regex::new("Game ([0-9]+):")
            .unwrap()
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
    }
}
