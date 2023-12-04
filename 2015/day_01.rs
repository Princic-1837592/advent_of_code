//! https://adventofcode.com/2015/day/1
//! https://adventofcode.com/2015/day/1/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

pub mod part1 {
    pub fn solve(input: &str) -> isize {
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum()
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

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_01_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
