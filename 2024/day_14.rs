//! https://adventofcode.com/2024/day/14
//! https://adventofcode.com/2024/day/14/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

type Parsed = Vec<Robot>;

#[derive(Copy, Clone, Debug)]
pub struct Robot {
	px: isize,
	py: isize,
	vx: isize,
	vy: isize,
}

impl FromStr for Robot {
	type Err = ();

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let mut parts = value.split(['=', ' ', ',']);
		Ok(Robot {
			px: parts.nth(1).unwrap().parse().unwrap(),
			py: parts.next().unwrap().parse().unwrap(),
			vx: parts.nth(1).unwrap().parse().unwrap(),
			vy: parts.next().unwrap().parse().unwrap(),
		})
	}
}

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

	use super::{Parsed, Robot};

	pub fn solve(mut robots: Parsed) -> usize {
		const W: isize = 101;
		const H: isize = 103;
		const QW: isize = W / 2 - 1;
		const QW1: isize = W / 2 + 1;
		const QH: isize = H / 2 - 1;
		const QH1: isize = H / 2 + 1;
		robots.par_iter_mut().for_each(|r| {
			r.px = ((r.px + r.vx * 100) % W + W) % W;
			r.py = ((r.py + r.vy * 100) % H + H) % H;
		});
		let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
		for Robot { px, py, .. } in robots {
			match (px, py) {
				(0..=QW, 0..=QH) => q1 += 1,
				(0..=QW, QH1..) => q2 += 1,
				(QW1.., 0..=QH) => q3 += 1,
				(QW1.., QH1..) => q4 += 1,
				_ => {}
			}
		}
		q1 * q2 * q3 * q4
	}
}

pub mod part2 {
	use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve(mut robots: Parsed) -> usize {
		const W: isize = 101;
		const H: isize = 103;
		let mut map = vec![vec!['.'; W as usize + 1]; H as usize + 1];
		for i in 1..10_000 {
			map.par_iter_mut().for_each(|r| r.fill('.'));
			for r in &mut robots {
				r.px = ((r.px + r.vx) % W + W) % W;
				r.py = ((r.py + r.vy) % H + H) % H;
				map[r.py as usize][r.px as usize] = '#';
			}
			if map
				.par_iter()
				.filter(|r| r.iter().filter(|&&c| c == '#').count() >= 31)
				.count() >= 2
			{
				return i;
			}
		}
		unreachable!()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_14_input.txt").unwrap()
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
