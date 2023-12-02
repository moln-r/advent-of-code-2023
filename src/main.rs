use crate::solution::first::Trebuchet;
use crate::solution::second::CubeConundrum;

pub mod solution;

fn main() {
    println!("Running Advent of code 2023...");
    let trebuchet = Trebuchet::new();
    println!("First day's answer: {}", trebuchet.solve());

    let cube_conundrum = CubeConundrum::new();
    println!("Second day's answer: {}", cube_conundrum.solve());
}
