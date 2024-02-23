//! https://adventofcode.com/2023/day/20
//! https://adventofcode.com/2023/day/20/input

use std::{
    collections::HashMap,
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Module {
    index: usize,
    module_type: Type,
    ins: Vec<usize>,
    outs: Vec<usize>,
    name: String,
}

impl From<(&str, &HashMap<&str, usize>)> for Module {
    fn from((value, indexes): (&str, &HashMap<&str, usize>)) -> Self {
        let mut parts = value.split(" -> ");
        let type_and_name = parts.next().unwrap();
        let module_type = (type_and_name, indexes.len()).into();
        Self {
            index: *indexes
                .get(&if let Type::Broadcast = module_type {
                    type_and_name
                } else {
                    type_and_name.trim_start_matches(|c| c == '%' || c == '&')
                })
                .unwrap(),
            module_type,
            ins: vec![],
            outs: parts
                .next()
                .unwrap()
                .split(", ")
                .map(|n| *indexes.get(&n).unwrap())
                .collect(),
            name: type_and_name.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { last_ins: Vec<bool> },
    None,
}

impl From<(&str, usize)> for Type {
    fn from((value, indexes): (&str, usize)) -> Self {
        if value == "broadcaster" {
            Self::Broadcast
        } else {
            match value.chars().next().unwrap() {
                '%' => Self::FlipFlop { on: false },
                '&' => Self::Conjunction {
                    last_ins: vec![false; indexes],
                },
                _ => Self::None,
            }
        }
    }
}

type Parsed = Vec<Module>;

fn parse(input: &str) -> Parsed {
    let lines: Vec<_> = input.lines().collect();
    let mut indexes = HashMap::new();
    for line in &lines {
        let mut line = line.split(" -> ");
        let name = line
            .next()
            .unwrap()
            .trim_start_matches(|c| c == '%' || c == '&');
        for name in line.next().unwrap().split(", ").chain([name]) {
            if !indexes.contains_key(name) {
                indexes.insert(name, indexes.len());
            }
        }
    }
    let mut result: Vec<_> = input.lines().map(|l| Module::from((l, &indexes))).collect();
    result.sort();
    let mut indexes: Vec<_> = indexes.into_iter().collect();
    indexes.sort_by_key(|&(_, i)| i);
    for (name, i) in indexes {
        if result[i].index != i {
            result.insert(
                i,
                Module {
                    index: i,
                    module_type: Type::None,
                    ins: vec![],
                    outs: vec![],
                    name: name.to_owned(),
                },
            );
        }
    }
    for from in 0..result.len() {
        let outs = result[from].outs.to_vec();
        for to in outs {
            if to < result.len() {
                result[to].ins.push(from);
            }
        }
    }
    result
}

#[derive(Copy, Clone, Debug)]
struct Pulse {
    high: bool,
    from: usize,
    to: usize,
}

impl Module {
    fn pulse(&mut self, high: bool, from: usize) -> Option<Vec<Pulse>> {
        let out_high = match &mut self.module_type {
            Type::Broadcast => Some(high),
            Type::FlipFlop { on } => {
                if !high {
                    *on = !*on;
                    Some(*on)
                } else {
                    None
                }
            }
            Type::Conjunction { last_ins } => {
                last_ins[from] = high;
                Some(!self.ins.iter().all(|&from| last_ins[from]))
            }
            _ => None,
        };
        out_high.map(|out_high| {
            self.outs
                .iter()
                .map(|&to| Pulse {
                    high: out_high,
                    from: self.index,
                    to,
                })
                .collect()
        })
    }
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{Parsed, Pulse, Type};

    pub fn solve(mut modules: Parsed) -> usize {
        let (mut low_pulses, mut high_pulses) = (0, 0);
        for _ in 0..1000 {
            let mut queue = VecDeque::from([Pulse {
                high: false,
                from: usize::MAX,
                to: modules
                    .iter()
                    .position(|m| matches!(m.module_type, Type::Broadcast))
                    .unwrap(),
            }]);
            while let Some(Pulse { high, from, to }) = queue.pop_front() {
                if high {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                if let Some(new_pulses) = modules[to].pulse(high, from) {
                    for new_pulse in new_pulses {
                        queue.push_back(new_pulse);
                    }
                }
            }
        }
        low_pulses * high_pulses
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use utils::math::lcm;

    use super::{Parsed, Pulse, Type};

    pub fn solve(modules: Parsed) -> usize {
        let broadcaster = modules
            .iter()
            .position(|m| matches!(m.module_type, Type::Broadcast))
            .unwrap();
        let inputs = modules[modules.iter().find(|m| m.name.ends_with("rx")).unwrap().ins[0]]
            .ins
            .clone();
        let mut loops = Vec::with_capacity(inputs.len());
        for input in inputs {
            let mut modules = modules.clone();
            'outer: for i in 1.. {
                let mut queue = VecDeque::from([Pulse {
                    high: false,
                    from: usize::MAX,
                    to: broadcaster,
                }]);
                while let Some(Pulse { high, from, to }) = queue.pop_front() {
                    if from == input && high {
                        loops.push(i);
                        break 'outer;
                    }
                    if let Some(new_pulses) = modules[to].pulse(high, from) {
                        for new_pulse in new_pulses {
                            queue.push_back(new_pulse);
                        }
                    }
                }
            }
        }
        loops.into_iter().fold(1, lcm)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_20_input.txt").unwrap()
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
