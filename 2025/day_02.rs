//! https://adventofcode.com/2025/day/2
//! https://adventofcode.com/2025/day/2/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
pub struct Range {
	start: usize,
	end: usize,
}

impl FromStr for Range {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split('-');
		Ok(Self {
			start: parts.next().unwrap().parse().unwrap(),
			end: parts.next().unwrap().parse().unwrap(),
		})
	}
}

type Parsed = Vec<Range>;

fn parse(input: &str) -> Parsed {
	input.split(',').flat_map(Range::from_str).collect()
}

pub mod part1 {
	use super::{Parsed, Range};

	pub fn solve(parsed: Parsed) -> usize {
		let mut total = 0;
		for Range { mut start, mut end } in parsed {
			let mut start_digits = start.ilog10() + 1;
			if start_digits % 2 == 1 {
				start = 10_usize.pow(start_digits);
				start_digits += 1;
			}
			let mut end_digits = end.ilog10() + 1;
			if end_digits % 2 == 1 {
				end = 10_usize.pow(end_digits - 1) - 1;
				end_digits -= 1;
			}
			let start_split = 10_usize.pow(start_digits / 2);
			let (mut a, b) = (start / start_split, start % start_split);
			let end_split = 10_usize.pow(end_digits / 2);
			let (c, d) = (end / end_split, end % end_split);
			if a >= b && c * end_split + d >= a * start_split + a {
				total += a * start_split + a;
			}
			a += 1;
			while a <= c {
				if c * end_split + d >= a * start_split + a {
					total += a * start_split + a;
				}
				a += 1;
			}
		}
		total
	}
}

pub mod part2 {
	use super::{Parsed, Range};

	pub fn solve(parsed: Parsed) -> usize {
		let mut total = 0;
		for Range { start, end } in parsed {
			for id in start..=end {
				let digits = id.ilog10() + 1;
				if digits == 4 {
					if id / 100 == id % 100 {
						total += id;
					}
				} else if digits == 6 {
					// abc def
					// ab cd ef
					let a = id / 10_000;
					let b = id / 100 % 100;
					let c = id % 100;
					if id / 1_000 == id % 1_000 || a == b && b == c {
						total += id;
					}
				} else if digits == 8 {
					// abcdefgh
					if id / 10_000 == id % 10_000 {
						total += id;
					}
				} else if digits == 9 {
					// abc def ghi
					let a = id / 1_000_000;
					let b = id / 1_000 % 1_000;
					let c = id % 1000;
					if a == b && b == c {
						total += id;
					}
				} else if digits == 10 {
					// abcde  fghij
					// ab cd ef gh ij
					let a = id / 100_000_000;
					let b = id / 1_000_000 % 100;
					let c = id / 10_000 % 100;
					let d = id / 100 % 100;
					let e = id % 100;
					if id / 100_000 == id % 100_000 || a == b && b == c && c == d && d == e {
						total += id;
					}
				} else if digits >= 2 {
					let str = id.to_string();
					let first = str.chars().next().unwrap();
					let mut valid = true;
					for i in 1..str.len() {
						if str.chars().nth(i).unwrap() != first {
							valid = false;
							break;
						}
					}
					if valid {
						total += id;
					}
				}
			}
		}
		total
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
		.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_02_input.txt").unwrap()
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
