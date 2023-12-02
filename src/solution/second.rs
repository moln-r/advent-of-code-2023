use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub struct CubeConundrum {
    // The Elf would first like to know which games would have been possible
    // if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    color_limit: HashMap<String, i32>,
}

impl CubeConundrum {
    pub(crate) fn new() -> CubeConundrum {
        CubeConundrum {
            color_limit: HashMap::from([
                ("red".to_string(), 12),
                ("green".to_string(), 13),
                ("blue".to_string(), 14),
            ])
        }
    }
}

impl CubeConundrum {
    pub(crate) fn solve(&self) -> (i32, i32) {
        let file = File::open("src/solution/input-02")
            .expect("Error opening file");

        let game_num_regex = Regex::new("Game ([0-9]+):").unwrap();
        let per_color_regex = Regex::new("([0-9]+ [a-z]+)").unwrap();

        let mut sum = 0;
        let mut product_sum = 0;
        for line in BufReader::new(file).lines() {
            if line.is_err() {
                println!("Error reading a line");
            } else {
                let line = line.unwrap();
                // println!("Line: {}", line);

                // parsing game number
                let game_num = game_num_regex.captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                // println!("Game number: {}", game_num);

                // parsing color info per game
                let mut any_impossible = false;

                let mut max_count_per_color = HashMap::from([
                    ("red".to_string(), 0),
                    ("green".to_string(), 0),
                    ("blue".to_string(), 0),
                ]);

                for color_info in per_color_regex.captures_iter(&line) {
                    let color_info = color_info.get(0).unwrap().as_str();
                    let split = color_info.split_whitespace().collect::<Vec<&str>>();
                    let count = split[0].parse::<i32>().unwrap();
                    let color = split[1];

                    let is_color_possible = count <= *self.color_limit.get(color).expect("Color not found");
                    // println!("Color [{}] is possible: {}", color_info, is_color_possible);
                    if !is_color_possible {
                        any_impossible = true;
                    }

                    // get max count for color and replace it if current count is bigger
                    let current_max = max_count_per_color.get(color).unwrap();
                    if current_max < &count {
                        max_count_per_color.insert(color.to_string(), count);
                    }
                }

                // println!("Max count per color: {:?}", max_count_per_color);
                product_sum += max_count_per_color.values().product::<i32>();

                if !any_impossible {
                    sum += game_num;
                }
            }
        }
        (sum, product_sum)
    }
}
