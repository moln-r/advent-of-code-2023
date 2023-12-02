#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

#[derive(Debug)]
pub struct Trebuchet {
    day: i32,
    nums: Vec<Num>,
}

impl AdventOfCode for Trebuchet {
    fn solve(&self) -> Solution {
        let file = File::open("src/solution/inputs/input-01")
            .expect("Error opening file");

        let mut part_one = 0;
        let mut part_two = 0;
        for line in BufReader::new(file).lines() {
            if line.is_err() {
                println!("Error reading a line");
            } else {
                let line = line.unwrap();

                let first_number = self.find_number(&line, WhichPart::First, true).unwrap();
                let last_number = self.find_number(&(line.chars().rev().collect()), WhichPart::Last, true).unwrap();

                part_one += format!("{}{}", first_number, last_number).parse::<i32>().unwrap();

                let first_number = self.find_number(&line, WhichPart::First, false).unwrap();
                let last_number = self.find_number(&(line.chars().rev().collect()), WhichPart::Last, false).unwrap();

                part_two += format!("{}{}", first_number, last_number).parse::<i32>().unwrap();
            }
        }
        Solution { day: self.day, part_one, part_two }
    }
}

impl Trebuchet {
    pub fn new() -> Trebuchet {
        Trebuchet {
            day: 1,
            nums: Num::init(),
        }
    }

    fn find_number(&self, string: &String, which_part: WhichPart, only_digit: bool) -> Option<char> {
        for (i, c) in string.chars().enumerate() {
            if c.is_ascii_digit() {
                // println!("It's a digit! {}", c);
                return Some(c);
            } else if !only_digit {
                // get nums that start with char based on length left
                let possible_nums: Vec<&Num> = self.nums.iter()
                    .filter(|n| {
                        return match which_part {
                            WhichPart::First => { n.first_char == c }
                            WhichPart::Last => { n.last_char == c }
                        };
                    })
                    .filter(|n| n.str_length <= string.len() - i)
                    .collect();

                // compare num string with the line's slice
                for num in possible_nums {
                    let string_to_compare: &str = match which_part {
                        WhichPart::First => { num.as_string.as_str() }
                        WhichPart::Last => { num.as_rev_string.as_str() }
                    };
                    if *string_to_compare == string[i..i + num.str_length] {
                        // println!("Found a match at index {}: {}", i, num.num);
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
    num: i32,
    first_char: char,
    last_char: char,
    as_string: String,
    as_rev_string: String,
    str_length: usize,
}

impl Num {
    fn init() -> Vec<Num> {
        let mut nums: Vec<Num> = Vec::new();
        let str_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for i in 0..str_numbers.len() {
            nums.push(Num {
                num: i as i32 + 1,
                first_char: str_numbers[i].chars().next().unwrap(),
                last_char: str_numbers[i].chars().last().unwrap(),
                as_string: str_numbers[i].to_string(),
                as_rev_string: str_numbers[i].chars().rev().collect::<String>(),
                str_length: str_numbers[i].len(),
            });
        }

        nums
    }
}

enum WhichPart { First, Last }