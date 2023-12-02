use crate::solution::first::Trebuchet;
use crate::solution::second::CubeConundrum;

pub mod solution;

fn main() {
    println!("Running Advent of code 2023...");

    // let trebuchet = Trebuchet::new();
    // println!("First day's answer: {}", trebuchet.solve());

    let second_day_solution = CubeConundrum::new().solve();
    println!("Second day's first answer is {}, second answer is {}", second_day_solution.0, second_day_solution.1);

}
