//! https://adventofcode.com/2024/day/9
//! https://adventofcode.com/2024/day/9/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<usize>;

fn parse(input: &str) -> Parsed {
	input
		.trim()
		.chars()
		.map(|c| c as usize - '0' as usize)
		.collect()
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(blocks: Parsed) -> usize {
		let mut memory = Vec::with_capacity(blocks.iter().sum());
		for (b, block_size) in blocks.into_iter().enumerate() {
			for _ in 0..block_size {
				memory.push(if b % 2 == 0 { b / 2 } else { usize::MAX });
			}
		}
		let mut left_i = 0;
		let mut right_i = memory.len() - 1;
		let mut result = 0;
		while left_i < right_i {
			if memory[left_i] != usize::MAX {
				result += left_i * memory[left_i];
				left_i += 1;
			} else if memory[right_i] == usize::MAX {
				right_i -= 1;
			} else {
				result += left_i * memory[right_i];
				left_i += 1;
				right_i -= 1;
			}
		}
		result
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "2333133121414131402".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_09_input.txt").unwrap()
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
