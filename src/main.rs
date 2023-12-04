use crate::solution::first::Trebuchet;
use crate::solution::fourth::Scratchcards;
use crate::solution::second::CubeConundrum;
use crate::solution::third::GearRatios;

pub mod solution;

#[derive(Debug)]
pub struct Solution {
    pub day: i32,
    pub part_one: i32,
    pub part_two: i32,
}

trait AdventOfCode {
    fn new() -> Self;
    fn solve(&self) -> Solution;
}

fn main() {
    println!("Running Advent of code 2023...");
    // Solution { day: 1, part_one: 53651, part_two: 53894 }
    println!("{:?}", Trebuchet::new().solve());
    // Solution { day: 2, part_one: 2207, part_two: 62241 }
    println!("{:?}", CubeConundrum::new().solve());
    // Solution { day: 3, part_one: 533775, part_two: 78236071 }
    println!("{:?}", GearRatios::new().solve());
    // Solution { day: 4, part_one: 24733, part_two: 0 }
    println!("{:?}", Scratchcards::new().solve());
}
