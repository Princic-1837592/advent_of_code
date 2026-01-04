//! https://adventofcode.com/2025/day/7
//! https://adventofcode.com/2025/day/7/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::parsing::parse_matrix;

type Parsed = (Vec<Vec<Tile>>, usize);

#[derive(Debug, Copy, Clone)]
pub enum Tile {
	Start,
	Free,
	Splitter,
}

impl From<char> for Tile {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Free,
			'^' => Self::Splitter,
			_s => Self::Start,
		}
	}
}

fn parse(input: &str) -> Parsed {
	let map = parse_matrix(input);
	let start = map[0]
		.iter()
		.position(|t| matches!(t, Tile::Start))
		.unwrap();
	(map, start)
}

pub mod part1 {
	use std::collections::VecDeque;

	use super::{Parsed, Tile};

	pub fn solve((map, start): Parsed) -> usize {
		let (width, height) = (map[0].len(), map.len());
		let mut result = 0;
		let mut queue = VecDeque::from([(0, start)]);
		let mut used = vec![vec![false; width]; height];
		while let Some((i, j)) = queue.pop_front() {
			if i >= height {
				continue;
			}
			if matches!(map[i][j], Tile::Splitter) {
				if !used[i][j] {
					used[i][j] = true;
					queue.push_back((i, j - 1));
					queue.push_back((i, j + 1));
					result += 1;
				}
			} else {
				queue.push_back((i + 1, j));
			}
		}
		result
	}
}

pub mod part2 {
	use super::{Parsed, Tile};

	pub fn solve((map, start): Parsed) -> usize {
		let (width, height) = (map[0].len(), map.len());
		let mut dp = vec![vec![0; width]; height];
		dp[1][start] = 1;
		for (i, row) in map.iter().enumerate().skip(2) {
			for j in (0..width).filter(|&j| matches!(row[j], Tile::Free)) {
				if j > 0 && matches!(row[j - 1], Tile::Splitter) {
					dp[i][j] += dp[i - 1][j - 1];
				}
				if j < width - 1 && matches!(row[j + 1], Tile::Splitter) {
					dp[i][j] += dp[i - 1][j + 1];
				}
				dp[i][j] += dp[i - 1][j];
			}
		}
		let result = dp.last().unwrap().iter().sum();
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_07_input.txt").unwrap()
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
