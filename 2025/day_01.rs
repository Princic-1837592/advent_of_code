//! https://adventofcode.com/2025/day/1
//! https://adventofcode.com/2025/day/1/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

#[derive(Copy, Clone)]
pub enum Rotation {
	R(usize),
	L(usize),
}

impl FromStr for Rotation {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().next().unwrap() {
			'R' => Ok(Self::R(s[1..].parse().unwrap())),
			'L' => Ok(Self::L(s[1..].parse().unwrap())),
			_ => Err(()),
		}
	}
}

type Parsed = Vec<Rotation>;

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use super::{Parsed, Rotation};

	pub fn solve(parsed: Parsed) -> usize {
		let mut count = 0;
		let mut dial = 50;
		for instr in parsed {
			dial = match instr {
				Rotation::R(v) => dial + v,
				Rotation::L(v) => dial + 100 - v % 100,
			} % 100;
			if dial == 0 {
				count += 1;
			}
		}
		count
	}
}

pub mod part2 {
	use super::{Parsed, Rotation};

	pub fn solve(parsed: Parsed) -> usize {
		let mut count: usize = 0;
		let mut dial = 50;
		for instr in parsed {
			match instr {
				Rotation::R(v) => {
					count += v / 100;
					if dial + v % 100 >= 100 {
						count += 1;
					}
					dial = (dial + v) % 100
				}
				Rotation::L(v) => {
					count += v / 100;
					if v % 100 >= dial && dial != 0 {
						count += 1;
					}
					dial = (dial + 100 - v % 100) % 100
				}
			}
		}
		count
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_01_input.txt").unwrap()
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
