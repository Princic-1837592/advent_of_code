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
	use super::Parsed;

	pub fn solve(_parsed: Parsed) -> usize {
		0
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "##########\r
#..O..O.O#\r
#......O.#\r
#.OO..O.O#\r
#..O@..O.#\r
#O#..O...#\r
#O..O..O.#\r
#.OO.O.OO#\r
#....O...#\r
##########\r
\r
<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\r
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\r
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\r
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\r
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\r
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\r
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\r
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\r
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\r
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\r
"
	.to_owned();
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
