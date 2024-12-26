//! https://adventofcode.com/2024/day/17
//! https://adventofcode.com/2024/day/17/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = (u64, u64, u64, Vec<u64>);

fn parse(input: &str) -> Parsed {
	let mut lines = input.lines();
	let a = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse()
		.unwrap();
	let b = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse()
		.unwrap();
	let c = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse()
		.unwrap();
	(
		a,
		b,
		c,
		lines
			.nth(1)
			.unwrap()
			.split_whitespace()
			.last()
			.unwrap()
			.split(',')
			.map(|c| c.parse().unwrap())
			.collect(),
	)
}

fn combo(a: u64, b: u64, c: u64, instructions: &[u64], pc: usize) -> u64 {
	match instructions[pc + 1] {
		res @ 0..=3 => res,
		4 => a,
		5 => b,
		6 => c,
		_ => unreachable!(),
	}
}

pub mod part1 {
	use super::{combo, Parsed};

	pub fn solve((mut a, mut b, mut c, instructions): Parsed) -> String {
		let mut pc = 0;
		let mut outputs = vec![];
		while pc < instructions.len() {
			let mut jumped = false;
			match instructions[pc] {
				0 => a = a / (1 << combo(a, b, c, &instructions, pc)),
				1 => b ^= instructions[pc + 1],
				2 => b = combo(a, b, c, &instructions, pc) % 8,
				3 => {
					if a != 0 {
						pc = instructions[pc + 1] as usize;
						jumped = true;
					}
				}
				4 => b ^= c,
				5 => {
					outputs.push(combo(a, b, c, &instructions, pc) % 8);
				}
				6 => b = a / (1 << combo(a, b, c, &instructions, pc)),
				7 => c = a / (1 << combo(a, b, c, &instructions, pc)),
				_ => unreachable!(),
			}
			if !jumped {
				pc += 2;
			}
		}
		outputs
			.into_iter()
			.map(|v| v.to_string())
			.collect::<Vec<_>>()
			.join(",")
	}
}

pub mod part2 {
	use super::{combo, Parsed};

	pub fn solve((_a, _b, _c, instructions): Parsed) -> u64 {
		for digit in 0o0..=0o7 {
			if let Some(result) = execute(&instructions, 1, digit) {
				return result;
			}
		}
		unreachable!()
	}

	fn execute(instructions: &[u64], to_match: usize, start_a: u64) -> Option<u64> {
		if to_match > instructions.len() {
			return Some(start_a >> 3);
		}
		let mut matched = 0;
		let (mut a, mut b, mut c) = (start_a, 0, 0);
		let mut pc = 0;
		while pc < instructions.len() {
			let mut jumped = false;
			match instructions[pc] {
				0 => a = a / (1 << combo(a, b, c, instructions, pc)),
				1 => b ^= instructions[pc + 1],
				2 => b = combo(a, b, c, instructions, pc) % 8,
				3 => {
					if a != 0 {
						pc = instructions[pc + 1] as usize;
						jumped = true;
					}
				}
				4 => b ^= c,
				5 => {
					if combo(a, b, c, instructions, pc) % 8
						!= instructions[instructions.len() - to_match + matched]
					{
						return None;
					}
					matched += 1;
					if matched == to_match {
						for digit in 0o0..=0o7 {
							if let Some(result) =
								execute(instructions, to_match + 1, (start_a << 3) | digit)
							{
								return Some(result);
							}
						}
						return None;
					}
				}
				6 => b = a / (1 << combo(a, b, c, instructions, pc)),
				7 => c = a / (1 << combo(a, b, c, instructions, pc)),
				_ => unreachable!(),
			}
			if !jumped {
				pc += 2;
			}
		}
		None
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_17_input.txt").unwrap()
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
