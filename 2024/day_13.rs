//! https://adventofcode.com/2024/day/13
//! https://adventofcode.com/2024/day/13/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

#[derive(Copy, Clone, Debug)]
pub struct Machine {
	ax: i64,
	ay: i64,
	bx: i64,
	by: i64,
	px: i64,
	py: i64,
}

impl From<&str> for Machine {
	fn from(value: &str) -> Self {
		let mut parts = value.lines();
		let mut a = parts.next().unwrap().split([',', '+']);
		let mut b = parts.next().unwrap().split([',', '+']);
		let mut p = parts.next().unwrap().split([',', '=']);
		Machine {
			ax: a.nth(1).unwrap().parse().unwrap(),
			ay: a.last().unwrap().parse().unwrap(),
			bx: b.nth(1).unwrap().parse().unwrap(),
			by: b.last().unwrap().parse().unwrap(),
			px: p.nth(1).unwrap().parse().unwrap(),
			py: p.last().unwrap().parse().unwrap(),
		}
	}
}

type Parsed = Vec<Machine>;

fn parse(input: &str) -> Parsed {
	let sep = LINE_ENDING.repeat(2);
	input.split(&sep).map(Machine::from).collect()
}

pub mod part1 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve(machines: Parsed) -> i64 {
		machines
			.into_par_iter()
			.map(|machine| {
				let a = (machine.px * machine.by - machine.py * machine.bx)
					/ (machine.ax * machine.by - machine.ay * machine.bx);
				let b = (machine.ax * machine.py - machine.ay * machine.px)
					/ (machine.ax * machine.by - machine.ay * machine.bx);
				if machine.ax * a + machine.bx * b == machine.px
					&& machine.ay * a + machine.by * b == machine.py
				{
					3 * a + b
				} else {
					0
				}
			})
			.sum()
	}
}

pub mod part2 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve(machines: Parsed) -> i64 {
		machines
			.into_par_iter()
			.map(|mut machine| {
				machine.px += 10000000000000;
				machine.py += 10000000000000;
				let a = (machine.px * machine.by - machine.py * machine.bx)
					/ (machine.ax * machine.by - machine.ay * machine.bx);
				let b = (machine.ax * machine.py - machine.ay * machine.px)
					/ (machine.ax * machine.by - machine.ay * machine.bx);
				if machine.ax * a + machine.bx * b == machine.px
					&& machine.ay * a + machine.by * b == machine.py
				{
					3 * a + b
				} else {
					0
				}
			})
			.sum()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_13_input.txt").unwrap()
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
