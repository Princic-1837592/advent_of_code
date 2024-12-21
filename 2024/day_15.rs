//! https://adventofcode.com/2024/day/15
//! https://adventofcode.com/2024/day/15/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::{from_char, parsing::parse_matrix};

use crate::LINE_ENDING;

type Coord = (usize, usize);
type Map = Vec<Vec<Cell>>;
type Directions = Vec<char>;
type Parsed = (Map, Directions, Coord);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[from_char]
pub enum Cell {
	Wall = '#',
	Empty = '.',
	Box = 'O',
	Robot = '@',
	Left = '[',
	Right = ']',
}

fn parse(input: &str) -> Parsed {
	let sep = LINE_ENDING.repeat(2);
	let mut parts = input.split(&sep);
	let map = parse_matrix(parts.next().unwrap());
	let directions = parts
		.next()
		.unwrap()
		.split_whitespace()
		.flat_map(&str::chars)
		.collect();
	for (i, row) in map.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			if let Cell::Robot = cell {
				return (map, directions, (i, j));
			}
		}
	}
	unreachable!()
}

pub mod part1 {
	use super::{Cell, Coord, Map, Parsed};

	fn can_move_in_direction(map: &Map, (mut i, mut j): Coord, direction: char) -> Option<Coord> {
		let (di, dj) = match direction {
			'<' => (0, usize::MAX),
			'^' => (usize::MAX, 0),
			'>' => (0, 1),
			_v => (1, 0),
		};
		i += di;
		j += dj;
		while map[i][j] == Cell::Box {
			i += di;
			j += dj;
		}
		(map[i][j] == Cell::Empty).then_some((i, j))
	}

	pub fn solve((mut map, directions, (mut i, mut j)): Parsed) -> usize {
		map[i][j] = Cell::Empty;
		for direction in directions {
			match direction {
				'<' => {
					if map[i][j - 1] == Cell::Empty {
						j -= 1;
					} else if let Some((di, dj)) = can_move_in_direction(&map, (i, j), direction) {
						map[di][dj] = Cell::Box;
						map[i][j - 1] = Cell::Empty;
						j -= 1;
					}
				}
				'^' => {
					if map[i - 1][j] == Cell::Empty {
						i -= 1;
					} else if let Some((di, dj)) = can_move_in_direction(&map, (i, j), direction) {
						map[di][dj] = Cell::Box;
						map[i - 1][j] = Cell::Empty;
						i -= 1;
					}
				}
				'>' => {
					if map[i][j + 1] == Cell::Empty {
						j += 1;
					} else if let Some((di, dj)) = can_move_in_direction(&map, (i, j), direction) {
						map[di][dj] = Cell::Box;
						map[i][j + 1] = Cell::Empty;
						j += 1;
					}
				}
				_v => {
					if map[i + 1][j] == Cell::Empty {
						i += 1;
					} else if let Some((di, dj)) = can_move_in_direction(&map, (i, j), direction) {
						map[di][dj] = Cell::Box;
						map[i + 1][j] = Cell::Empty;
						i += 1;
					}
				}
			}
		}
		let mut result = 0;
		for (i, row) in map.into_iter().enumerate() {
			for (j, cell) in row.into_iter().enumerate() {
				if let Cell::Box = cell {
					result += 100 * i + j;
				}
			}
		}
		result
	}
}

pub mod part2 {
	use std::collections::HashSet;

	use super::{Cell, Coord, Map, Parsed};

	fn wide_map(map: Map) -> Map {
		let mut new_map = vec![vec![Cell::Empty; map[0].len() * 2]; map.len()];
		for (i, row) in map.into_iter().enumerate() {
			for (j, cell) in row.into_iter().enumerate() {
				match cell {
					Cell::Wall => {
						new_map[i][j * 2] = Cell::Wall;
						new_map[i][j * 2 + 1] = Cell::Wall;
					}
					Cell::Box => {
						new_map[i][j * 2] = Cell::Left;
						new_map[i][j * 2 + 1] = Cell::Right;
					}
					_ => {}
				}
			}
		}
		new_map
	}

	fn can_move_horizontally(map: &Map, (mut i, mut j): Coord, direction: char) -> Option<Coord> {
		let (di, dj) = match direction {
			'<' => (0, usize::MAX),
			'>' => (0, 1),
			_ => unreachable!(),
		};
		i += di;
		j += dj;
		while map[i][j] == Cell::Left || map[i][j] == Cell::Right {
			i += di;
			j += dj;
		}
		(map[i][j] == Cell::Empty).then_some((i, j))
	}

