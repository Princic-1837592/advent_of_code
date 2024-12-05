//! https://adventofcode.com/2024/day/3
//! https://adventofcode.com/2024/day/3/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use regex::Regex;

type Parsed = Vec<Op>;

#[derive(Copy, Clone, Debug)]
pub enum Op {
	Do,
	Dont,
	Mul(usize, usize),
}

fn parse(input: &str) -> Parsed {
	let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
	pattern
		.captures_iter(input)
		.map(|c| match c[0].len() {
			4 => Op::Do,
			7 => Op::Dont,
			_ => Op::Mul(c[1].parse().unwrap(), c[2].parse().unwrap()),
		})
		.collect()
}

pub mod part1 {
	use super::{Op, Parsed};

	pub fn solve(muls: Parsed) -> usize {
		muls.into_iter()
			.filter_map(|op| match op {
				Op::Mul(a, b) => Some((a, b)),
				_ => None,
			})
			.map(|(a, b)| a * b)
			.sum()
	}
}

pub mod part2 {
	use super::{Op, Parsed};

	pub fn solve(ops: Parsed) -> usize {
		let mut active = true;
		let mut result = 0;
		for op in ops {
			match op {
				Op::Do => active = true,
				Op::Dont => active = false,
				Op::Mul(a, b) => {
					if active {
						result += a * b
					}
				}
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_03_input.txt").unwrap()
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
