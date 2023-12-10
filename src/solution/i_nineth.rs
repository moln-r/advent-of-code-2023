use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{AdventOfCode, Solution};

pub struct MirageMaintenance {
    day: i32,
    reports: Vec<Vec<i32>>,
}

impl AdventOfCode for MirageMaintenance {
    fn new() -> Self {
        let reports =
            BufReader::new(File::open("src/solution/inputs/input-09").expect("Error opening file"))
                .lines()
                .map(|line| line.unwrap())
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|split| split.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();
        MirageMaintenance { day: 9, reports }
    }

    fn solve(&self) -> Solution {
        let mut part_one = 0;
        for report in &self.reports {
            // println!("Report: {:?}", report);
            let mut sub_reports: Vec<Vec<i32>> = vec![];
            let mut temp = report.clone();
            while !zero_report(&temp) {
                temp = create_sub_report(&temp);
                sub_reports.push(temp.clone());
            }
            let lasts = sub_reports
                .iter()
                .map(|sr| *sr.last().unwrap())
                .collect::<Vec<i32>>();
            // println!("Lasts: {:?}", lasts);
            let next_value = report.last().unwrap() + lasts.iter().sum::<i32>();
            // println!("Next value: {}", next_value);
            part_one += next_value;
            // println!("---------");
        }

        Solution {
            day: self.day,
            part_one: part_one.try_into().unwrap(),
            part_two: 0,
        }
    }
}

fn create_sub_report(report: &Vec<i32>) -> Vec<i32> {
    let mut sub_report: Vec<i32> = vec![];
    for i in 0..report.len() - 1 {
        sub_report.push(report[i + 1] - report[i]);
    }
    // println!("Sub report: {:?}", sub_report);
    sub_report
}

fn zero_report(report: &Vec<i32>) -> bool {
    report.iter().min().unwrap() == &0 && report.iter().max().unwrap() == &0
}
