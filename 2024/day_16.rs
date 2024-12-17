//! https://adventofcode.com/2024/day/16
//! https://adventofcode.com/2024/day/16/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

type Coord = (usize, usize);
type Parsed = Vec<Vec<bool>>;

fn parse(input: &str) -> Parsed {
	input
		.lines()
		.map(|l| l.chars().map(|c| c == '#').collect())
		.collect()
}

pub mod part1 {
	use std::collections::BinaryHeap;

	use utils::{coords::Direction, IntoEnumIterator};

	use super::{Coord, Parsed};

	#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
	struct State {
		points: isize,
		position: Coord,
		direction: Direction,
	}

	pub fn solve(walls: Parsed) -> isize {
		let target = (1_usize, walls[0].len() - 2);
		let start = (walls.len() - 2, 1);
		let mut heap = BinaryHeap::from([State {
			points: 0,
			position: start,
			direction: Direction::E,
		}]);
		let mut seen = vec![vec![[false, false, false, false]; walls[0].len()]; walls.len()];
		while let Some(State {
			points,
			position: position @ (i, j),
			direction,
		}) = heap.pop()
		{
			if position == target {
				return -points;
			}
			if walls[i][j] {
				continue;
			}
			if seen[i][j][direction as usize] {
				continue;
			}
			seen[i][j][direction as usize] = true;
			for dir in Direction::iter() {
				if dir == direction {
					heap.push(State {
						points: points - 1,
						position: match direction {
							Direction::N => (i - 1, j),
							Direction::E => (i, j + 1),
							Direction::S => (i + 1, j),
							Direction::W => (i, j - 1),
						},
						direction,
					})
				} else {
					heap.push(State {
						points: points - 1000,
						position,
						direction: dir,
					})
				}
			}
		}
		unreachable!()
	}
}

pub mod part2 {
	use std::collections::{HashSet, VecDeque};

	use utils::{coords::Direction, IntoEnumIterator};

	use super::{Coord, Parsed};

	#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
	struct State {
		points: isize,
		position: Coord,
		direction: Direction,
		path: Vec<Coord>,
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	struct MinPath {
		points: isize,
		paths: HashSet<Coord>,
	}

	pub fn solve(walls: Parsed) -> usize {
		let (ti, tj) = (1_usize, walls[0].len() - 2);
		let start = (walls.len() - 2, 1);
		let mut queue = VecDeque::from([State {
			points: 0,
			position: start,
			direction: Direction::E,
			path: vec![start],
		}]);
		let mut min_path = vec![
			vec![
				[
					MinPath {
						points: isize::MIN,
						paths: HashSet::new(),
					},
					MinPath {
						points: isize::MIN,
						paths: HashSet::new(),
					},
					MinPath {
						points: isize::MIN,
						paths: HashSet::new(),
					},
					MinPath {
						points: isize::MIN,
						paths: HashSet::new(),
					}
				];
				walls[0].len()
			];
			walls.len()
		];
		while let Some(State {
			points,
			position: position @ (i, j),
			direction,
			path,
		}) = queue.pop_front()
		{
			if walls[i][j] {
				continue;
			}
			if points < min_path[i][j][direction as usize].points {
				continue;
			}
			if points > min_path[i][j][direction as usize].points {
				min_path[i][j][direction as usize].paths.clear();
				min_path[i][j][direction as usize].points = points;
			}
			min_path[i][j][direction as usize]
				.paths
				.extend(path.clone());
			for dir in Direction::iter() {
				if dir == direction {
					let mut path = path.clone();
					let position = match direction {
						Direction::N => (position.0 - 1, position.1),
						Direction::E => (position.0, position.1 + 1),
						Direction::S => (position.0 + 1, position.1),
						Direction::W => (position.0, position.1 - 1),
					};
					path.push(position);
					queue.push_back(State {
						points: points - 1,
						position,
						direction,
						path,
					})
				} else {
					queue.push_back(State {
						points: points - 1000,
						position,
						direction: dir,
						path: path.clone(),
					})
				}
			}
		}
		let mut result: HashSet<Coord> = HashSet::new();
		for d in 0..4 {
			result.extend(&min_path[ti][tj][d].paths);
		}
		min_path[ti][tj][0].paths.len()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_16_input.txt").unwrap()
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
