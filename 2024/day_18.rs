//! https://adventofcode.com/2024/day/18
//! https://adventofcode.com/2024/day/18/input

use std::{
	collections::VecDeque,
	fs::read_to_string,
	time::{Duration, Instant},
};

type Coord = (usize, usize);
type Parsed = Vec<Coord>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|l| {
			let mut parts = l.split(',');
			(
				parts.next().unwrap().parse().unwrap(),
				parts.next().unwrap().parse().unwrap(),
			)
		})
		.collect()
}

fn can_pass<const SIZE: usize>(unsafe_memory: &[Vec<bool>]) -> Option<usize> {
	let mut seen = vec![vec![false; SIZE + 1]; SIZE + 1];
	let mut queue = VecDeque::from([((0, 0), 0)]);
	while let Some((pos @ (i, j), steps)) = queue.pop_front() {
		if pos == (SIZE, SIZE) {
			return Some(steps);
		}
		if i > SIZE || j > SIZE || seen[i][j] || unsafe_memory[i][j] {
			continue;
		}
		seen[i][j] = true;
		for (ni, nj) in [
			Some((i + 1, j)),
			Some((i, j + 1)),
			i.checked_sub(1).map(|ni| (ni, j)),
			j.checked_sub(1).map(|nj| (i, nj)),
		]
		.into_iter()
		.flatten()
		{
			queue.push_back(((ni, nj), steps + 1));
		}
	}
	None
}

pub mod part1 {
	use super::{can_pass, Parsed};

	pub fn solve<const SIZE: usize>(bytes: Parsed) -> usize {
		let mut unsafe_memory = vec![vec![false; SIZE + 1]; SIZE + 1];
		for (i, j) in bytes.into_iter().take(1024) {
			unsafe_memory[i][j] = true;
		}
		can_pass::<SIZE>(&unsafe_memory).unwrap()
	}
}

pub mod part2 {
	use super::{can_pass, Parsed};

	pub fn solve<const SIZE: usize>(bytes: Parsed) -> String {
		let mut unsafe_memory = vec![vec![false; SIZE + 1]; SIZE + 1];
		for &(i, j) in bytes.iter().take(1024) {
			unsafe_memory[i][j] = true;
		}
		for (i, j) in bytes.into_iter().skip(1024) {
			unsafe_memory[i][j] = true;
			if can_pass::<SIZE>(&unsafe_memory).is_none() {
				return format!("{},{}", i, j);
			}
		}
		unreachable!()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_18_input.txt").unwrap()
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
	let result = part1::solve::<70>(parsed.clone());
	let elapsed = start.elapsed();
	println!("{}", result);
	println!("First part in {:?}", elapsed);
	total += elapsed;

	let start = Instant::now();
	let result = part2::solve::<70>(parsed);
	let elapsed = start.elapsed();
	println!("{}", result);
	println!("Second part in {:?}", elapsed);
	total += elapsed;

	if verbose {
		println!("Total {:?}", total);
	}
	total
}
