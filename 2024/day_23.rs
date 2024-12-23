//! https://adventofcode.com/2024/day/23
//! https://adventofcode.com/2024/day/23/input

use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

use utils::parsing::parse_alpha;

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
	let mut graph = vec![vec![]; 26 * 26];
	for line in input.lines() {
		let mut parts = line.split('-');
		let left = parse_alpha::<'a'>(parts.next().unwrap());
		let right = parse_alpha::<'a'>(parts.next().unwrap());
		graph[left].push(right);
		graph[right].push(left);
	}
	graph
}

pub mod part1 {
	use std::collections::{HashSet, VecDeque};

	use super::Parsed;

	pub fn solve(graph: Parsed) -> usize {
		let mut seen = HashSet::new();
		for node in 0..26 {
			let node = 19 * 26 + node;
			let target = node;
			let mut queue = VecDeque::from([(node, 0, [usize::MAX, usize::MAX, usize::MAX])]);
			while let Some((node, distance, mut path)) = queue.pop_front() {
				if node == target && distance == 3 {
					seen.insert(path.iter().product::<usize>());
					continue;
				}
				if distance >= 3 || path.contains(&node) {
					continue;
				}
				path[distance] = node;
				for &next in &graph[node] {
					queue.push_back((next, distance + 1, path));
				}
			}
		}
		seen.len()
	}
}

pub mod part2 {
	use std::collections::HashSet;

	use super::Parsed;

	pub fn solve(graph: Parsed) -> String {
		let mut connections = vec![vec![]; graph.len() + 1];
		for (node, nodes) in graph
			.iter()
			.enumerate()
			.filter(|(_, nodes)| !nodes.is_empty())
		{
			connections[nodes.len()].push(node);
		}
		let mut lan: Vec<_> = connections
			.into_iter()
			.enumerate()
			.filter(|(c, nodes)| nodes.len() > *c)
			.map(|(_, nodes)| bron_kerbosch(&graph, nodes))
			.max_by_key(|l| l.len())
			.unwrap();
		lan.sort();
		let lan: Vec<_> = lan
			.into_iter()
			.map(|n| {
				format!(
					"{}{}",
					((n / 26 % 26) as u8 + b'a') as char,
					((n % 26) as u8 + b'a') as char
				)
			})
			.collect();
		lan.join(",")
	}

	fn bron_kerbosch(graph: &Vec<Vec<usize>>, lan: Vec<usize>) -> Vec<usize> {
		let p: HashSet<_> = graph
			.iter()
			.enumerate()
			.filter_map(|(n, _)| lan.contains(&n).then_some(n))
			.collect();
		let mut cliques = vec![];
		fn recursive(
			graph: &Vec<Vec<usize>>,
			r: HashSet<usize>,
			p: HashSet<usize>,
			x: HashSet<usize>,
			cliques: &mut Vec<HashSet<usize>>,
		) {
			if p.is_empty() && x.is_empty() {
				cliques.push(r);
			} else {
				let &u = p.union(&x).next().unwrap();
				for &v in p.iter().filter(|n| !graph[u].contains(n)) {
					recursive(
						graph,
						r.union(&HashSet::from([v])).copied().collect(),
						p.iter().copied().filter(|p| graph[v].contains(p)).collect(),
						x.intersection(&graph[v].iter().copied().collect())
							.copied()
							.collect(),
						cliques,
					);
				}
			}
		}
		recursive(graph, HashSet::new(), p, HashSet::new(), &mut cliques);
		cliques
			.iter()
			.max_by_key(|c| c.len())
			.unwrap()
			.iter()
			.copied()
			.collect()
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_23_input.txt").unwrap()
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
