//! https://adventofcode.com/2024/day/5
//! https://adventofcode.com/2024/day/5/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

type Parsed = (Vec<(usize, usize)>, Vec<Vec<usize>>);

fn parse(input: &str) -> Parsed {
	// let sep = "\n".repeat(2);
	let sep = LINE_ENDING.repeat(2);
	let mut parts = input.split(&sep);
	let rules = parts.next().unwrap();
	let updates = parts.next().unwrap();
	(
		rules
			.lines()
			.map(|l| {
				let mut parts = l.split('|');
				(
					parts.next().unwrap().parse().unwrap(),
					parts.next().unwrap().parse().unwrap(),
				)
			})
			.collect(),
		updates
			.lines()
			.map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
			.collect(),
	)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve_graph((rules, updates): Parsed) -> usize {
		let mut result = 0;
		'update: for update in updates {
			let nodes_num = *update.iter().max().unwrap() + 1;
			let mut is_node = vec![false; nodes_num];
			for &node in &update {
				is_node[node] = true;
			}
			let mut graph = vec![vec![]; nodes_num];
			let mut reversed = vec![0; nodes_num];
			let mut levels = vec![usize::MAX; nodes_num];
			for &(a, b) in &rules {
				if a < is_node.len() && b < is_node.len() && is_node[a] && is_node[b] {
					graph[a].push(b);
					reversed[b] += 1;
				}
			}
			let mut free: Vec<_> = (0..reversed.len())
				.filter(|&n| is_node[n] && reversed[n] == 0)
				.collect();
			let mut level = 0;
			while !free.is_empty() {
				let mut next_free = Vec::new();
				for &node in &free {
					levels[node] = level;
					for &child in &graph[node] {
						reversed[child] -= 1;
						if reversed[child] == 0 {
							next_free.push(child);
						}
					}
				}
				level += 1;
				free = next_free;
			}
			for n in 1..update.len() {
				if levels[update[n]] < levels[update[n - 1]] {
					continue 'update;
				}
			}
			result += update[update.len() / 2];
		}
		result
	}

	pub fn solve((rules, updates): Parsed) -> usize {
		let mut result = 0;
		'update: for update in updates {
			for rule in &rules {
				if update.contains(&rule.0)
					&& update.contains(&rule.1)
					&& update.iter().position(|&e| e == rule.0)
						> update.iter().position(|&e| e == rule.1)
				{
					continue 'update;
				}
			}
			result += update[update.len() / 2]
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
	let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_05_input.txt").unwrap()
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
