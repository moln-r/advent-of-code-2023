use crate::{AdventOfCode, Solution};

pub struct PipeMaze {
    day: i32,
}

impl AdventOfCode for PipeMaze {
    fn new() -> Self {
        PipeMaze { day: 10 }
    }

    fn solve(&self) -> Solution {
        Solution {
            day: self.day,
            part_one: 0,
            part_two: 0,
        }
    }
}
