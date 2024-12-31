//! https://adventofcode.com/2024/day/21
//! https://adventofcode.com/2024/day/21/input

use std::{
	collections::HashMap,
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed<'a> = [[usize; 4]; 5];

fn parse(input: &str) -> Parsed {
	core::array::from_fn(|i| {
		let line = input.lines().nth(i).unwrap();
		core::array::from_fn(|j| {
			let c = line.chars().nth(j).unwrap();
			if c == 'A' {
				10
			} else {
				c as usize - '0' as usize
			}
		})
	})
}

/*
  ^ A
< v >

  0 1
2 3 4
-----------
7 8 9
4 5 6
1 2 3
  0 A
*/

const NUM_PATHS: [[&[&[usize]]; 11]; 11] = [
	[
		&[&[1]],
		&[&[0, 2, 1]],
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[0, 0, 2, 1], &[0, 2, 0, 1]],
		&[&[0, 0, 1]],
		&[&[4, 0, 0, 1], &[0, 0, 4, 1]],
		&[&[0, 0, 0, 2, 1]],
		&[&[0, 0, 0, 1]],
		&[&[4, 0, 0, 0, 1], &[0, 0, 0, 4, 1]],
		&[&[4, 1]],
	],
	[
		&[&[4, 3, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[4, 4, 1]],
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[4, 4, 0, 1], &[0, 4, 4, 1]],
		&[&[0, 0, 1]],
		&[&[4, 0, 0, 1], &[0, 0, 4, 1]],
		&[&[4, 4, 0, 0, 1], &[0, 0, 4, 4, 1]],
		&[&[4, 4, 3, 1]],
	],
	[
		&[&[3, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[0, 0, 2, 1], &[2, 0, 0, 1]],
		&[&[0, 0, 1]],
		&[&[4, 0, 0, 1], &[0, 0, 4, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
	],
	[
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[2, 2, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[0, 2, 2, 1], &[2, 2, 0, 1]],
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[0, 0, 2, 2, 1], &[2, 2, 0, 0, 1]],
		&[&[0, 0, 2, 1], &[2, 0, 0, 1]],
		&[&[0, 0, 1]],
		&[&[3, 1]],
	],
	[
		&[&[3, 4, 3, 1], &[4, 3, 3, 1]],
		&[&[3, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
		&[&[3, 4, 4, 1], &[4, 3, 4, 1], &[4, 4, 3, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[4, 4, 1]],
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[4, 4, 0, 1], &[4, 0, 4, 1], &[0, 4, 4, 1]],
		&[&[4, 4, 3, 3, 1]],
	],
	[
		&[&[3, 3, 1]],
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[3, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[3, 3, 4, 1], &[4, 3, 3, 1]],
	],
	[
		&[&[3, 3, 2, 1], &[2, 3, 3, 1]],
		&[&[3, 2, 2, 1], &[2, 2, 3, 1]],
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[3, 1]],
		&[&[2, 2, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[0, 2, 2, 1], &[2, 2, 0, 1]],
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[3, 3, 1]],
	],
	[
		&[&[3, 3, 4, 3, 1], &[4, 3, 3, 3, 1]],
		&[&[3, 3, 1]],
		&[&[3, 3, 4, 1], &[4, 3, 3, 1]],
		&[&[3, 3, 4, 4, 1], &[4, 4, 3, 3, 1]],
		&[&[3, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
		&[&[3, 4, 4, 1], &[4, 4, 3, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[4, 4, 1]],
		&[&[4, 4, 3, 3, 3, 1]],
	],
	[
		&[&[3, 3, 3, 1]],
		&[&[3, 3, 2, 1], &[2, 3, 3, 1]],
		&[&[3, 3, 1]],
		&[&[3, 3, 4, 1], &[4, 3, 3, 1]],
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[3, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[3, 3, 3, 4, 1], &[4, 3, 3, 3, 1]],
	],
	[
		&[&[3, 3, 3, 2, 1], &[2, 3, 3, 3, 1]],
		&[&[3, 3, 2, 2, 1], &[2, 2, 3, 3, 1]],
		&[&[3, 3, 2, 1], &[2, 3, 3, 1]],
		&[&[3, 3, 1]],
		&[&[3, 2, 2, 1], &[2, 2, 3, 1]],
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[3, 1]],
		&[&[2, 2, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[3, 3, 3, 1]],
	],
	[
		&[&[2, 1]],
		&[&[0, 2, 2, 1], &[2, 0, 2, 1]],
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[0, 0, 2, 2, 1]],
		&[&[0, 0, 2, 1], &[2, 0, 0, 1]],
		&[&[0, 0, 1]],
		&[&[0, 0, 0, 2, 2, 1]],
		&[&[0, 0, 0, 2, 1], &[2, 0, 0, 0, 1]],
		&[&[0, 0, 0, 1]],
		&[&[1]],
	],
];

const ARROW_PATHS: [[&[&[usize]]; 5]; 5] = [
	[
		&[&[1]],
		&[&[4, 1]],
		&[&[3, 2, 1]],
		&[&[3, 1]],
		&[&[3, 4, 1], &[4, 3, 1]],
	],
	[
		&[&[2, 1]],
		&[&[1]],
		&[&[3, 2, 2, 1]],
		&[&[3, 2, 1], &[2, 3, 1]],
		&[&[3, 1]],
	],
	[
		&[&[4, 0, 1]],
		&[&[4, 4, 0, 1]],
		&[&[1]],
		&[&[4, 1]],
		&[&[4, 4, 1]],
	],
	[
		&[&[0, 1]],
		&[&[4, 0, 1], &[0, 4, 1]],
		&[&[2, 1]],
		&[&[1]],
		&[&[4, 1]],
	],
	[
		&[&[0, 2, 1], &[2, 0, 1]],
		&[&[0, 1]],
		&[&[2, 2, 1]],
		&[&[2, 1]],
		&[&[1]],
	],
];

fn find_shortest(
	path: &[usize],
	robots_left: u8,
	cache: &mut HashMap<(Vec<usize>, u8), u64>,
) -> u64 {
	if robots_left == 0 {
		return path.len() as u64;
	}
	if let Some(&result) = cache.get(&(path.to_vec(), robots_left)) {
		return result;
	}
	let mut result = 0;
	let mut button = 1;
	for &next_button in path {
		result += ARROW_PATHS[button][next_button]
			.iter()
			.map(|path| find_shortest(path, robots_left - 1, cache))
			.min()
			.unwrap();
		button = next_button;
	}
	cache.insert((path.to_vec(), robots_left), result);
	result
}

pub fn solve_generic<const R: u8>(codes: Parsed) -> u64 {
	let mut result = 0;
	let mut cache = HashMap::new();
	for code in codes {
		let mut num = 0;
		for &n in &code[..3] {
			num *= 10;
			num += n;
		}
		let mut length = 0;
		let mut button = 10;
		for next_button in code {
			length += NUM_PATHS[button][next_button]
				.iter()
				.map(|path| find_shortest(path, R, &mut cache))
				.min()
				.unwrap();
			button = next_button;
		}
		result += length * num as u64;
	}
	result
}

pub mod part1 {
	use super::{solve_generic, Parsed};

	pub fn solve(codes: Parsed) -> u64 {
		solve_generic::<2>(codes)
	}
}

pub mod part2 {
	use super::{solve_generic, Parsed};

	pub fn solve(codes: Parsed) -> u64 {
		solve_generic::<25>(codes)
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "029A
980A
179A
456A
379A
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_21_input.txt").unwrap()
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
	let result = part1::solve(parsed);
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
