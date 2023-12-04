//! https://adventofcode.com/2023/day/12
//! https://adventofcode.com/2023/day/12/input

use std::{fs::read_to_string, time::Instant};

type Parsed = usize;

fn parse(_input: &str) -> Parsed {
    0
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(_input: &str, _parsed: Parsed) -> usize {
        0
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(_input: &str, _parsed: Parsed) -> usize {
        0
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
    };
    let parsed = parse(&puzzle_input);
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, parsed));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, parsed));
    println!("Run in {:?}", start.elapsed());
}
