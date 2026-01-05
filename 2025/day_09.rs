//! https://adventofcode.com/2025/day/9
//! https://adventofcode.com/2025/day/9/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

type Parsed = Vec<Point>;

#[derive(Copy, Clone, Debug)]
pub struct Point {
	x: isize,
	y: isize,
}

impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(',').unwrap();
		Ok(Self {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
		})
	}
}

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use super::{Parsed, Point};

	pub fn solve(points: Parsed) -> usize {
		let mut max = 0;
		for (i, &Point { x: xi, y: yi }) in points.iter().enumerate() {
			for &Point { x: xj, y: yj } in &points[i + 1..] {
				let area = (xi.abs_diff(xj) + 1) * (yi.abs_diff(yj) + 1);
				max = max.max(area);
			}
		}
		max
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_09_input.txt").unwrap()
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
