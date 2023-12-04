//! https://adventofcode.com/2017/day/2
//! https://adventofcode.com/2017/day/2/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse().unwrap())
                .collect()
        })
        .collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let matrix = parse(input);
        let mut sum = 0;
        for row in matrix {
            let (min, max) = (row.iter().min().unwrap(), row.iter().max().unwrap());
            sum += max - min;
        }
        sum
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let matrix = parse(input);
        let mut sum = 0;
        for row in matrix {
            'external: for i in 0..row.len() {
                for j in i + 1..row.len() {
                    if row[i] % row[j] == 0 {
                        sum += row[i] / row[j];
                        break 'external;
                    } else if row[j] % row[i] == 0 {
                        sum += row[j] / row[i];
                        break 'external;
                    }
                }
            }
        }
        sum
    }
}

pub fn main(test: bool) {
    let test_input = "5 9 2 8
9 4 7 3
3 8 6 5"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
