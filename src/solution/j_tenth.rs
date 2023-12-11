use crate::{AdventOfCode, Solution};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct PipeMaze {
    day: i32,
    maze: Vec<Vec<char>>,
    test_maze: Vec<Vec<char>>,
}

impl AdventOfCode for PipeMaze {
    fn new() -> Self {
        let maze =
            BufReader::new(File::open("src/solution/inputs/input-10").expect("Error opening file"))
                .lines()
                .map(|line| line.unwrap())
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

        let test_maze = vec![
            // x  0    1    2    3    4    // y
            vec!['.', '.', '.', '.', '.'], // 0
            vec!['.', 'S', '-', '7', '.'], // 1
            vec!['.', '|', '.', '|', '.'], // 2
            vec!['.', 'L', '-', 'J', '.'], // 3
            vec!['.', '.', '.', '.', '.'], // 4
        ];
        // .....
        // .S-7.
        // .|.|.
        // .L-J.
        // .....

        PipeMaze {
            day: 10,
            maze,
            test_maze,
        }
    }

    fn solve(&self) -> Solution {
        println!("{:?}", self.find_start());

        Solution {
            day: self.day,
            part_one: 0,
            part_two: 0,
        }
    }
}

impl PipeMaze {
    fn find_start(&self) -> (usize, usize) {
        for (y, row) in self.test_maze.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 'S' {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }
}
