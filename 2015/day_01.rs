//! https://adventofcode.com/2015/day/1
//! https://adventofcode.com/2015/day/1/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    pub fn solve(input: &str) -> isize {
        input.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        })
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> usize {
        let mut floor = 0;
        for (i, c) in input.chars().enumerate() {
            floor += match c {
                ')' => -1,
                '(' => 1,
                _ => 0,
            };
            if floor <= -1 {
                return i + 1;
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_01_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
