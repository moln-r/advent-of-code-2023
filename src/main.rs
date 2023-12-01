use crate::solution::first::Trebuchet;

pub mod solution;

fn main() {
    println!("Running Advent of code 2023...");
    let trebuchet = Trebuchet::new();
    println!("First day's answer: {}", trebuchet.solve());
}
