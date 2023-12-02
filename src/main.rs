use crate::solution::first::Trebuchet;
use crate::solution::second::CubeConundrum;

pub mod solution;

#[derive(Debug)]
pub struct Solution {
    pub day: i32,
    pub part_one: i32,
    pub part_two: i32,
}

trait AdventOfCode {
    fn solve(&self) -> Solution;
}

fn main() {
    println!("Running Advent of code 2023...");
    println!("{:?}", Trebuchet::new().solve());
    println!("{:?}", CubeConundrum::new().solve());
}
