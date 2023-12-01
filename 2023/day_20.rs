//! https://adventofcode.com/2023/day/20
//! https://adventofcode.com/2023/day/20/input

#![allow(unused)]

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> usize {
    0
}

pub mod part1 {
    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        0
    }
}

pub mod part2 {
    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        0
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_20_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
