//! https://adventofcode.com/2024/day/12
//! https://adventofcode.com/2024/day/12/input

use std::{
	collections::VecDeque,
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = Vec<Vec<char>>;
type Coord = (usize, usize);

fn parse(input: &str) -> Parsed {
	input.lines().map(|l| l.chars().collect()).collect()
}

fn bfs(i: usize, j: usize, map: &Parsed, seen: &mut [Vec<bool>]) -> Vec<Coord> {
	let char = map[i][j];
	let mut queue = VecDeque::from([(i, j)]);
	let mut area = vec![];
	while let Some((i, j)) = queue.pop_front() {
		if seen[i][j] {
			continue;
		}
		seen[i][j] = true;
		area.push((i, j));
		for (ni, nj) in [
			Some((i + 1, j)),
			Some((i, j + 1)),
			i.checked_sub(1).map(|ni| (ni, j)),
			j.checked_sub(1).map(|nj| (i, nj)),
		]
		.into_iter()
		.flatten()
		{
			if ni >= map.len() || nj >= map[0].len() || map[ni][nj] != char {
				continue;
			}
			queue.push_back((ni, nj));
		}
	}
	area
}

fn find_regions(map: &Parsed) -> Vec<Vec<Coord>> {
	let mut regions = vec![];
	let mut seen = vec![vec![false; map[0].len()]; map.len()];
	for i in 0..map.len() {
		for j in 0..map[i].len() {
			if seen[i][j] {
				continue;
			}
			let region = bfs(i, j, map, &mut seen);
			region.iter().for_each(|&(i, j)| seen[i][j] = true);
			regions.push(region);
		}
	}
	regions
}

pub mod part1 {
	use super::{find_regions, Parsed};

	pub fn solve(map: Parsed) -> usize {
		let regions = find_regions(&map);
		regions
			.into_iter()
			.map(|region| {
				let char = map[region[0].0][region[0].1];
				region.len()
					* region
						.into_iter()
						.flat_map(|(i, j)| {
							[
								Some((i + 1, j)),
								Some((i, j + 1)),
								i.checked_sub(1).map(|ni| (ni, j)),
								j.checked_sub(1).map(|nj| (i, nj)),
							]
							.into_iter()
							.map(|near| match near {
								None => 1,
								Some((ni, nj))
									if ni >= map.len()
										|| nj >= map[0].len() || map[ni][nj] != char =>
								{
									1
								}
								_ => 0,
							})
						})
						.sum::<usize>()
			})
			.sum()
	}
}

pub mod part2 {
	use super::{find_regions, Parsed};

	pub fn solve(map: Parsed) -> usize {
		let regions = find_regions(&map);
		regions
			.into_iter()
			.map(|region| {
				let char = map[region[0].0][region[0].1];
				region.len() * {
					let mut corners = 0;
					for &(i, j) in &region {
						let mut pattern: u8 = 0;
						for near in [
							i.checked_sub(1)
								.and_then(|ni| j.checked_sub(1).map(|nj| (ni, nj))),
							i.checked_sub(1).map(|ni| (ni, j)),
							i.checked_sub(1).map(|ni| (ni, j + 1)),
							j.checked_sub(1).map(|nj| (i, nj)),
							Some((i, j + 1)),
							j.checked_sub(1).map(|nj| (i + 1, nj)),
							Some((i + 1, j)),
							Some((i + 1, j + 1)),
						] {
							let v = match near {
								None => 1,
								Some((ni, nj))
									if ni >= map.len()
										|| nj >= map[0].len() || map[ni][nj] != char =>
								{
									1
								}
								_ => 0,
							};
							pattern <<= 1;
							pattern |= v;
						}
						for (corner, adjacent) in [
							(0b_1000_0000, 0b0101_0000),
							(0b_0010_0000, 0b0100_1000),
							(0b_0000_0100, 0b0001_0010),
							(0b_0000_0001, 0b0000_1010),
						] {
							if pattern & corner != 0 && (pattern & adjacent).count_ones() % 2 == 0
								|| pattern & corner == 0 && (pattern & adjacent).count_ones() == 2
							{
								corners += 1;
							}
						}
					}
					corners
				}
			})
			.sum()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_12_input.txt").unwrap()
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
