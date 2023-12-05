#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

#[derive(Debug)]
pub struct Trebuchet {
    day: i32,
    nums: Vec<Num>, // We store the numbers from one to nine in a special struct to help us resolve the problem
}

impl AdventOfCode for Trebuchet {
    fn new() -> Self {
        Trebuchet {
            day: 1,
            nums: Num::init(),
        }
    }

    fn solve(&self) -> Solution {
        // Open input file for first day
        let file = File::open("src/solution/inputs/input-01").expect("Error opening file");

        let mut part_one: i64 = 0;
        let mut part_two = 0;
        // Read file line by line
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            // For the first part, find the first and last numbers, looking for digits only
            let first_number = self.find_number(&line, WhichPart::First, true).unwrap();
            let last_number = self
                .find_number(&(line.chars().rev().collect()), WhichPart::Last, true)
                .unwrap();
            // Format the numbers as a string, parse it to i32 and add it to the part one sum
            part_one += format!("{}{}", first_number, last_number)
                .parse::<i64>()
                .unwrap();

            // For the second part, find the first and last numbers, looking for numbers as digit or substring
            let first_number = self.find_number(&line, WhichPart::First, false).unwrap();
            let last_number = self
                .find_number(&(line.chars().rev().collect()), WhichPart::Last, false)
                .unwrap();
            // Format the numbers as a string, parse it to i32 and add it to the part two sum
            part_two += format!("{}{}", first_number, last_number)
                .parse::<i32>()
                .unwrap();
        }
        Solution {
            day: self.day,
            part_one,
            part_two,
        }
    }
}

impl Trebuchet {
    fn find_number(
        &self,
        string: &String,
        which_part: WhichPart,
        only_digit: bool,
    ) -> Option<char> {
        // Iterate over the string's chars
        for (i, c) in string.chars().enumerate() {
            if c.is_ascii_digit() {
                // If char is a digit, return it
                return Some(c);
            } else if !only_digit {
                // If we look for numbers as digit or substring, run extra logic

                // Filter possible numbers by first or last char and by length based on the rest of the string
                let possible_nums: Vec<&Num> = self
                    .nums
                    .iter()
                    .filter(|n| {
                        return match which_part {
                            // Based on which part we are looking for, check if the first or last char matches
                            WhichPart::First => n.first_char == c,
                            WhichPart::Last => n.last_char == c,
                        };
                    })
                    .filter(|n| n.str_length <= string.len() - i)
                    .collect();

                // Iterate over possible numbers and try to find a match
                for num in possible_nums {
                    let string_to_compare: &str = match which_part {
                        // Based on which part we are looking for, get the right string to compare
                        WhichPart::First => num.as_string.as_str(),
                        WhichPart::Last => num.as_rev_string.as_str(),
                    };
                    if *string_to_compare == string[i..i + num.str_length] {
                        return Some(char::from_digit(num.num as u32, 10).unwrap());
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct Num {
    // We store the number, the first and last char, the number as string and as reversed string, and its string length
    num: i32,
    first_char: char,
    last_char: char,
    as_string: String,
    as_rev_string: String,
    str_length: usize,
}

impl Num {
    fn init() -> Vec<Num> {
        // Initialize the numbers from one to nine and create Num structs
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .map(|(i, n)| Num {
            num: i as i32 + 1,
            first_char: n.chars().next().unwrap(),
            last_char: n.chars().last().unwrap(),
            as_string: n.to_string(),
            as_rev_string: n.chars().rev().collect::<String>(),
            str_length: n.len(),
        })
        .collect()
    }
}

enum WhichPart {
    First,
    Last,
}
