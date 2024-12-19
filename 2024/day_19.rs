//! https://adventofcode.com/2024/day/19
//! https://adventofcode.com/2024/day/19/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

type Parsed = ([Vec<Vec<usize>>; 26], Vec<Vec<usize>>);

fn parse(input: &str) -> Parsed {
	let sep = LINE_ENDING.repeat(2);
	let mut parts = input.split(&sep);
	let patterns = parts.next().unwrap();
	let designs = parts
		.next()
		.unwrap()
		.lines()
		.map(|l| l.chars().map(|c| c as usize - 'a' as usize).collect())
		.collect();
	let mut patterns_array = core::array::from_fn(|_| vec![]);
	for pattern in patterns.split(", ").map(|p| p.to_string()) {
		patterns_array[pattern.chars().next().unwrap() as usize - 'a' as usize]
			.push(pattern.chars().map(|c| c as usize - 'a' as usize).collect());
	}
	(patterns_array, designs)
}

pub mod part1 {
	use std::collections::BinaryHeap;

	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve((patterns, designs): Parsed) -> usize {
		designs
			// .into_iter()
			// .enumerate()
			.into_par_iter()
			.filter(|design| {
				let mut queue = BinaryHeap::from([0]);
				while let Some(matched) = queue.pop() {
					let next_char = design[matched];
					for pattern in &patterns[next_char] {
						if design[matched..].starts_with(pattern) {
							if matched + pattern.len() == design.len() {
								// dbg!(i);
								return true;
							} else {
								queue.push(matched + pattern.len());
							}
						}
					}
				}
				dbg!(false)
			})
			.count()
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "r, wr, b, g, bwu, rb, gb, br\r
\r
brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_19_input.txt").unwrap()
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
