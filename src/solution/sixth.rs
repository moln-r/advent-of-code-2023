use crate::{AdventOfCode, Solution};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct WaitForIt {
    day: i32,
    times: [i32; 4],
    distances: [i32; 4],
}

impl AdventOfCode for WaitForIt {
    fn new() -> Self {
        let file = File::open("src/solution/inputs/input-06").expect("Error opening file");
        let mut lines = BufReader::new(file).lines();

        let time_line = lines
            .next()
            .expect("No time line")
            .expect("Error at time line");
        let times = parse_line(&time_line);

        let distance_line = lines
            .next()
            .expect("No destination line")
            .expect("Error at destination line");
        let distances = parse_line(&distance_line);

        WaitForIt {
            day: 6,
            times,
            distances,
        }
    }

    fn solve(&self) -> Solution {
        let mut part_one = 1;

        for i in 0..4 {
            let distance = self.distances[i];
            let current_race_time = self.times[i];

            let mut win_count = 0;
            for acceleration_time in 1..current_race_time {
                let possible_distance = acceleration_time * (current_race_time - acceleration_time);
                if possible_distance > distance {
                    win_count += 1;
                }
            }
            part_one *= win_count;
        }

        Solution {
            day: self.day,
            part_one: part_one as i64,
            part_two: 0,
        }
    }
}

fn parse_line(line: &String) -> [i32; 4] {
    let mut parsed = [0; 4];
    for (i, value) in line[line.find(':').unwrap() + 1..]
        .split_whitespace()
        .enumerate()
    {
        parsed[i] = value.parse().expect("Error parsing");
    }
    parsed
}
