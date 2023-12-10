use crate::solution::h_eight::HauntedWasteland;

pub mod solution;

#[derive(Debug)]
pub struct Solution {
    pub day: i32,
    pub part_one: i64,
    pub part_two: i64,
}

trait AdventOfCode {
    fn new() -> Self;
    fn solve(&self) -> Solution;
}

fn main() {
    println!("Running Advent of code 2023...");
    // Solution { day: 1, part_one: 53651, part_two: 53894 }
    // println!("{:?}", Trebuchet::new().solve());
    // Solution { day: 2, part_one: 2207, part_two: 62241 }
    // println!("{:?}", CubeConundrum::new().solve());
    // Solution { day: 3, part_one: 533775, part_two: 78236071 }
    // println!("{:?}", GearRatios::new().solve());
    // Solution { day: 4, part_one: 24733, part_two: 5422730 }
    // println!("{:?}", Scratchcards::new().solve());
    // Solution { day: 5, part_one: 26273516, part_two: ? }
    // println!("{:?}", GiveASeedAFertilizer::new().solve());
    // Solution { day: 6, part_one: 2344708, part_two: 30125202 }
    // println!("{:?}", WaitForIt::new().solve());
    // Solution { day: 7, part_one: 252656917, part_two: 253499763 }
    // println!("{:?}", CamelCards::new().solve());
    // Solution { day: 9, part_one: 0, part_two: 0 }
    println!("{:?}", HauntedWasteland::new().solve());
}
