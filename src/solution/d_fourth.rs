#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

pub struct Scratchcards {
    day: i32,
}

impl AdventOfCode for Scratchcards {
    fn new() -> Self {
        Scratchcards { day: 4 }
    }

    fn solve(&self) -> Solution {
        // Open input file for second day
        let file = File::open("src/solution/inputs/input-04").expect("Error opening file");

        // HashMap containing the game number and number of game copies it has
        // We use this to calculate the number of scratchcards for part two
        let mut copy_count: HashMap<i32, i64> = HashMap::new();

        let mut part_one: i64 = 0;
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            // Parse the game number from the line
            let game_num = line[5..8].trim().parse::<i32>().unwrap();
            // println!("Game {}", game_num);

            // Increase the game number count
            let count = copy_count.entry(game_num).or_insert(0);
            *count += 1;
            // println!("Copy count increased by 1 for Card {:?} to {:?}", game_num, count);

            // Create a split of the line to get the winning and scratchcard numbers
            let split = line[10..].split('|').collect::<Vec<&str>>();
            // println!("{:?}", split);

            // Parse the winning numbers into Vec of i32
            let winning_numbers = split[0]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            // println!("Winning numbers: {:?}", winning_numbers);

            // Parse the scratchcard numbers into Vec of i32
            let scratchcard_numbers = split[1]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            // println!("Scratchcard numbers: {:?}", scratchcard_numbers);

            // Get the matching numbers between the winning and scratchcard numbers
            let matching_numbers = scratchcard_numbers
                .iter()
                .filter(|x| winning_numbers.contains(x))
                .collect::<Vec<&i32>>();
            // println!("Matching numbers: {:?}", matching_numbers);

            // Get the count of winning numbers
            let winning_numbers_count = matching_numbers.len() as i32;
            // println!("{:?}", winning_numbers_count);

            // Increase the count for each game number after the current game number
            let num_of_copies = copy_count.get(&game_num).unwrap().clone();
            for num in (game_num + 1)..=(game_num + winning_numbers_count) {
                let count = copy_count.entry(num).or_insert(0);
                *count += num_of_copies;
                // println!("Copy count increased by 1 for Card {:?} to {:?}", num, count);
            }

            // Calculate the points with power of 2 for part one
            if winning_numbers_count > 0 {
                part_one += 2_i32.pow((winning_numbers_count - 1) as u32) as i64;
            }
        }
        // println!("Copy count: {:?}", copy_count);

        Solution {
            day: self.day,
            part_one,
            part_two: copy_count.values().sum(),
        }
    }
}
