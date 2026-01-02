//! https://adventofcode.com/2025/day/4
//! https://adventofcode.com/2025/day/4/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<Vec<bool>>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|l| l.chars().map(|c| c == '@').collect())
		.collect()
}

pub mod part1 {
	use utils::coords::u_iter_near;

	use super::Parsed;

	pub fn solve(map: Parsed) -> usize {
		let mut near = vec![vec![0; map[0].len()]; map.len()];
		let mut result = 0;
		for (i, row) in map.iter().enumerate() {
			for (j, &cell) in row.iter().enumerate() {
				if cell {
					for (ni, nj) in u_iter_near(i, j, map[0].len(), map.len()) {
						near[ni][nj] += 1;
					}
				}
			}
		}
		for (i, row) in near.iter().enumerate() {
			for (j, &cell) in row.iter().enumerate() {
				if map[i][j] && cell < 4 {
					result += 1;
				}
			}
		}
		result
	}
}

pub mod part2 {
	use utils::coords::u_iter_near;

	use super::Parsed;

	pub fn solve(mut map: Parsed) -> usize {
		let mut result = 0;
		let mut partial = 1;
		while partial != 0 {
			let mut near = vec![vec![0; map[0].len()]; map.len()];
			partial = 0;
			for (i, row) in map.iter().enumerate() {
				for (j, &cell) in row.iter().enumerate() {
					if cell {
						for (ni, nj) in u_iter_near(i, j, map[0].len(), map.len()) {
							near[ni][nj] += 1;
						}
					}
				}
			}
			for (i, row) in near.iter().enumerate() {
				for (j, &cell) in row.iter().enumerate() {
					if map[i][j] && cell < 4 {
						partial += 1;
						map[i][j] = false;
					}
				}
			}
			result += partial;
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_04_input.txt").unwrap()
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
