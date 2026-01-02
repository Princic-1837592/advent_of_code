//! https://adventofcode.com/2025/day/5
//! https://adventofcode.com/2025/day/5/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

type Parsed = (Vec<(usize, usize)>, Vec<usize>);

fn parse(input: &str) -> Parsed {
	let separator = &LINE_ENDING.repeat(2);
	let (ranges, ids) = input.split_once(separator).unwrap();
	let mut ranges: Vec<(usize, usize)> = ranges
		.lines()
		.map(|l| {
			let (left, right) = l.split_once('-').unwrap();
			(left.parse().unwrap(), right.parse().unwrap())
		})
		.collect();
	ranges.sort();
	for i in (1..ranges.len()).rev() {
		if ranges[i].0 <= ranges[i - 1].1 {
			ranges[i - 1].1 = (ranges[i - 1].1).max(ranges[i].1);
			ranges[i - 1].0 = (ranges[i - 1].0).min(ranges[i].0);
			ranges.remove(i);
		}
	}
	(ranges, ids.lines().map(|l| l.parse().unwrap()).collect())
}

pub mod part1 {
	use super::Parsed;

	pub fn solve((ranges, ids): Parsed) -> usize {
		let mut result = 0;
		for id in ids {
			for &(left, right) in &ranges {
				if left <= id && id <= right {
					result += 1;
					break;
				}
			}
		}
		result
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve((ranges, _): Parsed) -> usize {
		let mut result = 0;
		for (left, right) in ranges {
			result += right - left + 1;
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "3-5\r
10-14\r
16-20\r
12-18\r
\r
1\r
5\r
8\r
11\r
17\r
32"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_05_input.txt").unwrap()
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
