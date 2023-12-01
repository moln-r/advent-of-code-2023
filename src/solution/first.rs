use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Trebuchet {}

impl Trebuchet {
    pub(crate) fn solve() -> i32 {
        println!("Solving first days problem");

        let mut sum = 0;

        let file = File::open("src/solution/input-01")
            .expect("Error opening file");
        for line in BufReader::new(file).lines() {
            if line.is_ok() {
                let line = line.unwrap();

                let mut num_as_string: String = String::new();

                for c in line.chars() {
                    if c.is_ascii_digit() {
                        num_as_string.push(c);
                        break;
                    }
                }
                for c in line.chars().rev() {
                    if c.is_ascii_digit() {
                        num_as_string.push(c);
                        break;
                    }
                }
                sum += num_as_string.parse::<i32>().unwrap();
            } else {
                println!("Error reading a line");
            }
        }

        sum
    }

}