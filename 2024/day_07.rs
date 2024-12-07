//! https://adventofcode.com/2024/day/7
//! https://adventofcode.com/2024/day/7/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

#[derive(Debug, Clone)]
pub struct Equation {
	target: u64,
	factors: Vec<u64>,
}

impl FromStr for Equation {
	type Err = ();

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let mut parts = line.split(':');
		let target = parts.next().unwrap().parse().unwrap();
		let factors = parts.next().unwrap()[1..]
			.split_whitespace()
			.map(|n| n.parse().unwrap())
			.collect();
		Ok(Equation { target, factors })
	}
}

type Parsed = Vec<Equation>;

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::{Equation, Parsed};

	fn valid(equation: &Equation) -> bool {
		for combo in 0..1 << (equation.factors.len() - 1) {
			let mut accumulator = equation.factors[0];
			for f in 1..equation.factors.len() {
				if combo & (1 << (f - 1)) != 0 {
					accumulator *= equation.factors[f];
				} else {
					accumulator += equation.factors[f];
				}
				if accumulator > equation.target {
					return false;
				}
			}
			if accumulator == equation.target {
				return true;
			}
		}
		false
	}

	pub fn solve(equations: Parsed) -> u64 {
		equations
			.into_par_iter()
			.filter(valid)
			.map(|eq| eq.target)
			.sum()
	}
}

pub mod part2 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::{Equation, Parsed};

	fn valid(equation: &Equation) -> bool {
		for mut combo in 0..=3_u64.pow((equation.factors.len() - 1) as u32) {
			let mut accumulator = equation.factors[0];
			for f in 1..equation.factors.len() {
				match combo % 3 {
					0 => accumulator *= equation.factors[f],
					1 => accumulator += equation.factors[f],
					2 => {
						accumulator = accumulator * 10_u64.pow(equation.factors[f].ilog10() + 1)
							+ equation.factors[f]
					}
					_ => unreachable!(),
				}
				if accumulator > equation.target {
					return false;
				}
				combo /= 3;
			}
			if accumulator == equation.target {
				return true;
			}
		}
		false
	}

	pub fn solve(equations: Parsed) -> u64 {
		equations
			.into_par_iter()
			.filter(valid)
			.map(|eq| eq.target)
			.sum()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_07_input.txt").unwrap()
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
