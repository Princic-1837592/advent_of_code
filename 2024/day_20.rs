//! https://adventofcode.com/2024/day/20
//! https://adventofcode.com/2024/day/20/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::coords::u_iter_cross_near;

type Coord = (usize, usize);
type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
	let walls: Vec<Vec<_>> = input
		.lines()
		.map(|l| l.chars().map(|c| c == '#').collect())
		.collect();
	let mut start = (usize::MAX, usize::MAX);
	let mut end = (usize::MAX, usize::MAX);
	for (i, row) in input.lines().enumerate() {
		for (j, c) in row.chars().enumerate() {
			if c == 'S' {
				start = (i, j);
			}
			if c == 'E' {
				end = (i, j);
			}
		}
	}
	find_distances(walls, start, end)
}

fn find_distances(walls: Vec<Vec<bool>>, start: Coord, end: Coord) -> Vec<Vec<usize>> {
	let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; walls[0].len()]; walls.len()];
	let mut pos = end;
	let mut dist = 0;
	'pos: while pos != start {
		let (i, j) = pos;
		distances[pos.0][pos.1] = dist;
		dist += 1;
		for (ni, nj) in u_iter_cross_near(i, j, walls[0].len(), walls.len()) {
			if !walls[ni][nj] && distances[ni][nj] == usize::MAX {
				pos = (ni, nj);
				continue 'pos;
			}
		}
	}
	distances[start.0][start.1] = dist;
	distances
}

pub mod part1 {
	use utils::coords::u_iter_near;

	use super::Parsed;

	pub fn solve(distances: Parsed) -> usize {
		let mut result = 0;
		let (width, height) = (distances[0].len(), distances.len());
		for (i, row) in distances.iter().enumerate() {
			for (j, &distance) in row.iter().enumerate() {
				if distances[i][j] == usize::MAX {
					continue;
				}
				for (ni, nj) in u_iter_near(i, j, width, height).chain(
					[
						(i + 2 < width).then(|| (i + 2, j)),
						(j + 2 < height).then(|| (i, j + 2)),
						(i >= 2).then(|| (i - 2, j)),
						(j >= 2).then(|| (i, j - 2)),
					]
					.into_iter()
					.flatten(),
				) {
					if distances[ni][nj] != usize::MAX
						&& distances[ni][nj] > distance
						&& distances[ni][nj] - distance > 100
					{
						result += 1;
					}
				}
			}
		}
		result
	}
}

pub mod part2 {
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_20_input.txt").unwrap()
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