	fn can_move_vertically(
		map: &Map,
		(mut i, mut j): Coord,
		direction: char,
	) -> Option<Vec<HashSet<Coord>>> {
		let (di, dj) = match direction {
			'^' => (usize::MAX, 0),
			'v' => (1, 0),
			_ => unreachable!(),
		};
		i += di;
		j += dj;
		if let Cell::Wall = map[i][j] {
			return None;
		}
		let mut current_lvl = HashSet::from([
			(i, j),
			if map[i][j] == Cell::Left {
				(i, j + 1)
			} else {
				(i, j - 1)
			},
		]);
		let mut result = vec![current_lvl.clone()];
		loop {
			let mut next_lvl: HashSet<_> = current_lvl
				.iter()
				.filter_map(|&(i, j)| {
					matches!(map[i][j], Cell::Left | Cell::Right).then_some((i + di, j + dj))
				})
				.collect();
			for (i, j) in next_lvl.clone() {
				match map[i][j] {
					Cell::Left if !next_lvl.contains(&(i, j + 1)) => {
						next_lvl.insert((i, j + 1));
					}
					Cell::Right if !next_lvl.contains(&(i, j - 1)) => {
						next_lvl.insert((i, j - 1));
					}
					_ => {}
				}
			}
			if next_lvl.iter().any(|&(i, j)| map[i][j] == Cell::Wall) {
				return None;
			}
			if !next_lvl
				.iter()
				.any(|&(i, j)| matches!(map[i][j], Cell::Left | Cell::Right))
			{
				result.push(next_lvl);
				return Some(result);
			}
			current_lvl = next_lvl.clone();
			result.push(next_lvl);
		}
	}

	pub fn solve((mut map, directions, (mut i, mut j)): Parsed) -> usize {
		map[i][j] = Cell::Empty;
		map = wide_map(map);
		j *= 2;
		for direction in directions {
			match direction {
				'<' => {
					if map[i][j - 1] == Cell::Empty {
						j -= 1;
					} else if let Some((_, dj)) = can_move_horizontally(&map, (i, j), direction) {
						for nj in (dj..=j - 2).step_by(2) {
							map[i][nj] = Cell::Left;
							map[i][nj + 1] = Cell::Right;
						}
						map[i][j - 1] = Cell::Empty;
						j -= 1;
					}
				}
				'>' => {
					if map[i][j + 1] == Cell::Empty {
						j += 1;
					} else if let Some((_, dj)) = can_move_horizontally(&map, (i, j), direction) {
						for nj in (j + 2..=dj).step_by(2) {
							map[i][nj] = Cell::Left;
							map[i][nj + 1] = Cell::Right;
						}
						map[i][j + 1] = Cell::Empty;
						j += 1;
					}
				}
				'^' => {
					if map[i - 1][j] == Cell::Empty {
						i -= 1;
					} else if let Some(levels) = can_move_vertically(&map, (i, j), direction) {
						for level in levels.iter().rev().skip(1) {
							for &(i, j) in level {
								if matches!(map[i][j], Cell::Left | Cell::Right) {
									map[i - 1][j] = map[i][j];
									map[i][j] = Cell::Empty;
								}
							}
						}
						i -= 1;
					}
				}
				_v => {
					if map[i + 1][j] == Cell::Empty {
						i += 1;
					} else if let Some(levels) = can_move_vertically(&map, (i, j), direction) {
						for level in levels.iter().rev().skip(1) {
							for &(i, j) in level {
								if matches!(map[i][j], Cell::Left | Cell::Right) {
									map[i + 1][j] = map[i][j];
									map[i][j] = Cell::Empty;
								}
							}
						}
						i += 1;
					}
				}
			}
		}
		let mut result = 0;
		for (i, row) in map.into_iter().enumerate() {
			for (j, cell) in row.into_iter().enumerate() {
				if let Cell::Left = cell {
					result += 100 * i + j;
				}
			}
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########\r
\r
<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
	.to_owned();
	// 	let test_input = "##########
	// #..O..O.O#
	// #......O.#
	// #.OO..O.O#
	// #..O@..O.#
	// #O#..O...#
	// #O..O..O.#
	// #.OO.O.OO#
	// #....O...#
	// ##########\r
	// \r
	// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
	// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
	// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
	// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
	// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
	// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
	// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
	// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
	// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
	// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
	// "
	// 	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_15_input.txt").unwrap()
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
