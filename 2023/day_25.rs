//! https://adventofcode.com/2023/day/25
//! https://adventofcode.com/2023/day/25/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<usize>;

fn parse(_input: &str) -> Parsed {
    unimplemented!()
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {
        unimplemented!()
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {
        unimplemented!()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_25_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
