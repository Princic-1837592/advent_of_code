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

fn solve_generic<const STEPS: usize, const TARGET: usize>(distances: Parsed) -> usize {
	let mut result = 0;
	let (width, height) = (distances[0].len(), distances.len());
	for (i, row) in distances.iter().enumerate() {
		for (j, &distance) in row.iter().enumerate() {
			if distances[i][j] == usize::MAX {
				continue;
			}
			for di in 1..STEPS {
				for dj in 1..=(STEPS - di) {
					let (neg_i, neg_j) = (i.wrapping_sub(di), j.wrapping_sub(dj));
					let (pos_i, pos_j) = (i + di, j + dj);
					if neg_i < width
						&& neg_j < height && distances[neg_i][neg_j] != usize::MAX
						&& distance.saturating_sub(distances[neg_i][neg_j] + di + dj) >= TARGET
					{
						result += 1;
					}
					if neg_i < width
						&& pos_j < height && distances[neg_i][pos_j] != usize::MAX
						&& distance.saturating_sub(distances[neg_i][pos_j] + di + dj) >= TARGET
					{
						result += 1;
					}
					if pos_i < width
						&& neg_j < height && distances[pos_i][neg_j] != usize::MAX
						&& distance.saturating_sub(distances[pos_i][neg_j] + di + dj) >= TARGET
					{
						result += 1;
					}
					if pos_i < width
						&& pos_j < height && distances[pos_i][pos_j] != usize::MAX
						&& distance.saturating_sub(distances[pos_i][pos_j] + di + dj) >= TARGET
					{
						result += 1;
					}
				}
			}
			for d in 1..=STEPS {
				let (neg_i, neg_j) = (i.wrapping_sub(d), j.wrapping_sub(d));
				let (pos_i, pos_j) = (i + d, j + d);
				if neg_i < width
					&& distances[neg_i][j] != usize::MAX
					&& distance.saturating_sub(distances[neg_i][j] + d) >= TARGET
				{
					result += 1;
				}
				if pos_i < width
					&& distances[pos_i][j] != usize::MAX
					&& distance.saturating_sub(distances[pos_i][j] + d) >= TARGET
				{
					result += 1;
				}
				if neg_j < height
					&& distances[i][neg_j] != usize::MAX
					&& distance.saturating_sub(distances[i][neg_j] + d) >= TARGET
				{
					result += 1;
				}
				if pos_j < height
					&& distances[i][pos_j] != usize::MAX
					&& distance.saturating_sub(distances[i][pos_j] + d) >= TARGET
				{
					result += 1;
				}
			}
		}
	}
	result
}

pub mod part1 {
	use super::{solve_generic, Parsed};

	pub fn solve(distances: Parsed) -> usize {
		solve_generic::<2, 100>(distances)
	}
}

pub mod part2 {
	use super::{solve_generic, Parsed};

	pub fn solve(distances: Parsed) -> usize {
		solve_generic::<20, 100>(distances)
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
