use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{AdventOfCode, Solution};

pub struct PointOfIncidence {
    day: i32,
    patterns: Vec<Pattern>,
}

#[derive(Clone)]
struct Pattern {
    p: Vec<Vec<char>>,
    column_count: usize,
    row_count: usize,
}

fn flip(c: &char) -> char {
    match c {
        '.' => '#',
        '#' => '.',
        _ => panic!("Unknown character"),
    }
}

#[allow(dead_code)]
fn print_pattern(p: &Vec<Vec<char>>) {
    println!();
    print!("   ");
    for i in 0..p.get(0).unwrap().len() {
        if i >= 9 {
            print!("{}", i + 1 - 10);
        } else {
            print!("{}", i + 1);
        }
    }
    println!();
    for (i, row) in p.iter().enumerate() {
        print!("{}: ", i + 1);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

impl Pattern {
    fn pattern_value(&self) -> (Vec<i64>, Vec<i64>) {
        (
            self.vertical_reflection_column_count(),
            self.horizontal_reflection_row_count(),
        )
    }

    fn pattern_value_without_fix(&self) -> (Option<i64>, Option<i64>) {
        let pattern_value = self.pattern_value();
        (
            if pattern_value.0.is_empty() {
                None
            } else {
                Some(*(pattern_value.0.get(0).unwrap()))
            },
            if pattern_value.1.is_empty() {
                None
            } else {
                Some(*(pattern_value.1.get(0).unwrap()))
            },
        )
    }

    fn pattern_value_with_smudge_fix(&self) -> (Option<i64>, Option<i64>) {
        // print_pattern(&self.p);
        let original = self.pattern_value_without_fix();
        let original_vec: (Vec<i64>, Vec<i64>) = (
            if original.0.is_none() {
                vec![]
            } else {
                vec![original.0.unwrap()]
            },
            if original.1.is_none() {
                vec![]
            } else {
                vec![original.1.unwrap()]
            },
        );
        // println!("original pattern: {:?}", original_vec);

        let mut new_stuff: (Vec<i64>, Vec<i64>) = (vec![], vec![]);
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                let mut alternation = self.p.clone();
                alternation[i][j] = flip(&alternation[i][j]);

                let alternative_pattern = Pattern {
                    p: alternation,
                    column_count: self.column_count,
                    row_count: self.row_count,
                };
                let new: (Vec<i64>, Vec<i64>) = alternative_pattern.pattern_value();

                if new.0.is_empty() && new.1.is_empty() {
                    // ignore this
                } else if new != original_vec {
                    // println!("new pattern found at ({},{}): {:?}", i, j, new);
                    new_stuff = new;
                }
            }
        }

        if !original_vec.0.is_empty() {
            new_stuff.0.retain(|x| !original_vec.0.contains(x))
        }
        if !original_vec.1.is_empty() {
            new_stuff.1.retain(|x| !original_vec.1.contains(x))
        }

        (
            new_stuff.0.first().map(|x| *x),
            new_stuff.1.first().map(|x| *x),
        )
    }

    fn get_column(&self, i: usize) -> Vec<char> {
        self.p.iter().map(|row| row[i]).collect::<Vec<char>>()
    }

    fn get_row(&self, i: usize) -> &Vec<char> {
        &self.p[i]
    }

    // find the columns that starts the reflection and return the left one's index
    fn vertical_reflection_column_count(&self) -> Vec<i64> {
        let mut columns = vec![];
        for i in 0..self.column_count - 1 {
            if self.check_columns(i, i + 1) {
                columns.push((i + 1) as i64);
            }
        }
        // if columns.len() > 1 {
        //     println!("Columns: {:?}", columns);
        // }
        columns
    }

    fn check_columns(&self, i: usize, j: usize) -> bool {
        // println!("Checking columns {} and {}", i, j);
        if self.get_column(i) == self.get_column(j) {
            // println!("Columns {} and {} are equal", i, j);
            return if i == 0 || j == (self.column_count - 1) {
                // println!("--- No more columns to check");
                true
            } else {
                self.check_columns(i - 1, j + 1)
            };
        } else {
            // println!("Columns {} and {} are not equal", i, j);
            false
        }
    }

    // find the rows that starts the reflection and return the top one's index
    fn horizontal_reflection_row_count(&self) -> Vec<i64> {
        let mut rows = vec![];
        for i in 0..self.row_count - 1 {
            if self.check_rows(i, i + 1) {
                rows.push((i + 1) as i64);
            }
        }
        // if rows.len() > 1 {
        //     println!("Rows: {:?}", rows);
        // }
        rows
    }

    fn check_rows(&self, i: usize, j: usize) -> bool {
        if self.get_row(i) == self.get_row(j) {
            // println!("Rows {} and {} are equal", i, j);
            return if i == 0 || j == (self.row_count - 1) {
                // println!("--- No more rows to check");
                true
            } else {
                self.check_rows(i - 1, j + 1)
            };
        } else {
            // println!("Rows {} and {} are not equal", i, j);
            false
        }
    }
}

impl AdventOfCode for PointOfIncidence {
    fn new() -> Self {
        let mut patterns = vec![];

        let content =
            BufReader::new(File::open("src/solution/inputs/input-13").expect("Error opening file"))
                .lines()
                .map(|line| line.unwrap())
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

        let empty_line_indexes: Vec<usize> = content
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_empty())
            .map(|(i, _)| i)
            .collect();

        let mut begin = 0;
        for i in empty_line_indexes {
            let pattern = content[begin..i].to_vec();
            let column_count = pattern.first().unwrap().len();
            let row_count = pattern.len();
            patterns.push(Pattern {
                p: pattern,
                column_count,
                row_count,
            });
            begin = i + 1;
        }

        let pattern = content[begin..content.len()].to_vec();
        let column_count = pattern.first().unwrap().len();
        let row_count = pattern.len();
        patterns.push(Pattern {
            p: pattern,
            column_count,
            row_count,
        });

        PointOfIncidence { day: 13, patterns }
    }

    fn solve(&self) -> Solution {
        let pattern_values: Vec<(Option<i64>, Option<i64>)> = self
            .patterns
            .iter()
            .map(|p| p.pattern_value_without_fix())
            .collect();
        let columns = pattern_values
            .iter()
            .map(|p| p.0)
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .sum::<i64>();
        let rows = pattern_values
            .iter()
            .map(|p| p.1)
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .sum::<i64>();

        let pattern_values_with_smudge_fix: Vec<(Option<i64>, Option<i64>)> = self
            .patterns
            .iter()
            .map(|p| p.pattern_value_with_smudge_fix())
            .collect();

        let new_columns = pattern_values_with_smudge_fix
            .iter()
            .map(|p| p.0)
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .sum::<i64>();
        let new_rows = pattern_values_with_smudge_fix
            .iter()
            .map(|p| p.1)
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .sum::<i64>();

        Solution {
            day: self.day,
            part_one: columns + rows * 100,
            part_two: new_columns + new_rows * 100,
        }
    }
}
