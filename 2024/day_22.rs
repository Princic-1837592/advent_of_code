//! https://adventofcode.com/2024/day/22
//! https://adventofcode.com/2024/day/22/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

type Parsed = Vec<u64>;

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve(secrets: Parsed) -> u64 {
		secrets
			.into_par_iter()
			.map(|mut secret| {
				for _ in 0..2000 {
					let mut next = secret << 6;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;

					next = secret >> 5;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;

					next = secret << 11;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;
				}
				secret
			})
			.sum()
	}
}

pub mod part2 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	#[derive(Copy, Clone, Debug)]
	struct Secret {
		cost: u8,
		diff: usize,
	}

	pub fn solve(secrets: Parsed) -> usize {
		let secrets: Vec<_> = secrets
			.into_par_iter()
			.map(|mut secret| {
				let mut result = Vec::with_capacity(2000);
				for _ in 0..2000 {
					let old = secret;
					let mut next = secret << 6;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;

					next = secret >> 5;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;

					next = secret << 11;
					next ^= secret;
					secret = next & 0b11111111_11111111_11111111;
					result.push(Secret {
						cost: (secret % 10) as u8,
						diff: (secret % 10).wrapping_sub(old % 10).wrapping_add(9) as usize,
					});
				}
				result
			})
			.collect();
		let first_occurrence: Vec<_> = secrets
			.into_par_iter()
			.map(|monkey| {
				let mut result = vec![u8::MAX; 130_321];
				for s in 3..monkey.len() {
					let diff = monkey[s - 3].diff * 6859
						+ monkey[s - 2].diff * 361
						+ monkey[s - 1].diff * 19
						+ monkey[s].diff;
					if result[diff] == u8::MAX {
						result[diff] = monkey[s].cost;
					}
				}
				result
			})
			.collect();
		(0..130_321)
			.into_par_iter()
			.map(|v| {
				first_occurrence
					.iter()
					.map(|f| if f[v] != u8::MAX { f[v] as usize } else { 0 })
					.sum()
			})
			.max()
			.unwrap()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "1
2
3
2024
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_22_input.txt").unwrap()
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
