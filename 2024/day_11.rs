//! https://adventofcode.com/2024/day/11
//! https://adventofcode.com/2024/day/11/input

use std::{
	collections::{hash_map::Entry, HashMap},
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

fn nonna<const TARGET: usize>(
	stone: usize,
	steps_left: usize,
	cache: &mut HashMap<usize, Vec<usize>>,
) -> usize {
	if steps_left == 0 {
		return 1;
	}
	match cache.entry(stone) {
		Entry::Occupied(entry) => {
			if entry.get()[steps_left] != usize::MAX {
				return entry.get()[steps_left];
			}
		}
		Entry::Vacant(entry) => {
			entry.insert(vec![usize::MAX; TARGET + 1]);
		}
	}
	if stone == 0 {
		let result = nonna::<TARGET>(1, steps_left - 1, cache);
		cache.get_mut(&stone).unwrap()[steps_left] = result;
		return result;
	}
	if stone.ilog10() % 2 == 1 {
		let mut result = nonna::<TARGET>(
			stone % 10_usize.pow((stone.ilog10() + 1) / 2),
			steps_left - 1,
			cache,
		);
		result += nonna::<TARGET>(
			stone / 10_usize.pow((stone.ilog10() + 1) / 2),
			steps_left - 1,
			cache,
		);
		cache.get_mut(&stone).unwrap()[steps_left] = result;
		return result;
	}
	let result = nonna::<TARGET>(stone * 2024, steps_left - 1, cache);
	cache.get_mut(&stone).unwrap()[steps_left] = result;
	result
}

pub fn solve_generic<const TARGET: usize>(stones: Parsed) -> usize {
	let mut cache = HashMap::new();
	stones
		.into_iter()
		.map(|stone| nonna::<TARGET>(stone, TARGET, &mut cache))
		.sum()
}

pub mod part1 {
	use super::{solve_generic, Parsed};

	pub fn solve(stones: Parsed) -> usize {
		solve_generic::<25>(stones)
	}
}

pub mod part2 {
	use super::{solve_generic, Parsed};

	pub fn solve(stones: Parsed) -> usize {
		solve_generic::<75>(stones)
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
