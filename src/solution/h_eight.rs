use crate::{AdventOfCode, Solution};

pub struct HauntedWasteland {
    day: i32,
}

impl AdventOfCode for HauntedWasteland {
    fn new() -> Self {
        HauntedWasteland { day: 8 }
    }

    fn solve(&self) -> Solution {
        Solution {
            day: self.day,
            part_one: 0,
            part_two: 0,
        }
    }
}
