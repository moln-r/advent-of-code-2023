use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solution::j_tenth::Direction::*;
use crate::{AdventOfCode, Solution};

const START: char = 'S';

pub struct PipeMaze {
    day: i32,
    maze: Vec<Vec<char>>,
    test_maze: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
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
        let maze = &self.maze;
        let mut pipeline = vec![];
        // let maze = &self.test_maze;

        let start_position = find_start(maze);
        println!("{:?}", start_position);

        let mut position: Position = start_position.clone();
        let mut previous_direction: Option<Direction> = None;
        loop {
            pipeline.push(position.clone());
            // println!("\nWe are at {:?}, made step to {:?}", position, previous_direction);
            let next_move = get_next_move(maze, &position, &previous_direction);
            let next_position = move_to(&position, &next_move);

            // println!("We are moving {:?}, to {:?}", next_move, next_position);
            let current_char = char_at_position(&maze, &next_position);
            // println!("Next step is on a {}", current_char);
            if current_char == START {
                // we're done
                break;
            }
            previous_direction = Some(next_move);
            position = next_position;
        }

        let step = pipeline.len();
        let half = if step % 2 == 0 {
            step / 2
        } else {
            (step + 1) / 2
        };
        println!(
            "We made {} steps, so the farthest position is {} steps away",
            step, half
        );

        Solution {
            day: self.day,
            part_one: half.try_into().unwrap(),
            part_two: 0,
        }
    }
}

// finds where 'S' is
fn find_start(maze: &Vec<Vec<char>>) -> Position {
    for (y, row) in maze.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == START {
                return Position { x, y };
            }
        }
    }
    panic!("No start found");
}

fn char_at_position(maze: &Vec<Vec<char>>, pos: &Position) -> char {
    maze.get(pos.y).unwrap().get(pos.x).unwrap().clone()
}

// finds which way to go
fn get_next_move(
    maze: &Vec<Vec<char>>,
    pos: &Position,
    previous_dir: &Option<Direction>,
) -> Direction {
    let char = char_at_position(maze, pos);

    let possible_moves = match char {
        'S' => vec![Up, Down, Left, Right],
        '-' => vec![Left, Right],
        '|' => vec![Up, Down],
        'F' => vec![Down, Right],
        '7' => vec![Down, Left],
        'J' => vec![Up, Left],
        'L' => vec![Up, Right],
        _ => panic!("We are calculating route from an invalid character!"),
    };

    for dir in possible_moves {
        // println!("Trying to find move: {:?}", dir);
        if let Some(from) = previous_dir {
            if dir == from.opposite() {
                // we don't want to go back
                // println!("No no, we've been here before");
                continue;
            }
        }
        if let Some(next_pos) = get_next_position(maze, &pos, &dir) {
            if can_go(maze, &dir, &next_pos) {
                return dir;
            }
        }
    }
    panic!("Could not find valid direction")
}

// gets the next coordinates
fn get_next_position(
    maze: &Vec<Vec<char>>,
    pos: &Position,
    direction: &Direction,
) -> Option<Position> {
    let coordinates = match direction {
        Up => {
            if pos.y == 0 {
                None
            } else {
                Some((pos.x, pos.y - 1))
            }
        }
        Down => {
            if pos.y == maze.len() - 1 {
                None
            } else {
                Some((pos.x, pos.y + 1))
            }
        }
        Left => {
            if pos.x == 0 {
                None
            } else {
                Some((pos.x - 1, pos.y))
            }
        }
        Right => {
            if pos.x == maze.get(pos.y).unwrap().len() - 1 {
                None
            } else {
                Some((pos.x + 1, pos.y))
            }
        }
    };
    match coordinates {
        None => None,
        Some(coordinates) => Some(Position {
            x: coordinates.0,
            y: coordinates.1,
        }),
    }
}

const WAY_UP: [char; 4] = ['S', '|', 'F', '7'];
const WAY_DOWN: [char; 4] = ['S', '|', 'L', 'J'];
const WAY_LEFT: [char; 4] = ['S', '-', 'F', 'L'];
const WAY_RIGHT: [char; 4] = ['S', '-', '7', 'J'];

fn can_go(maze: &Vec<Vec<char>>, direction: &Direction, pos: &Position) -> bool {
    let char = char_at_position(maze, pos);
    let ways = match direction {
        Up => WAY_UP,
        Down => WAY_DOWN,
        Left => WAY_LEFT,
        Right => WAY_RIGHT,
    };
    ways.contains(&char)
}

fn move_to(pos: &Position, dir: &Direction) -> Position {
    match dir {
        Up => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Down => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Left => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Right => Position {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}
