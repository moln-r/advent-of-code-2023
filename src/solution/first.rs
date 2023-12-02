#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Trebuchet {
    nums: Vec<Num>,
}

impl Trebuchet {
    pub fn new() -> Trebuchet {
        Trebuchet {
            nums: Num::init(),
        }
    }

    pub(crate) fn solve(&self) -> i32 {
        println!("Solving first days problem");

        let file = File::open("src/solution/input-01")
            .expect("Error opening file");

        let mut sum = 0;
        for line in BufReader::new(file).lines() {
            if line.is_err() {
                println!("Error reading a line");
            } else {
                let line = line.unwrap();
                // println!("Line: {}", line);

                let first_number = self.find_first_number(&line);
                let last_number = self.find_last_number(&line);

                if first_number.is_none() || last_number.is_none() {
                    panic!("Error finding a number in line: {}", line);
                }

                let mut num_as_string: String = String::new();
                // building the number string
                num_as_string.push(first_number.unwrap());
                num_as_string.push(last_number.unwrap());

                // converting the string to a number and adding it to the sum
                sum += num_as_string.parse::<i32>().unwrap();
            }
        }
        sum
    }

    fn find_first_number(&self, line: &String) -> Option<char> {
        // println!("Finding first number in line: {}", line);
        return self.find_number(line, WhichPart::First);
    }

    fn find_last_number(&self, line: &String) -> Option<char> {
        // println!("Finding last number in line: {}", line);
        let reverse_string: String = line.chars().rev().collect();
        return self.find_number(&reverse_string, WhichPart::Last);
    }

    fn find_number(&self, string: &String, which_part: WhichPart) -> Option<char> {
        for (i, c) in string.chars().enumerate() {
            if c.is_ascii_digit() {
                // println!("It's a digit! {}", c);
                return Some(c);
            } else {
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

                // if !possible_nums.is_empty() {
                //     println!("Possible nums for char {}: {:?}", c, possible_nums);
                // }

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