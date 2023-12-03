#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

const DOT: char = '.';

pub struct GearRatios {
    char_matrix: Vec<Vec<char>>,
}

impl AdventOfCode for GearRatios {
    fn new() -> Self {
        GearRatios {
            char_matrix: BufReader::new(
                File::open("src/solution/inputs/input-03").expect("Error opening file"),
            )
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect(),
        }
    }

    fn solve(&self) -> Solution {
        let mut part_one = 0;
        for row in 0..self.char_matrix.len() {
            let mut num_first_index: Option<usize> = None;
            for column in 0..self.char_matrix[row].len() {
                let char = self.char_matrix[row][column];
                // print!("{}, ", char);
                if char.is_ascii_digit() {
                    // println!("digit");
                    // if we find a number we store the first index, if it has not been set
                    if num_first_index.is_none() {
                        num_first_index = Some(column);
                    }

                    // if we reached end of line we have to run check
                    if column == self.char_matrix[row].len() - 1 {
                        part_one +=
                            self.number_value_with_surrounding_symbol(row, num_first_index, column);
                    }
                } else if num_first_index.is_some() {
                    // if we find a non-number but we have a first index set, then we finished the number by using the previous index (which was the last number)
                    // print!("not digit, num ended, ");
                    let num_last_index = column - 1;

                    part_one += self.number_value_with_surrounding_symbol(
                        row,
                        num_first_index,
                        num_last_index,
                    );

                    // clearing out indexes
                    num_first_index = None;
                } else {
                    // println!("not digit");
                    // if we find a non-number and thew first index was not set, we simply go on
                }
            }
        }

        Solution {
            day: 3,
            part_one,
            part_two: 0,
        }
    }
}

impl GearRatios {
    fn number_value_with_surrounding_symbol(
        &self,
        row: usize,
        mut num_first_index: Option<usize>,
        column: usize,
    ) -> i32 {
        if self.has_surrounding_symbols(row, num_first_index.unwrap(), column) {
            // println!("surrounding symbols!");
            let num_as_string: String = self.char_matrix[row][num_first_index.unwrap()..=column]
                .iter()
                .collect();
            // println!("--> Found num: {}", num_as_string);
            num_as_string.parse::<i32>().unwrap()
        } else {
            // println!("no surrounding symbols!");
            0
        }
    }

    fn has_surrounding_symbols(
        &self,
        current_row: usize,
        first_column: usize,
        last_column: usize,
    ) -> bool {
        let true_first_column = if first_column == 0 {
            first_column
        } else {
            first_column - 1
        };
        let true_last_colum = if last_column == self.char_matrix[current_row].len() - 1 {
            last_column
        } else {
            last_column + 1
        };

        // check current row before and after
        let mut any_symbol: bool =
            Self::any_symbol(&self.char_matrix[current_row][true_first_column..=true_last_colum]);

        // check row above
        if !any_symbol && current_row > 0 {
            any_symbol = Self::any_symbol(
                &self.char_matrix[current_row - 1][true_first_column..=true_last_colum],
            );
        }

        // check row below
        if !any_symbol && current_row < self.char_matrix.len() - 1 {
            any_symbol = Self::any_symbol(
                &self.char_matrix[current_row + 1][true_first_column..=true_last_colum],
            );
        }

        any_symbol
    }

    fn any_symbol(chars: &[char]) -> bool {
        return chars
            .iter()
            .any(|char| !char.is_ascii_digit() && char != &DOT);
    }
}
