//! https://adventofcode.com/2024/day/6
//! https://adventofcode.com/2024/day/6/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = (Vec<Vec<bool>>, (usize, usize));

fn parse(input: &str) -> Parsed {
	let map = input
		.lines()
		.map(|l| l.chars().map(|c| c == '#').collect())
		.collect();
	let row = input.lines().position(|l| l.contains('^')).unwrap();
	let col = input
		.lines()
		.nth(row)
		.unwrap()
		.chars()
		.position(|c| c == '^')
		.unwrap();
	(map, (row, col))
}

fn route(map: Vec<Vec<bool>>, (mut i, mut j): (usize, usize)) -> Vec<Vec<bool>> {
	let (mut di, mut dj) = (-1, 0);
	let mut seen = vec![vec![false; map[0].len()]; map.len()];
	seen[i][j] = true;
	loop {
		let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
		if ni >= map.len() || nj >= map[0].len() {
			break;
		}
		if map[ni][nj] {
			(di, dj) = (dj, -di);
		} else {
			(i, j) = (ni, nj);
			seen[i][j] = true;
		}
	}
	seen
}

pub mod part1 {
	use super::{route, Parsed};

	pub fn solve((map, start): Parsed) -> usize {
		let seen = route(map, start);
		seen.into_iter().flatten().filter(|&b| b).count()
	}
}

pub mod part2 {
	use super::{route, Parsed};

	pub fn solve((mut map, start): Parsed) -> usize {
		let seen: Vec<_> = route(map.clone(), start)
			.iter()
			.enumerate()
			.flat_map(|(i, row)| {
				row.iter()
					.enumerate()
					.filter(|&(_, &b)| b)
					.map(move |(j, _)| (i, j))
			})
			.collect();
		seen.into_iter()
			.filter(|&(i, j)| {
				map[i][j] = true;
				let result = detect_loop(&map, start);
				map[i][j] = false;
				result
			})
			.count()
	}

	fn detect_loop(map: &[Vec<bool>], (mut i, mut j): (usize, usize)) -> bool {
		let (mut di, mut dj) = (-1, 0);
		for _ in 0..map.len() * map[0].len() {
			let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
			if ni >= map.len() || nj >= map[0].len() {
				return false;
			}
			if map[ni][nj] {
				(di, dj) = (dj, -di);
			} else {
				(i, j) = (ni, nj);
			}
		}
		true
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_06_input.txt").unwrap()
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
