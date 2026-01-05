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
	use super::{Parsed, Point};

	#[derive(Copy, Clone, Debug)]
	struct Segment {
		a: Point,
		b: Point,
	}

	pub fn solve(mut points: Parsed) -> usize {
		let mut verticals = Vec::with_capacity(points.len() / 2);
		let mut horizontals = Vec::with_capacity(points.len() / 2);
		let mut min_x = usize::MAX;
		let mut min_y = usize::MAX;
		let mut current = points[0];
		points.push(points[0]);
		for &next in &points[1..] {
			let segment = Segment {
				a: current,
				b: next,
			};
			if current.x == next.x {
				min_y = min_y.min(current.y.abs_diff(next.y));
				verticals.push(segment);
			} else {
				min_x = min_x.min(current.x.abs_diff(next.x));
				horizontals.push(segment);
			}
			current = next;
		}
		points.pop();
		let mut max = 0;
		for (i, &Point { x: xi, y: yi }) in points.iter().enumerate() {
			'next_point: for &Point { x: xj, y: yj } in &points[i + 1..] {
				let x1 = xi.min(xj);
				let x2 = xi.max(xj);
				let y1 = yi.min(yj);
				let y2 = yi.max(yj);
				for &Segment {
					a: Point { x: sx, y: sy1 },
					b: Point { y: sy2, .. },
				} in &verticals
				{
					let (sy1, sy2) = (sy1.min(sy2), sy1.max(sy2));
					if (x1 < sx && sx < x2) && (sy1 < y1 && y1 < sy2) {
						continue 'next_point;
					}
					if (x1 < sx && sx < x2) && (sy1 < y2 && y2 < sy2) {
						continue 'next_point;
					}
					if (x1 < sx && sx < x2) && (y1 < sy1 && sy2 < y2) {
						continue 'next_point;
					}
				}
				for &Segment {
					a: Point { x: sx1, y: sy },
					b: Point { x: sx2, .. },
				} in &horizontals
				{
					let (sx1, sx2) = (sx1.min(sx2), sx1.max(sx2));
					if (y1 < sy && sy < y2) && (sx1.min(sx2) < x1 && x1 < sx1.max(sx2)) {
						continue 'next_point;
					}
					if (y1 < sy && sy < y2) && (sx1.min(sx2) < x2 && x2 < sx1.max(sx2)) {
						continue 'next_point;
					}
					if (y1 < sy && sy < y2) && (x1 < sx1 && sx2 < x2) {
						continue 'next_point;
					}
				}
				let area = (xi.abs_diff(xj) + 1) * (yi.abs_diff(yj) + 1);
				if area > max {
					max = area;
				}
			}
		}
		max
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
