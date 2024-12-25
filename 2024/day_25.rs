//! https://adventofcode.com/2024/day/25
//! https://adventofcode.com/2024/day/25/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

type Parsed = (Vec<u64>, Vec<u64>);

fn parse(input: &str) -> Parsed {
	let (mut locks, mut keys) = (
		Vec::with_capacity(input.lines().count() / 2),
		Vec::with_capacity(input.lines().count() / 2),
	);
	let sep = LINE_ENDING.repeat(2);
	for schematic in input.split(&sep) {
		let mut bits = 0;
		for char in schematic.lines().flat_map(|l| l.chars()) {
			bits <<= 1;
			if char == '#' {
				bits |= 1;
			}
		}
		if schematic.starts_with('#') {
			locks.push(bits);
		} else {
			keys.push(bits);
		}
	}
	(locks, keys)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve((locks, keys): Parsed) -> usize {
		let mut result = 0;
		for lock in locks {
			for &key in keys.iter() {
				if lock & key == 0 {
					result += 1;
				}
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_25_input.txt").unwrap()
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

	if verbose {
		println!("Total {:?}", total);
	}
	total
}
