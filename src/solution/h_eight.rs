#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::{AdventOfCode, Solution};

#[derive(Debug)]
pub struct HauntedWasteland {
    day: i32,
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Self {
        return match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        };
    }
}

impl HauntedWasteland {
    fn do_step(&self, current_node: &str, direction: &Direction) -> &String {
        let node_info = self
            .nodes
            .get(current_node)
            .expect("Node in missing from map");

        match direction {
            Direction::Left => &node_info.0,
            Direction::Right => &node_info.1,
        }
    }
}

impl AdventOfCode for HauntedWasteland {
    fn new() -> Self {
        let mut lines =
            BufReader::new(File::open("src/solution/inputs/input-08").expect("Error opening file"))
                .lines();

        // first line is the directions
        let directions = lines
            .next()
            .expect("No direction line in file")
            .expect("Could not parse first line")
            .chars()
            .map(|c| Direction::new(c))
            .collect::<Vec<Direction>>();

        // skipping empty line
        lines.next();

        let node_regex = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();

        // looping over nodes
        let nodes = lines
            .into_iter()
            .map(|line| line.unwrap())
            .map(|line| {
                let node_info = node_regex.captures(&line).unwrap();

                (
                    node_info.get(1).unwrap().as_str().to_string(),
                    (
                        node_info.get(2).unwrap().as_str().to_string(),
                        node_info.get(3).unwrap().as_str().to_string(),
                    ),
                )
            })
            .collect::<HashMap<String, (String, String)>>();

        HauntedWasteland {
            day: 8,
            directions,
            nodes,
        }
    }

    fn solve(&self) -> Solution {
        // part one
        let mut current_node = "AAA";
        let last_node = "ZZZ";
        let mut steps = 0;
        loop {
            self.directions
                .iter()
                .for_each(|dir| current_node = self.do_step(current_node, dir));
            steps += self.directions.len();
            if current_node == last_node {
                break;
            }
        }

        // part two
        let mut ghost_nodes = self
            .nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .collect::<Vec<&String>>();
        // println!("Ghost nodes: {:?}", ghost_nodes);
        let mut ghost_iter_counts: Vec<i64> = vec![];
        let mut ghost_iters: i64 = 0;

        loop {
            self.directions.iter().for_each(|dir| {
                // println!("Each ghost node [{:?}] goes to {:?}", ghost_nodes, dir);
                for i in 0..ghost_nodes.len() {
                    ghost_nodes[i] = self.do_step(ghost_nodes[i], dir);
                }
            });
            ghost_iters += 1;

            for i in (0..ghost_nodes.len()).rev() {
                if ghost_nodes[i].ends_with('Z') {
                    ghost_iter_counts.push(ghost_iters);
                    ghost_nodes.remove(i);
                }
            }

            if ghost_nodes.is_empty() {
                break;
            }
        }

        let ghost_steps = lcm_for_vector(
            ghost_iter_counts
                .iter()
                .map(|i| (i * (self.directions.len() as i64)) as u128)
                .collect::<Vec<u128>>(),
        );

        Solution {
            day: self.day,
            part_one: steps.try_into().unwrap(),
            part_two: ghost_steps.try_into().unwrap(),
        }
    }
}

fn lcm_for_vector(numbers: Vec<u128>) -> u128 {
    let mut least_common_multiplier: u128 = numbers[0];
    for i in 1..numbers.len() {
        least_common_multiplier = num::integer::lcm(least_common_multiplier, numbers[i]);
    }
    least_common_multiplier
}
