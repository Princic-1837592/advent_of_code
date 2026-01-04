//! https://adventofcode.com/2025/day/8
//! https://adventofcode.com/2025/day/8/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

type Parsed = Vec<Box>;

#[derive(Copy, Clone, Debug)]
pub struct Box {
	x: usize,
	y: usize,
	z: usize,
}

impl FromStr for Box {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.splitn(3, ',');
		Ok(Self {
			x: parts.next().unwrap().parse().unwrap(),
			y: parts.next().unwrap().parse().unwrap(),
			z: parts.next().unwrap().parse().unwrap(),
		})
	}
}

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use std::collections::BinaryHeap;

	use union_find::{QuickFindUf, UnionBySize, UnionFind};

	use super::Parsed;

	pub fn solve(boxes: Parsed, iterations: usize) -> usize {
		let n_boxes = boxes.len();
		let mut distances = vec![vec![0; n_boxes]; n_boxes];
		let mut heap = BinaryHeap::with_capacity(n_boxes);
		for i in 0..n_boxes {
			for j in i + 1..n_boxes {
				let dist = -(((boxes[i].x.abs_diff(boxes[j].x).pow(2)
					+ boxes[i].y.abs_diff(boxes[j].y).pow(2)
					+ boxes[i].z.abs_diff(boxes[j].z).pow(2)) as f32)
					.sqrt()
					.to_bits() as i32);
				distances[i][j] = dist;
				distances[j][i] = dist;
				heap.push((dist, i, j));
			}
		}
		let mut uf = QuickFindUf::<UnionBySize>::new(n_boxes);
		let mut unions = 0;
		while unions < iterations {
			let (_dist, box_i, box_j) = heap.pop().unwrap();
			uf.union(box_i, box_j);
			unions += 1;
		}
		let mut sizes = vec![0; n_boxes];
		for box_i in 0..n_boxes {
			sizes[uf.find(box_i)] += 1;
		}
		sizes.sort();
		sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
	}
}

pub mod part2 {
	use std::collections::BinaryHeap;

	use union_find::{QuickFindUf, UnionBySize, UnionFind};

	use super::Parsed;

	pub fn solve(boxes: Parsed) -> usize {
		let n_boxes = boxes.len();
		let mut distances = vec![vec![0; n_boxes]; n_boxes];
		let mut heap = BinaryHeap::with_capacity(n_boxes);
		for i in 0..n_boxes {
			for j in i + 1..n_boxes {
				let dist = -(((boxes[i].x.abs_diff(boxes[j].x).pow(2)
					+ boxes[i].y.abs_diff(boxes[j].y).pow(2)
					+ boxes[i].z.abs_diff(boxes[j].z).pow(2)) as f32)
					.sqrt()
					.to_bits() as i32);
				distances[i][j] = dist;
				distances[j][i] = dist;
				heap.push((dist, i, j));
			}
		}
		let mut uf = QuickFindUf::<UnionBySize>::new(n_boxes);
		while let Some((_dist, box_i, box_j)) = heap.pop() {
			if uf.union(box_i, box_j) && uf.get(box_i).size() == n_boxes {
				return boxes[box_i].x * boxes[box_j].x;
			}
		}
		unreachable!()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
	.to_owned();
	let (puzzle_input, iterations) = if test {
		(test_input, 10)
	} else {
		(
			read_to_string("../inputs/2025/day_08_input.txt").unwrap(),
			1000,
		)
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
	let result = part1::solve(parsed.clone(), iterations);
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
