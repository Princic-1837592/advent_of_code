//! https://adventofcode.com/2025/day/11
//! https://adventofcode.com/2025/day/11/input

use std::{
	collections::{hash_map::Entry, HashMap},
	fs::read_to_string,
	time::{Duration, Instant},
};

type Parsed = (Vec<Vec<usize>>, usize, usize, usize, usize, usize);

fn parse(input: &str) -> Parsed {
	let mut graph: Vec<Vec<_>> = vec![];
	let mut indexes: HashMap<&str, usize> = HashMap::new();
	for line in input.lines() {
		let parent = &line[..3];
		let parent_node = match indexes.entry(parent) {
			Entry::Occupied(entry) => *entry.get(),
			Entry::Vacant(entry) => {
				let node = graph.len();
				entry.insert(node);
				graph.push(vec![]);
				node
			}
		};
		for node in line[5..].split_whitespace() {
			let child_node = match indexes.entry(node) {
				Entry::Occupied(entry) => *entry.get(),
				Entry::Vacant(entry) => {
					let node = graph.len();
					entry.insert(node);
					graph.push(vec![]);
					node
				}
			};
			graph[parent_node].push(child_node);
		}
	}

	(
		graph,
		*indexes.get("you").unwrap_or(&usize::MAX),
		*indexes.get("out").unwrap_or(&usize::MAX),
		*indexes.get("dac").unwrap_or(&usize::MAX),
		*indexes.get("fft").unwrap_or(&usize::MAX),
		*indexes.get("svr").unwrap_or(&usize::MAX),
	)
}

pub mod part1 {
	use std::collections::VecDeque;

	use super::Parsed;

	pub fn solve((graph, you, out, _, _, _): Parsed) -> usize {
		let mut result = 0;
		let mut queue = VecDeque::from([(you, vec![false; graph.len()], 0)]);
		let mut last_steps = 0;
		while let Some((node, mut visited, steps)) = queue.pop_front() {
			if visited[node] {
				continue;
			}
			if steps > last_steps {
				last_steps = steps;
			}
			visited[node] = true;
			if node == out {
				result += 1;
				continue;
			}
			for &child in &graph[node] {
				queue.push_back((child, visited.clone(), steps + 1));
			}
		}
		result
	}
}

pub mod part2 {
	use std::collections::VecDeque;

	use super::Parsed;

	fn partial_dfs(
		graph: &Vec<Vec<usize>>,
		node: usize,
		target: usize,
		visited: &mut [bool],
		can_reach: &Vec<bool>,
	) -> usize {
		if node == target {
			return 1;
		}
		let mut result = 0;
		for &child in &graph[node] {
			if !visited[child] && can_reach[child] {
				visited[child] = true;
				result += partial_dfs(graph, child, target, visited, can_reach);
				visited[child] = false;
			}
		}
		result
	}

	fn can_reach(graph: &[Vec<usize>], start: usize, target: usize) -> bool {
		let mut visited = vec![false; graph.len()];
		let mut queue = VecDeque::from([start]);
		while let Some(node) = queue.pop_front() {
			if visited[node] {
				continue;
			}
			if node == target {
				return true;
			}
			visited[node] = true;
			for &child in &graph[node] {
				queue.push_back(child);
			}
		}
		false
	}

	pub fn solve((graph, _, out, dac, fft, svr): Parsed) -> usize {
		let mut result = 1;
		for (start, target) in [(svr, fft), (fft, dac), (dac, out)] {
			let mut can_reach_target = vec![false; graph.len()];
			for (start_node, reached) in can_reach_target.iter_mut().enumerate() {
				if can_reach(&graph, start_node, target) {
					*reached = true;
				}
			}
			result *= partial_dfs(
				&graph,
				start,
				target,
				&mut vec![false; graph.len()],
				&can_reach_target,
			);
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_11_input.txt").unwrap()
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
