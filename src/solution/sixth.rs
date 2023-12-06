use crate::{AdventOfCode, Solution};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct WaitForIt {
    day: i32,
    times: [i64; 4],
    distances: [i64; 4],
    fixed_time: i64,
    fixed_distance: i64,
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
        let fixed_time = parse_fixed_line(&time_line);

        let distance_line = lines
            .next()
            .expect("No destination line")
            .expect("Error at destination line");
        let distances = parse_line(&distance_line);
        let fixed_distance = parse_fixed_line(&distance_line);

        WaitForIt {
            day: 6,
            times,
            distances,
            fixed_time,
            fixed_distance,
        }
    }

    fn solve(&self) -> Solution {
        let mut part_one = 1;
        for i in 0..4 {
            part_one *= get_win_count(self.times[i], self.distances[i]);
        }

        let part_two = get_win_count(self.fixed_time, self.fixed_distance);
        // let part_two = 1;

        Solution {
            day: self.day,
            part_one: part_one as i64,
            part_two,
        }
    }
}

fn parse_line(line: &String) -> [i64; 4] {
    let mut parsed = [0; 4];
    for (i, value) in line[line.find(':').unwrap() + 1..]
        .split_whitespace()
        .enumerate()
    {
        parsed[i] = value.parse().expect("Error parsing");
    }
    parsed
}

fn parse_fixed_line(line: &String) -> i64 {
    line[line.find(':').unwrap() + 1..]
        .replace(' ', "")
        .parse()
        .expect("Error parsing fixed time")
}

fn get_win_count(time: i64, distance: i64) -> i64 {
    println!("time: {}, distance: {}", time, distance);
    let half_time = time / 2;
    println!("half_time: {}", half_time);

    let half_win_fixer = if time % 2 == 0 { 0.5 } else { 0.0 };
    println!("half_win_fixer: {}", half_win_fixer);

    let half_wins = (1..=half_time)
        .filter(|&acceleration_time| acceleration_time * (time - acceleration_time) > distance)
        .count() as f64
        - half_win_fixer;
    println!("half_wins: {}", half_wins);
    let wins = (1..time)
        .filter(|&acceleration_time| acceleration_time * (time - acceleration_time) > distance)
        .count() as i64;
    println!("wins: {}\n", wins);
    (half_wins * 2.0) as i64
}
