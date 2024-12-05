//! https://adventofcode.com/2024/day/4
//! https://adventofcode.com/2024/day/4/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::parsing::parse_matrix;

type Parsed = Vec<Vec<char>>;

fn parse(input: &str) -> Parsed {
	parse_matrix(input)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(matrix: Parsed) -> usize {
		let mut result = 0;
		for word in [['X', 'M', 'A', 'S'], ['S', 'A', 'M', 'X']] {
			for r in 0..matrix.len() {
				for c in 0..matrix[r].len() {
					if matrix[r][c] == word[0] {
						if c + 3 < matrix[r].len()
							&& matrix[r][c + 1] == word[1]
							&& matrix[r][c + 2] == word[2]
							&& matrix[r][c + 3] == word[3]
						{
							result += 1;
						}
						if r + 3 < matrix.len()
							&& matrix[r + 1][c] == word[1]
							&& matrix[r + 2][c] == word[2]
							&& matrix[r + 3][c] == word[3]
						{
							result += 1;
						}
						if r + 3 < matrix.len()
							&& c + 3 < matrix[r].len() && matrix[r + 1][c + 1] == word[1]
							&& matrix[r + 2][c + 2] == word[2]
							&& matrix[r + 3][c + 3] == word[3]
						{
							result += 1;
						}
						if r >= 3
							&& c + 3 < matrix[r].len() && matrix[r - 1][c + 1] == word[1]
							&& matrix[r - 2][c + 2] == word[2]
							&& matrix[r - 3][c + 3] == word[3]
						{
							result += 1;
						}
					}
				}
			}
		}
		result
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(matrix: Parsed) -> usize {
		let mut result = 0;
		for r in 1..matrix.len() - 1 {
			for c in 1..matrix[r].len() - 1 {
				if matrix[r][c] == 'A'
					&& (matrix[r - 1][c - 1] == 'M' && matrix[r + 1][c + 1] == 'S'
						|| matrix[r - 1][c - 1] == 'S' && matrix[r + 1][c + 1] == 'M')
					&& (matrix[r - 1][c + 1] == 'M' && matrix[r + 1][c - 1] == 'S'
						|| matrix[r - 1][c + 1] == 'S' && matrix[r + 1][c - 1] == 'M')
				{
					result += 1
				}
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "".to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_04_input.txt").unwrap()
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
