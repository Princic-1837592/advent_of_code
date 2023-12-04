//! https://adventofcode.com/2016/day/11
//! https://adventofcode.com/2016/day/11/input

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
    time::Instant,
};

use regex::Regex;

const UP: u64 = 0;
const DOWN: u64 = 1;
const GENERATOR: u64 = 0;
const MICROCHIP: u64 = 1;
const FIRST_FLOOR: u64 = 0;
const LAST_FLOOR: u64 = 3;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    items: u64,
    elevator: usize,
}

fn gm(generator: u64, microchip: u64) -> u64 {
    1 << (generator << 4) << (microchip << 2)
}

fn parse(input: &str) -> (State, State) {
    #[derive(Debug, Clone, Copy)]
    struct Pair {
        generator: u64,
        microchip: u64,
    }

    let mut elements: HashMap<&str, Pair> = HashMap::new();
    let pattern = Regex::new(r"a (\w+)( generator|-compatible microchip)").unwrap();
    for (floor, line) in input.lines().enumerate().map(|(i, l)| (i as u64, l)) {
        for capture in pattern.captures_iter(line) {
            let element = capture.get(1).unwrap().as_str();
            let item = capture.get(2).unwrap().as_str().chars().nth(1).unwrap();
            match elements.entry(element) {
                Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    if item == 'g' {
                        entry.generator = floor;
                    } else {
                        entry.microchip = floor;
                    }
                }
                Entry::Vacant(entry) => {
                    if item == 'g' {
                        entry.insert(Pair {
                            generator: floor,
                            microchip: 0,
                        });
                    } else {
                        entry.insert(Pair {
                            generator: 0,
                            microchip: floor,
                        });
                    }
                }
            }
        }
    }

    let mut start = State {
        items: 0,
        elevator: FIRST_FLOOR as usize,
    };
    let mut end = State {
        items: 0,
        elevator: LAST_FLOOR as usize,
    };
    for pair in elements.values() {
        start.items += gm(pair.generator, pair.microchip);
        end.items += gm(LAST_FLOOR, LAST_FLOOR);
    }
    (start, end)
}

fn udgmo(up_down: u64, gen_or_micro: u64, other: u64) -> u64 {
    (up_down << 3) | (gen_or_micro << 2) | other
}

fn move_table() -> [[u64; 16]; 4] {
    let mut result = [[0x8888_8888_8888_8888; 16]; 4];
    for (floor, row) in result.iter_mut().enumerate().map(|(i, r)| (i as u64, r)) {
        for other in FIRST_FLOOR..=LAST_FLOOR {
            if floor > FIRST_FLOOR {
                row[udgmo(DOWN, GENERATOR, other) as usize] =
                    gm(floor - 1, other).wrapping_sub(gm(floor, other));
                row[udgmo(DOWN, MICROCHIP, other) as usize] =
                    gm(other, floor - 1).wrapping_sub(gm(other, floor));
            }
            if floor < LAST_FLOOR {
                row[udgmo(UP, GENERATOR, other) as usize] =
                    gm(floor + 1, other).wrapping_sub(gm(floor, other));
                row[udgmo(UP, MICROCHIP, other) as usize] =
                    gm(other, floor + 1).wrapping_sub(gm(other, floor));
            }
        }
    }
    result
}

fn legal(state: u64) -> bool {
    state & 0x8888_8888_8888_8888 == 0
}

fn compatible(state: u64) -> bool {
    !(state & 0x0000_0000_0000_ffff != 0 && state & 0x000f_000f_000f_0000 != 0
        || state & 0x0000_0000_ffff_0000 != 0 && state & 0x00f0_00f0_0000_00f0 != 0
        || state & 0x0000_ffff_0000_0000 != 0 && state & 0x0f00_0000_0f00_0f00 != 0
        || state & 0xffff_0000_0000_0000 != 0 && state & 0x0000_f000_f000_f000 != 0)
}

fn sign(depth: isize) -> isize {
    (depth * 2 + 1).signum()
}

#[allow(unused)]
fn pretty_print(state: u64) -> String {
    format!(
        "0x_{:0>4x}_{:0>4x}_{:0>4x}_{:0>4x}",
        (state & 0xffff_0000_0000_0000) >> (16 * 3),
        (state & 0x0000_ffff_0000_0000) >> (16 * 2),
        (state & 0x0000_0000_ffff_0000) >> 16,
        (state & 0x0000_0000_0000_ffff)
    )
}

fn solve_generic(start: State, end: State) -> usize {
    let move_table = move_table();
    let (mut prev, mut curr, mut next) = (
        HashMap::new(),
        HashMap::from([(start, 0), (end, -1)]),
        HashMap::new(),
    );
    while !curr.is_empty() {
        for (&State { items, elevator }, &depth) in &curr {
            let cur_sign = sign(depth);
            for c1 in 0..16 {
                let items = items.wrapping_add(move_table[elevator][c1]);
                if !legal(items) {
                    continue;
                }
                let next_elevator = elevator.wrapping_sub((c1 >> 2) & 2).wrapping_add(1);
                for c2 in 0..=8 {
                    let mut items = items;
                    if c2 != 8 {
                        items = items.wrapping_add(move_table[elevator][c2 | (c1 & 8)]);
                    }
                    if !legal(items) || !compatible(items) {
                        continue;
                    }
                    let state = State {
                        items,
                        elevator: next_elevator,
                    };
                    match prev
                        .get(&state)
                        .or_else(|| curr.get(&state))
                        .or_else(|| next.get(&state))
                    {
                        None => {
                            next.insert(state, depth + cur_sign);
                        }
                        Some(&contained) if cur_sign != sign(contained) => {
                            return depth.unsigned_abs() + contained.unsigned_abs();
                        }
                        _ => {}
                    }
                }
            }
        }
        let tmp = prev;
        prev = curr;
        curr = next;
        next = tmp;
        next.clear();
    }
    usize::MAX
}

pub mod part1 {
    use super::{parse, solve_generic};

    pub fn solve(input: &str) -> usize {
        let (start, end) = parse(input);
        solve_generic(start, end)
    }
}

pub mod part2 {
    use super::{gm, parse, solve_generic, FIRST_FLOOR, LAST_FLOOR};

    pub fn solve(input: &str) -> usize {
        let (mut start, mut end) = parse(input);
        start.items += 2 * gm(FIRST_FLOOR, FIRST_FLOOR);
        end.items += 2 * gm(LAST_FLOOR, LAST_FLOOR);
        solve_generic(start, end)
    }
}

pub fn main(test: bool) {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_11_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
