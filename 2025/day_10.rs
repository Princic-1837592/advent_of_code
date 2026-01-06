//! https://adventofcode.com/2025/day/10
//! https://adventofcode.com/2025/day/10/input

use std::{
	fs::read_to_string,
	str::FromStr,
	time::{Duration, Instant},
};

use utils::parsing::parse_lines;

type Parsed = Vec<Machine>;

#[derive(Clone, Debug)]
pub struct Machine {
	lights: u16,
	correct: u16,
	combos: Vec<Vec<usize>>,
	combos_b: Vec<u16>,
	joltage: Vec<usize>,
}

impl FromStr for Machine {
	type Err = ();

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let parts: Vec<_> = line.split_whitespace().collect();
		let n_lights = parts[0].len() - 2;
		let mut correct = 0;
		let mut mask = 1;
		for c in parts[0].chars().skip(1).take(n_lights) {
			if c == '#' {
				correct |= mask;
			}
			mask <<= 1;
		}
		let combos: Vec<Vec<_>> = parts[1..parts.len() - 1]
			.iter()
			.map(|c| {
				c[1..c.len() - 1]
					.split(',')
					.map(|b| b.parse().unwrap())
					.collect()
			})
			.collect();
		let combos_b = parts[1..parts.len() - 1]
			.iter()
			.map(|b| {
				let mut mask = 0;
				for button in b[1..b.len() - 1]
					.split(',')
					.map(|l| l.parse::<u16>().unwrap())
				{
					mask |= 1 << button
				}
				mask
			})
			.collect();
		let joltage = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
			.split(',')
			.map(|j| j.parse().unwrap())
			.collect();
		Ok(Machine {
			lights: 0,
			correct,
			combos,
			combos_b,
			joltage,
		})
	}
}

impl Machine {
	fn apply(&mut self, combo: usize) {
		self.lights ^= self.combos_b[combo];
	}
}

fn parse(input: &str) -> Parsed {
	parse_lines(input)
}

pub mod part1 {
	use super::Parsed;

	pub fn solve(machines: Parsed) -> usize {
		let mut result = 0;
		for mut machine in machines {
			let mut min_buttons = u32::MAX;
			for combo in 0..2_usize.pow(machine.combos_b.len() as u32) {
				machine.lights = 0;
				for b in 0..machine.combos_b.len() {
					if combo & (1 << b) != 0 {
						machine.apply(b);
					}
				}
				if machine.lights == machine.correct && combo.count_ones() < min_buttons {
					min_buttons = combo.count_ones();
				}
			}
			result += min_buttons as usize;
		}
		result
	}
}

pub mod part2 {
	use std::{
		io::{Read, Write},
		process::{Command, Stdio},
	};

	use crate::day_10::Parsed;

	pub fn solve(machines: Parsed) -> usize {
		let mut result = 0;
		for machine in machines {
			let mut command = Command::new("z3")
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.arg("-in")
				.spawn()
				.unwrap();
			let mut model = String::new();
			for i in 0..machine.combos.len() {
				model.push_str(&format!("(declare-const x{i} Int)\n"));
				model.push_str(&format!("(assert (>= x{i} 0))\n"));
			}
			model.push_str("\n(declare-const result Int)\n\n");
			for (j, joltage) in machine.joltage.iter().enumerate() {
				let buttons = machine
					.combos
					.iter()
					.enumerate()
					.filter_map(|(c, combo)| combo.contains(&j).then_some(format!("x{c}")))
					.collect::<Vec<_>>()
					.join(" ");
				model.push_str(&format!("(assert (= (+ {buttons}) {joltage}))\n"));
			}
			let all_vars = (0..machine.combos.len())
				.map(|c| format!("x{c}"))
				.collect::<Vec<_>>()
				.join(" ");
			model.push_str(&format!("\n(assert (= result (+ {all_vars})))\n"));
			model.push_str("(minimize result)\n(check-sat)\n(get-value (result))");
			let mut input = command.stdin.take().unwrap();
			input.write_all(model.as_bytes()).unwrap();
			drop(input);
			let mut output = String::new();
			command
				.stdout
				.as_mut()
				.unwrap()
				.read_to_string(&mut output)
				.unwrap();
			let result_line = output.lines().nth(1).unwrap();
			result += result_line[9..result_line.len() - 2]
				.parse::<usize>()
				.unwrap();
			command.wait().unwrap();
		}
		result
	}
}

pub fn main(test: bool, verbose: bool) -> Duration {
	let test_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
	.to_owned();
	let puzzle_input = if test {
		test_input
	} else {
		read_to_string("../inputs/2025/day_10_input.txt").unwrap()
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
