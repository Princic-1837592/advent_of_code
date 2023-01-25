//! https://adventofcode.com/2018/day/1
//! https://adventofcode.com/2018/day/1/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub mod part1 {
    use crate::day_01::parse;

    pub fn solve(input: &str) -> isize {
        parse(input).iter().sum()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use crate::day_01::parse;

    pub fn solve(input: &str) -> isize {
        let frequencies = parse(input);
        let mut seen = HashSet::new();
        let mut frequency = 0;
        let mut i = 0;
        loop {
            frequency += frequencies[i];
            if seen.contains(&frequency) {
                return frequency;
            }
            seen.insert(frequency);
            i = (i + 1) % frequencies.len();
        }
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
