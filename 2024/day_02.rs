//! https://adventofcode.com/2024/day/2
//! https://adventofcode.com/2024/day/2/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|line| {
			line.split_whitespace()
				.map(|n| n.parse().unwrap())
				.collect()
		})
		.collect()
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(reports: Parsed) -> usize {
		reports
			.into_iter()
			.filter(|report| {
				let mut diffs = Vec::with_capacity(report.len() - 1);
				for i in 1..report.len() {
					diffs.push(report[i] as isize - report[i - 1] as isize);
				}
				diffs.iter().all(|&diff| {
					diff.signum() == diffs[0].signum() && 1 <= diff.abs() && diff.abs() <= 3
				})
			})
			.count()
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(reports: Parsed) -> usize {
		reports
			.into_iter()
			.filter(|report| {
				let mut diffs = Vec::with_capacity(report.len() - 1);
				for i in 1..report.len() {
					diffs.push(report[i] as isize - report[i - 1] as isize);
				}
				diffs.iter().all(|&diff| {
					diff.signum() == diffs[0].signum() && 1 <= diff.abs() && diff.abs() <= 3
				}) || {
					(0..report.len()).any(|to_remove| {
						let mut report = report.clone();
						report.remove(to_remove);
						let mut diffs = Vec::with_capacity(report.len() - 1);
						for i in 1..report.len() {
							diffs.push(report[i] as isize - report[i - 1] as isize);
						}
						diffs.iter().all(|&diff| {
							diff.signum() == diffs[0].signum() && 1 <= diff.abs() && diff.abs() <= 3
						})
					})
				}
			})
			.count()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_02_input.txt").unwrap()
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
