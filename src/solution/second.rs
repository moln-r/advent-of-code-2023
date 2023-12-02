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
    pub(crate) fn solve(&self) -> i32 {
        let file = File::open("src/solution/input-02")
            .expect("Error opening file");

        let game_num_regex = Regex::new("Game ([0-9]+):").unwrap();
        let per_color_regex = Regex::new("([0-9]+ [a-z]+)").unwrap();

        let mut sum = 0;
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
                for color_info in per_color_regex.captures_iter(&line) {
                    let color_info = color_info.get(0).unwrap().as_str();
                    let is_color_possible = self.is_color_possible(color_info);
                    // println!("Color [{}] is possible: {}", color_info, is_color_possible);

                    if !is_color_possible {
                        any_impossible = true;
                    }
                }

                if !any_impossible {
                    sum += game_num;
                }
            }
        }
        sum
    }

    fn is_color_possible(&self, count_for_color: &str) -> bool {
        let split = count_for_color.split_whitespace().collect::<Vec<&str>>();
        let count = split[0].parse::<i32>().unwrap();
        let color = split[1];

        let limit = self.color_limit.get(color).expect("Color not found");
        count <= *limit
    }
}
