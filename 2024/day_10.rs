//! https://adventofcode.com/2024/day/10
//! https://adventofcode.com/2024/day/10/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<Vec<u8>>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| c.to_digit(10).unwrap_or(100) as u8)
				.collect()
		})
		.collect()
}

pub mod part1 {
	use std::collections::VecDeque;

	use utils::coords::iter_cross_near;

	use super::Parsed;

	pub fn solve(map: Parsed) -> usize {
		let mut result = 0;
		let mut found = vec![vec![false; map[0].len()]; map.len()];
		for (i, row) in map.iter().enumerate() {
			for (j, &digit) in row.iter().enumerate() {
				if digit == 0 {
					found.iter_mut().for_each(|r| r.fill(false));
					let mut queue = VecDeque::from([((i, j), digit)]);
					while let Some(((i, j), value)) = queue.pop_front() {
						if value == 9 {
							if !found[i][j] {
								found[i][j] = true;
								result += 1;
							}
							continue;
						}
						for (ni, nj) in iter_cross_near(i as isize, j as isize)
							.map(|(i, j)| (i as usize, j as usize))
						{
							if ni >= map.len() || nj >= map[0].len() || map[ni][nj] != value + 1 {
								continue;
							}
							queue.push_back(((ni, nj), map[ni][nj]));
						}
					}
				}
			}
		}
		result
	}
}

pub mod part2 {
	use std::collections::VecDeque;

	use utils::coords::iter_cross_near;

	use super::Parsed;

	pub fn solve(map: Parsed) -> usize {
		let mut result = 0;
		for (i, row) in map.iter().enumerate() {
			for (j, &digit) in row.iter().enumerate() {
				if digit == 0 {
					let mut queue = VecDeque::from([((i, j), digit)]);
					while let Some(((i, j), value)) = queue.pop_front() {
						if value == 9 {
							result += 1;
							continue;
						}
						for (ni, nj) in iter_cross_near(i as isize, j as isize)
							.map(|(i, j)| (i as usize, j as usize))
						{
							if ni >= map.len() || nj >= map[0].len() || map[ni][nj] != value + 1 {
								continue;
							}
							queue.push_back(((ni, nj), map[ni][nj]));
						}
					}
				}
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_10_input.txt").unwrap()
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
