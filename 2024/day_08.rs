//! https://adventofcode.com/2024/day/8
//! https://adventofcode.com/2024/day/8/input

use std::{
	collections::HashMap,
	fs::read_to_string,
	time::{Duration, Instant},
};

type Coord = (usize, usize);

type Parsed = (Coord, HashMap<char, Vec<Coord>>);

fn parse(input: &str) -> Parsed {
	let mut result = HashMap::new();
	for (l, line) in input.lines().enumerate() {
		for (c, char) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
			result.entry(char).or_insert_with(Vec::new).push((l, c));
		}
	}
	(
		(input.lines().count(), input.lines().next().unwrap().len()),
		result,
	)
}

pub mod part1 {
	use std::collections::HashSet;

	use super::Parsed;

	pub fn solve(((h, w), antennas): Parsed) -> usize {
		let mut occupied = HashSet::new();
		for coords in antennas.values() {
			for (i, &(r1, c1)) in coords.iter().enumerate() {
				for &(r2, c2) in coords.iter().skip(i + 1) {
					let dr = r2 - r1;
					let dc = c2 - c1;
					let a1r = r1 - dr;
					let a1c = c1 - dc;
					if a1r < h && a1c < w {
						occupied.insert((a1r, a1c));
					}
					let a2r = r2 + dr;
					let a2c = c2 + dc;
					if a2r < h && a2c < w {
						occupied.insert((a2r, a2c));
					}
				}
			}
		}
		occupied.len()
	}
}

pub mod part2 {
	use std::collections::HashSet;

	use super::Parsed;

	pub fn solve(((h, w), antennas): Parsed) -> usize {
		let mut occupied = HashSet::new();
		for coords in antennas.values() {
			for (i, &(r1, c1)) in coords.iter().enumerate() {
				for &(mut r2, mut c2) in coords.iter().skip(i + 1) {
					let mut r1 = r1;
					let mut c1 = c1;
					let dr = r2 - r1;
					let dc = c2 - c1;
					while r1 < h && c1 < w {
						occupied.insert((r1, c1));
						r1 -= dr;
						c1 -= dc;
					}
					while r2 < h && c2 < w {
						occupied.insert((r2, c2));
						r2 += dr;
						c2 += dc;
					}
				}
			}
		}
		occupied.len()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "............
	........0...
	.....0......
	.......0....
	....0.......
	......A.....
	............
	............
	........A...
	.........A..
	............
	............
	"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_08_input.txt").unwrap()
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
