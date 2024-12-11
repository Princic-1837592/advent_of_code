//! https://adventofcode.com/2024/day/11
//! https://adventofcode.com/2024/day/11/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<usize>;

fn parse(input: &str) -> Parsed {
	input
		.split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect()
}

pub mod part1 {
	use std::collections::VecDeque;

	use super::Parsed;

	pub fn solve(stones: Parsed) -> usize {
		let mut queue: VecDeque<_> = stones.into_iter().map(|stone| (0, stone)).collect();
		let mut result = 0;
		while let Some((applied, stone)) = queue.pop_front() {
			if applied == 25 {
				result += 1;
			} else if stone == 0 {
				queue.push_back((applied + 1, 1));
			} else if stone.ilog10() % 2 == 1 {
				queue.push_back((applied + 1, stone % 10_usize.pow((stone.ilog10() + 1) / 2)));
				queue.push_back((applied + 1, stone / 10_usize.pow((stone.ilog10() + 1) / 2)));
			} else {
				queue.push_back((applied + 1, stone * 2024));
			}
		}
		result
	}
}

pub mod part2 {
	use std::collections::VecDeque;

	use super::Parsed;

	pub fn solve(stones: Parsed) -> usize {
		const CACHE_TARGET: usize = 40;
		let cache: Vec<_> = (0..10_usize)
			.map(|stone| {
				let mut queue: VecDeque<_> = VecDeque::from([(0, stone)]);
				let mut steps = vec![0; CACHE_TARGET + 1];
				while let Some((applied, stone)) = queue.pop_front() {
					steps[applied] += 1;
					if applied == CACHE_TARGET {
						continue;
					} else if stone == 0 {
						queue.push_back((applied + 1, 1));
					} else if stone.ilog10() % 2 == 1 {
						queue.push_back((
							applied + 1,
							stone % 10_usize.pow((stone.ilog10() + 1) / 2),
						));
						queue.push_back((
							applied + 1,
							stone / 10_usize.pow((stone.ilog10() + 1) / 2),
						));
					} else {
						queue.push_back((applied + 1, stone * 2024));
					}
				}
				steps
			})
			.collect();
		const TARGET: usize = 75;
		let mut queue: VecDeque<_> = stones.into_iter().map(|stone| (0, stone)).collect();
		let mut result = 0;
		while let Some((applied, stone)) = queue.pop_front() {
			if stone < 10 && TARGET - applied <= CACHE_TARGET {
				result += cache[stone][TARGET - applied];
			} else if applied == TARGET {
				result += 1;
			} else if stone == 0 {
				queue.push_back((applied + 1, 1));
			} else if stone.ilog10() % 2 == 1 {
				queue.push_back((applied + 1, stone % 10_usize.pow((stone.ilog10() + 1) / 2)));
				queue.push_back((applied + 1, stone / 10_usize.pow((stone.ilog10() + 1) / 2)));
			} else {
				queue.push_back((applied + 1, stone * 2024));
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "125 17".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_11_input.txt").unwrap()
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
