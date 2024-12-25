//! https://adventofcode.com/2024/day/24
//! https://adventofcode.com/2024/day/24/input

use std::{
	collections::{hash_map::Entry, HashMap},
	fs::read_to_string,
	time::{Duration, Instant},
};

use crate::LINE_ENDING;

#[derive(Copy, Clone, Debug)]
pub enum Value {
	Val(bool),
	And(usize, usize),
	Or(usize, usize),
	Xor(usize, usize),
	Invalid,
}

#[derive(Copy, Clone, Debug)]
pub struct Gate<'a> {
	name: &'a str,
	value: Value,
}

type Parsed<'a> = (Vec<Gate<'a>>, HashMap<&'a str, usize>);

fn parse(input: &str) -> Parsed {
	let sep = LINE_ENDING.repeat(2);
	let mut parts = input.split(&sep);
	let [values, connections] = core::array::from_fn(|_| parts.next().unwrap());
	let mut gates = Vec::with_capacity(values.len());
	let mut name_to_index = HashMap::new();
	for gate in values.lines() {
		let mut parts = gate.split(": ");
		let [name, value] = core::array::from_fn(|_| parts.next().unwrap());
		let index = gates.len();
		name_to_index.insert(name, index);
		gates.push(Gate {
			name,
			value: Value::Val(value.starts_with('1')),
		});
		// gates.push(Gate::Val(value.starts_with('1')));
	}
	for gate in connections.lines() {
		let mut parts = gate.split_whitespace();
		let [left, op, right, _, name] = core::array::from_fn(|_| parts.next().unwrap());
		let left_index = match name_to_index.entry(left) {
			Entry::Occupied(index) => *index.get(),
			Entry::Vacant(entry) => {
				let index = gates.len();
				entry.insert(index);
				gates.push(Gate {
					name,
					value: Value::Invalid,
				});
				index
			}
		};
		let right_index = match name_to_index.entry(right) {
			Entry::Occupied(index) => *index.get(),
			Entry::Vacant(entry) => {
				let index = gates.len();
				entry.insert(index);
				gates.push(Gate {
					name,
					value: Value::Invalid,
				});
				index
			}
		};
		let gate = match op.chars().next().unwrap() {
			'A' => Gate {
				name,
				value: Value::And(left_index, right_index),
			},
			'O' => Gate {
				name,
				value: Value::Or(left_index, right_index),
			},
			_x => Gate {
				name,
				value: Value::Xor(left_index, right_index),
			},
		};
		match name_to_index.entry(name) {
			Entry::Occupied(index) => {
				let &index = index.get();
				gates[index] = gate;
				index
			}
			Entry::Vacant(entry) => {
				let index = gates.len();
				entry.insert(index);
				gates.push(gate);
				index
			}
		};
	}
	(gates, name_to_index)
}

fn extract(name_to_index: &HashMap<&str, usize>, start: char) -> Vec<usize> {
	let mut gates: Vec<_> = name_to_index
		.iter()
		.filter(|(name, _)| name.starts_with(start))
		.collect();
	gates.sort();
	let indexes: Vec<_> = gates.iter().map(|(_, &index)| index).collect();
	indexes
}

fn sum_binary(gates: &[Gate], zs_index: &[usize]) -> u64 {
	let mut result = 0;
	for bit in zs_index
		.iter()
		.rev()
		.map(|&index| match gates[index].value {
			Value::Val(val) => {
				if val {
					1
				} else {
					0
				}
			}
			_ => {
				unreachable!()
			}
		}) {
		result <<= 1;
		result |= bit;
	}
	result
}

pub mod part1 {
	use super::{sum_binary, Parsed, Value};

	pub fn solve((mut gates, name_to_index): Parsed) -> u64 {
		let mut zs: Vec<_> = name_to_index
			.iter()
			.filter(|(name, _)| name.starts_with('z'))
			.collect();
		zs.sort();
		let zs_index: Vec<_> = zs.iter().map(|(_, &index)| index).collect();
		let mut stack = zs_index.clone();
		while let Some(gate) = stack.pop() {
			match gates[gate].value {
				Value::Val(_) => continue,
				Value::And(left, right)
					if matches!(
						(gates[left].value, gates[right].value),
						(Value::Val(_), Value::Val(_))
					) =>
				{
					let (Value::Val(left), Value::Val(right)) =
						(gates[left].value, gates[right].value)
					else {
						unreachable!()
					};
					gates[gate].value = Value::Val(left && right)
				}
				Value::Or(left, right)
					if matches!(
						(gates[left].value, gates[right].value),
						(Value::Val(_), Value::Val(_))
					) =>
				{
					let (Value::Val(left), Value::Val(right)) =
						(gates[left].value, gates[right].value)
					else {
						unreachable!()
					};
					gates[gate].value = Value::Val(left || right)
				}
				Value::Xor(left, right)
					if matches!(
						(gates[left].value, gates[right].value),
						(Value::Val(_), Value::Val(_))
					) =>
				{
					let (Value::Val(left), Value::Val(right)) =
						(gates[left].value, gates[right].value)
					else {
						unreachable!()
					};
					gates[gate].value = Value::Val(left ^ right);
				}
				Value::And(left, right) | Value::Or(left, right) | Value::Xor(left, right) => {
					stack.push(gate);
					stack.push(left);
					stack.push(right);
				}
				Value::Invalid => {}
			}
		}
		sum_binary(&gates, &zs_index)
	}
}

pub mod part2 {
	use std::fs;

	use super::{extract, Parsed, Value};

	pub fn solve((gates, name_to_index): Parsed) -> String {
		let mut graph = String::new();
		graph.push_str("Digraph G {\n");
		graph.push_str("node [shape=box, style=filled, color=lightgrey];\n");
		graph.push_str("graph [model=subset];\n");
		graph.push_str("rankdir=TB;\n");
		graph.push_str("{\n");
		graph.push_str("rank=same;\n");
		for gate in extract(&name_to_index, 'x')
			.into_iter()
			.chain(extract(&name_to_index, 'y'))
			.map(|n| gates[n])
		{
			graph.push_str(&format!("{} [color=gold];\n", gate.name));
		}
		graph.push_str("};\n");
		graph.push_str("{\n");
		graph.push_str("rank=sink;\n");
		for gate in extract(&name_to_index, 'z').into_iter().map(|n| gates[n]) {
			graph.push_str(&format!("{} [color=lightblue];\n", gate.name));
		}
		graph.push_str("};\n");
		for gate in gates
			.iter()
			.filter(|gate| !(gate.name.starts_with('x') || gate.name.starts_with('y')))
		{
			let (op, left, right) = match gate.value {
				Value::And(left, right) => ("AND", left, right),
				Value::Or(left, right) => ("OR", left, right),
				Value::Xor(left, right) => ("XOR", left, right),
				_ => {
					unreachable!()
				}
			};
			graph.push_str(&format!(
				"{} [label=\"{} ({})\"];\n",
				gate.name, gate.name, op
			));
			graph.push_str(&format!("{} -> {};\n", gates[left].name, gate.name));
			graph.push_str(&format!("{} -> {};\n", gates[right].name, gate.name));
		}
		graph.push_str("}\n");
		fs::write("graph.dot", graph).unwrap();
		"dot -Tpdf graph.dot -o graph.pdf".to_owned()
	}
}

#[allow(unused)]
pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0\r
\r
x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"
	.to_owned();
	let test_input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1\r
\r
ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
	.to_owned();
	let test_input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1\r
\r
x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2024/day_24_input.txt").unwrap()
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
