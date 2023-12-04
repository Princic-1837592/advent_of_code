//! https://adventofcode.com/2023/day/9
//! https://adventofcode.com/2023/day/9/input

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

    pub fn solve(_input: &str, _parsed: Parsed) -> usize {
        unimplemented!()
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(_input: &str, _parsed: Parsed) -> usize {
        unimplemented!()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_09_input.txt").unwrap()
    };

    let parsed = parse(&puzzle_input);
    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Run in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Run in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
