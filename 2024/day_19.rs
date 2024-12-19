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
			.into_par_iter()
			.filter(|design| {
				let mut queue = BinaryHeap::from([0]);
				let mut seen = vec![false; design.len() + 1];
				while let Some(matched) = queue.pop() {
					let next_char = design[matched];
					if seen[matched] {
						continue;
					}
					seen[matched] = true;
					for pattern in &patterns[next_char] {
						if design[matched..].starts_with(pattern) {
							if matched + pattern.len() == design.len() {
								return true;
							} else {
								queue.push(matched + pattern.len());
							}
						}
					}
				}
				false
			})
			.count()
	}
}

pub mod part2 {
	use rayon::iter::{IntoParallelIterator, ParallelIterator};

	use super::Parsed;

	pub fn solve((patterns, designs): Parsed) -> usize {
		let patterns: Vec<_> = patterns.into_iter().flatten().collect();
		designs
			.into_par_iter()
			.map(|design| {
				let mut pd = vec![0; design.len() + 1];
				pd[0] = 1;
				for i in 1..pd.len() {
					for pattern in patterns.iter().filter(|p| p.len() <= i) {
						if design[i - pattern.len()..i] == *pattern {
							pd[i] += pd[i - pattern.len()];
						}
					}
				}
				*pd.last().unwrap()
			})
			.sum()
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
