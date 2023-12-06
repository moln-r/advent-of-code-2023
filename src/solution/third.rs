#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

// Relevant chars in input file
const DOT: char = '.';
const STAR: char = '*';

// We map the input file into a two dimensional char matrix to make it easier to work with
pub struct GearRatios {
    day: i32,
    char_matrix: Vec<Vec<char>>,
}

// Struct to store numbers from the matrix with their position and value
struct NumberInMatrix {
    row: usize,
    first_column: usize,
    last_column: usize,
    value: i32,
}

impl NumberInMatrix {
    // Constructor, using the char_matrix
    fn new(
        char_matrix: &Vec<Vec<char>>,
        row: usize,
        first_column: usize,
        last_column: usize,
    ) -> Self {
        let number_in_matrix: String = char_matrix[row][first_column..=last_column]
            .iter()
            .collect();
        NumberInMatrix {
            row,
            first_column,
            last_column,
            value: number_in_matrix.parse::<i32>().unwrap(),
        }
    }
}

// For the second part we need to store the product of characters around a gear (star)
struct GearRatio {
    product: i32,
    num_of_gears: i64,
}

impl GearRatio {
    fn new() -> Self {
        GearRatio {
            product: 1,
            num_of_gears: 0,
        }
    }

    fn add(&mut self, value: i32) {
        self.product *= value;
        self.num_of_gears += 1;
    }
}

impl AdventOfCode for GearRatios {
    fn new() -> Self {
        // Parse input file into char matrix
        GearRatios {
            day: 3,
            char_matrix: BufReader::new(
                File::open("src/solution/inputs/input-03").expect("Error opening file"),
            )
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect(),
        }
    }

    fn solve(&self) -> Solution {
        let numbers_in_matrix: Vec<NumberInMatrix> = self.map_char_matrix_into_numbers();
        let part_one = self.sum_with_surrounding_symbol(&numbers_in_matrix) as i64;
        let part_two = self.sum_gear_ratios(&numbers_in_matrix) as i64;

        Solution {
            day: self.day,
            part_one,
            part_two,
        }
    }
}

impl GearRatios {
    // Collect all numbers in the matrix and store them in a Vec as NumberInMatrix
    fn map_char_matrix_into_numbers(&self) -> Vec<NumberInMatrix> {
        let mut numbers_in_matrix: Vec<NumberInMatrix> = Vec::new();
        for row in 0..self.char_matrix.len() {
            let mut num_first_index: Option<usize> = None;
            for column in 0..self.char_matrix[row].len() {
                if self.char_matrix[row][column].is_ascii_digit() {
                    // If we find a number, we store the first index, if it has not been set already
                    if num_first_index.is_none() {
                        num_first_index = Some(column);
                    }

                    // If the last character in the row is a number, we store the number
                    if column == self.char_matrix[row].len() - 1 {
                        numbers_in_matrix.push(NumberInMatrix::new(
                            &self.char_matrix,
                            row,
                            num_first_index.unwrap(),
                            column,
                        ));
                    }
                } else if num_first_index.is_some() {
                    // If we find a non-number and we have a first index set, then we reached the end of a number
                    numbers_in_matrix.push(NumberInMatrix::new(
                        &self.char_matrix,
                        row,
                        num_first_index.unwrap(),
                        column - 1,
                    ));

                    // clearing out index of first number
                    num_first_index = None;
                }
            }
        }
        numbers_in_matrix
    }

    // Checks if there is a symbol in the matrix in the given row and between the given columns
    fn has_symbol(
        &self,
        row: usize,
        first_column: usize,
        last_column: usize,
        symbol: Option<char>,
    ) -> (bool, usize, usize) {
        let symbol_filter = if symbol.is_none() {
            // For the first part we look for any symbol that is not a digit or a dot
            |char: &char| !char.is_ascii_digit() && char != &DOT
        } else {
            // For the second part we look for a star
            |char: &char| char == &STAR
        };

        for column in first_column..=last_column {
            let c: char = self.char_matrix[row][column];
            if symbol_filter(&c) {
                return (true, row, column);
            }
        }

        // If we didn't find a symbol, we return (0, 0) coordinates
        (false, 0, 0)
    }

    // Function to check if there is a symbol around a number in the matrix
    fn has_symbol_around(
        &self,
        number_in_matrix: &NumberInMatrix,
        symbol: Option<char>,
    ) -> (bool, usize, usize) {
        // We check the column before and after the number, but we need to make sure we don't go out of bounds
        let true_first_column =
            std::cmp::max(number_in_matrix.first_column as isize - 1, 0) as usize;
        let true_last_colum = std::cmp::min(
            number_in_matrix.last_column + 1,
            self.char_matrix[number_in_matrix.row].len() - 1,
        );

        // Check current row before and after
        let mut any_symbol = self.has_symbol(
            number_in_matrix.row,
            true_first_column,
            true_last_colum,
            symbol,
        );

        // Check row above
        if !any_symbol.0 && number_in_matrix.row > 0 {
            any_symbol = self.has_symbol(
                number_in_matrix.row - 1,
                true_first_column,
                true_last_colum,
                symbol,
            );
        }

        // Check row below
        if !any_symbol.0 && number_in_matrix.row < self.char_matrix.len() - 1 {
            any_symbol = self.has_symbol(
                number_in_matrix.row + 1,
                true_first_column,
                true_last_colum,
                symbol,
            );
        }

        any_symbol
    }

    fn sum_with_surrounding_symbol(&self, numbers_in_matrix: &Vec<NumberInMatrix>) -> i32 {
        // Returns the sum of all numbers that have any symbol around them
        numbers_in_matrix
            .iter()
            .filter(|num| self.has_symbol_around(num, None).0)
            .map(|num| num.value)
            .sum()
    }

    fn sum_gear_ratios(&self, numbers_in_matrix: &Vec<NumberInMatrix>) -> i32 {
        let mut gear_ratios: HashMap<(usize, usize), GearRatio> = HashMap::new();
        // Check if there is a star around each number and store the product of the numbers in the HashMap by the coordinates of the star
        for num in numbers_in_matrix {
            let star_around = self.has_symbol_around(num, Some(STAR));
            if star_around.0 {
                let gear_ratio_for_star_at = gear_ratios.get_mut(&(star_around.1, star_around.2));

                if gear_ratio_for_star_at.is_none() {
                    let mut gear_ratio = GearRatio::new();
                    gear_ratio.add(num.value);
                    gear_ratios.insert((star_around.1, star_around.2), gear_ratio);
                } else {
                    gear_ratio_for_star_at.unwrap().add(num.value);
                }
            }
        }
        // At this point we have a HashMap with all the products of numbers around any stars

        // We filter the HashMap to only keep the ones with two gears around them and sum their products
        gear_ratios
            .iter()
            .filter(|(_, gr)| gr.num_of_gears == 2)
            .map(|(_, gr)| gr.product)
            .sum()
    }
}
