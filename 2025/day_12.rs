//! https://adventofcode.com/2025/day/12
//! https://adventofcode.com/2025/day/12/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

type Parsed = (Vec<Vec<Piece>>, Vec<Tree>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
	rows: [u64; 3],
	occupied: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Tree {
	width: usize,
	height: usize,
	gifts: [usize; 6],
}

impl Piece {
	fn rotate(&self) -> Self {
		Self {
			rows: [
				(self.rows[2] & (1 << 63))
					| ((self.rows[1] & (1 << 63)) >> 1)
					| ((self.rows[0] & (1 << 63)) >> 2),
				((self.rows[2] & (1 << 62)) << 1)
					| (self.rows[1] & (1 << 62))
					| ((self.rows[0] & (1 << 62)) >> 1),
				((self.rows[2] & (1 << 61)) << 2)
					| ((self.rows[1] & (1 << 61)) << 1)
					| (self.rows[0] & (1 << 61)),
			],
			..*self
		}
	}

	fn flip(&self) -> Self {
		Self {
			rows: self.rows.map(Self::flip_row),
			..*self
		}
	}

	fn flip_row(row: u64) -> u64 {
		((row & (1 << 63)) >> 2) | (row & (1 << 62)) | ((row & (1 << 61)) << 2)
	}
}

fn parse(input: &str) -> Parsed {
	let mut pieces = vec![];
	let mut trees = vec![];
	let separator = &LINE_ENDING.repeat(2);
	let parts: Vec<_> = input.split(separator).collect();
	for piece in &parts[..6] {
		let mut rows = [0; 3];
		for (l, line) in piece.lines().skip(1).enumerate() {
			for (c, char) in line.chars().enumerate() {
				if char == '#' {
					rows[l] |= 1 << (63 - c);
				}
			}
		}
		let this_piece = Piece {
			rows,
			occupied: rows.map(|r| r.count_ones() as usize).iter().sum(),
		};
		let mut this_pieces = vec![this_piece];
		for piece in [
			this_piece.rotate(),
			this_piece.rotate().rotate(),
			this_piece.rotate().rotate().rotate(),
			this_piece.flip(),
			this_piece.flip().rotate(),
			this_piece.flip().rotate().rotate(),
			this_piece.flip().rotate().rotate().rotate(),
		] {
			if !this_pieces.contains(&piece) {
				this_pieces.push(piece);
			}
		}
		pieces.push(this_pieces);
	}
	for line in parts[6].lines() {
		let mut gifts = [0; 6];
		let mut parts = line.split_whitespace();
		let mut wxh = parts.next().unwrap().trim_end_matches(':').split('x');
		for (i, gift_count) in parts.enumerate() {
			gifts[i] = gift_count.parse().unwrap();
		}
		trees.push(Tree {
			width: wxh.next().unwrap().parse().unwrap(),
			height: wxh.next().unwrap().parse().unwrap(),
			gifts,
		})
	}
	(pieces, trees)
}

pub mod part1 {
	use super::Parsed;

	/*fn can_solve(
		map: &mut [u64],
		width: usize,
		counts: &mut [usize; 6],
		left: usize,
		pieces: &[Vec<Piece>],
	) -> bool {
		if left == 0 {
			return true;
		}
		for (p, piece) in pieces.iter().enumerate() {
			if counts[p] == 0 {
				continue;
			}
			for orientation in piece {
				for i in 0..map.len() - 2 {
					for j in 0..width - 2 {
						if (0..3).all(|r| map[i + r] & (orientation.rows[r] >> j) == 0) {
							(0..3).for_each(|r| map[i + r] |= orientation.rows[r] >> j);
							counts[p] -= 1;
							if can_solve(map, width, counts, left - 1, pieces) {
								return true;
							}
							(0..3).for_each(|r| map[i + r] &= !(orientation.rows[r] >> j));
							counts[p] += 1;
						}
					}
				}
			}
		}
		false
	}*/

	pub fn solve((pieces, trees): Parsed) -> usize {
		trees
			.iter()
			.filter(|tree| {
				tree.gifts
					.iter()
					.zip(&pieces)
					.map(|(count, piece)| count * piece[0].occupied)
					.sum::<usize>() <= tree.width * tree.height
				/*&& can_solve(
					&mut vec![0; tree.height],
					tree.width,
					&mut tree.gifts.clone(),
					tree.gifts.iter().sum(),
					&pieces,
				)*/
			})
			.count()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "0:\r
###\r
##.\r
##.\r
\r
1:\r
###\r
##.\r
.##\r
\r
2:\r
.##\r
###\r
##.\r
\r
3:\r
##.\r
###\r
##.\r
\r
4:\r
###\r
#..\r
###\r
\r
5:\r
###\r
.#.\r
###\r
\r
4x4: 0 0 0 0 2 0\r
12x5: 1 0 1 0 2 2\r
12x5: 1 0 1 0 3 2\r
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_12_input.txt").unwrap()
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

	if verbose {
		println!("Total {:?}", total);
	}
	total
}
