//! https://adventofcode.com/2025/day/3
//! https://adventofcode.com/2025/day/3/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| c.to_digit(10).unwrap() as usize)
				.collect()
		})
		.collect()
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(parsed: Parsed) -> usize {
		parsed
			.iter()
			.map(|bank| {
				let max_left = *bank[..bank.len() - 1].iter().max().unwrap();
				let max_right = *bank[bank.iter().position(|b| *b == max_left).unwrap() + 1..]
					.iter()
					.max()
					.unwrap();
				max_left * 10 + max_right
			})
			.sum()
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(parsed: Parsed) -> usize {
		parsed
			.iter()
			.map(|bank| {
				let mut sum = 0;
				let mut left = 0;
				for b in (0..12).rev() {
					sum *= 10;
					let (i, max) = bank[left..bank.len() - b].iter().enumerate().fold(
						(0, 0),
						|acc @ (_, prev_max), (i, &v)| if v > prev_max { (i, v) } else { acc },
					);
					left += i + 1;
					sum += max;
				}
				sum
			})
			.sum()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "987654321111111
811111111111119
234234234234278
818181911112111
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_03_input.txt").unwrap()
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
