//! https://adventofcode.com/2024/day/1
//! https://adventofcode.com/2024/day/1/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = (Vec<usize>, Vec<usize>);

fn parse(input: &str) -> Parsed {
	let (mut left, mut right) = (Vec::new(), Vec::new());
	for line in input.lines() {
		let mut parts = line.split_whitespace();
		left.push(parts.next().unwrap().parse().unwrap());
		right.push(parts.next().unwrap().parse().unwrap());
	}
	(left, right)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve((mut left, mut right): Parsed) -> usize {
		left.sort();
		right.sort();
		left.into_iter()
			.zip(right)
			.map(|(l, r)| l.abs_diff(r))
			.sum()
	}
}

pub mod part2 {
	use std::collections::HashMap;

	use super::Parsed;

	pub fn solve((left, right): Parsed) -> usize {
		let mut occurrencies = HashMap::new();
		for n in right {
			*occurrencies.entry(n).or_insert(0) += 1;
		}
		left.into_iter()
			.map(|n| n * occurrencies.get(&n).unwrap_or(&0))
			.sum()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_01_input.txt").unwrap()
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
