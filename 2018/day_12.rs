//! https://adventofcode.com/2018/day/12
//! https://adventofcode.com/2018/day/12/input

use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fs::read_to_string,
    time::Instant,
};

use itertools::Itertools;

fn parse(input: &str) -> (VecDeque<bool>, [bool; 32]) {
    let mut lines = input.lines();
    let initial_state = lines.next().unwrap()[15..]
        .chars()
        .map(|char| char == '#')
        .collect();
    lines.next();
    let mut rules = [false; 32];
    lines.for_each(|line| {
        let mut parts = line.split(" => ");
        let pattern = parts.next().unwrap();
        let key = pattern
            .chars()
            .fold(0, |key, char| (key << 1) | (char == '#') as usize);
        rules[key] = parts.next().unwrap().starts_with('#');
    });
    (initial_state, rules)
}

fn apply_generations(mut state: VecDeque<bool>, rules: [bool; 32], generations: usize) -> isize {
    let mut zero_plant = 0;
    let mut mask;
    let mut support = VecDeque::with_capacity(state.len());
    let mut seen = HashMap::new();
    support.clone_from(&state);
    let mut g = 0;
    while g < generations {
        for _ in 0..2 {
            support.push_back(false);
            support.push_front(false);
            zero_plant += 1;
        }
        mask = 0;
        for (i, plant) in support.iter_mut().enumerate() {
            mask <<= 1;
            mask = (mask | *state.get(i).unwrap_or(&false) as usize) & 0b011111;
            *plant = rules[mask];
        }
        while !*support.back().unwrap() {
            support.pop_back();
        }
        while !*support.front().unwrap() {
            support.pop_front();
            zero_plant -= 1;
        }
        state.clone_from(&support);
        g += 1;
        let key = state.iter().join("");
        match seen.entry(key) {
            Entry::Occupied(entry) => {
                let (last, zero) = entry.get();
                let growth = zero_plant - zero;
                let interval = g - last;
                let left = generations - g;
                let cycles = left / interval;
                zero_plant += cycles as isize * growth;
                g += cycles * interval;
            }
            Entry::Vacant(entry) => {
                entry.insert((g, zero_plant));
            }
        }
    }
    state
        .iter()
        .enumerate()
        .filter_map(|(i, state)| state.then_some(i as isize - zero_plant))
        .sum()
}

pub mod part1 {
    use crate::day_12::{apply_generations, parse};

    pub fn solve(input: &str) -> isize {
        let (initial_state, rules) = parse(input);
        apply_generations(initial_state, rules, 20)
    }
}

pub mod part2 {
    use crate::day_12::{apply_generations, parse};

    pub fn solve(input: &str) -> isize {
        let (initial_state, rules) = parse(input);
        apply_generations(initial_state, rules, 50000000000)
    }
}

pub fn main(test: bool) {
    let test_input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
