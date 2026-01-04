//! https://adventofcode.com/2025/day/7
//! https://adventofcode.com/2025/day/7/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<usize>;

fn parse(_input: &str) -> Parsed {
	vec![]
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_07_input.txt").unwrap()
	};

	let mut total = Duration::default();

	let start = Instant::now();
	let parsed = parse(&puzzle_input);
	let elapsed = start.elapsed();
	if verbose {
		println!("Parsed in {:?}", elapsed);
		total += elapsed;
	}

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

	if verbose {
		println!("Total {:?}", total);
	}
	total
}
