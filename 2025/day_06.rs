//! https://adventofcode.com/2025/day/6
//! https://adventofcode.com/2025/day/6/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = (Vec<Vec<usize>>, Vec<bool>);

fn parse(input: &str) -> Parsed {
	let lines: Vec<_> = input.lines().collect();
	let numbers = lines
		.iter()
		.take(lines.len() - 1)
		.map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
		.collect();
	(
		numbers,
		lines
			.last()
			.unwrap()
			.split_whitespace()
			.map(|op| op == "+")
			.collect(),
	)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve((numbers, add): Parsed) -> usize {
		let mut result = 0;
		for (j, &add) in add.iter().enumerate() {
			let mut partial = if add { 0 } else { 1 };
			for row in &numbers {
				if add {
					partial += row[j];
				} else {
					partial *= row[j];
				}
			}
			result += partial;
		}
		result
	}
}

pub mod part2 {
	type Parsed = (Vec<Vec<usize>>, Vec<bool>);

	pub fn parse(input: &str) -> Parsed {
		let mut lines: Vec<_> = input.lines().collect();
		let ops = lines.pop().unwrap();
		let mut transposed = vec![String::with_capacity(lines.len()); lines[0].len() + 20];
		for line in lines {
			for (j, c) in line.chars().enumerate() {
				transposed[j].push(c);
			}
		}
		let new = transposed
			.iter()
			.map(|l| l.trim())
			.collect::<Vec<_>>()
			.join("\n");
		let new = new.trim();
		let numbers = new
			.split("\n\n")
			.map(|l| l.lines().map(|n| n.trim().parse().unwrap()).collect())
			.collect();
		(
			numbers,
			ops.split_whitespace().map(|op| op == "+").collect(),
		)
	}

	pub fn solve((numbers, add): Parsed) -> usize {
		let mut result = 0;
		for (add, row) in add.into_iter().zip(numbers) {
			let mut partial = if add { 0 } else { 1 };
			for number in row {
				if add {
					partial += number;
				} else {
					partial *= number;
				}
			}
			result += partial;
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_06_input.txt").unwrap()
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
	let result = part2::solve(part2::parse(&puzzle_input));
	let elapsed = start.elapsed();
	println!("{}", result);
	println!("Second part in {:?}", elapsed);
	total += elapsed;

	if verbose {
		println!("Total {:?}", total);
	}
	total
}
