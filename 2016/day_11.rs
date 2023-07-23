//! https://adventofcode.com/2016/day/11
//! https://adventofcode.com/2016/day/11/input

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
    time::Instant,
};

use regex::Regex;

const UP: usize = 0;
const DOWN: usize = 1;
const GENERATOR: usize = 0;
const MICROCHIP: usize = 1;
const FIRST_FLOOR: usize = 0;
const LAST_FLOOR: usize = 3;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    items: usize,
    elevator: usize,
}

fn gm(generator: usize, microchip: usize) -> usize {
    1 << (generator << 4) << (microchip << 2)
}

fn parse(input: &str) -> (State, State) {
    #[derive(Debug, Clone, Copy)]
    struct Pair {
        generator: usize,
        microchip: usize,
    }

    let mut elements: HashMap<&str, Pair> = HashMap::new();
    let pattern = Regex::new(r"a (\w+)( generator|-compatible microchip)").unwrap();
    for (floor, line) in input.lines().enumerate() {
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
        elevator: FIRST_FLOOR,
    };
    let mut end = State {
        items: 0,
        elevator: LAST_FLOOR,
    };
    for pair in elements.values() {
        start.items += gm(pair.generator, pair.microchip);
        end.items += gm(LAST_FLOOR, LAST_FLOOR);
    }
    (start, end)
}

fn udgmo(up_down: usize, gen_or_micro: usize, other: usize) -> usize {
    (up_down << 3) | (gen_or_micro << 2) | other
}

fn move_table() -> [[usize; 16]; 4] {
    let mut result = [[0x8888888888888888; 16]; 4];
    for (floor, row) in result.iter_mut().enumerate() {
        for other in FIRST_FLOOR..=LAST_FLOOR {
            if floor > FIRST_FLOOR {
                row[udgmo(DOWN, GENERATOR, other)] =
                    gm(floor - 1, other).wrapping_sub(gm(floor, other));
                row[udgmo(DOWN, MICROCHIP, other)] =
                    gm(other, floor - 1).wrapping_sub(gm(other, floor));
            }
            if floor < LAST_FLOOR {
                row[udgmo(UP, GENERATOR, other)] =
                    gm(floor + 1, other).wrapping_sub(gm(floor, other));
                row[udgmo(UP, MICROCHIP, other)] =
                    gm(other, floor + 1).wrapping_sub(gm(other, floor));
            }
        }
    }
    result
}

fn legal(state: usize) -> bool {
    state & 0x8888888888888888 == 0
}

fn compatible(state: usize) -> bool {
    !(state & 0x000000000000ffff != 0 && state & 0x000f000f000f0000 != 0
        || state & 0x00000000ffff0000 != 0 && state & 0x00f000f0000000f0 != 0
        || state & 0x0000ffff00000000 != 0 && state & 0x0f0000000f000f00 != 0
        || state & 0xffff000000000000 != 0 && state & 0x0000f000f000f000 != 0)
}

fn sign(depth: isize) -> isize {
    (depth * 2 + 1).signum()
}

fn solve_generic(start: State, end: State) -> usize {
    let move_table = move_table();
    let (mut prev, mut curr, mut next) = (
        HashMap::new(),
        HashMap::from([(start, 0), (end, -1)]),
        HashMap::new(),
    );
    for _ in 0..1000 {
        for (&State { items, elevator }, &depth) in &curr {
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
                    let mut contained;
                    if !{
                        contained = prev.get(&state);
                        contained.is_some()
                    } && !{
                        contained = curr.get(&state);
                        contained.is_some()
                    } && !{
                        contained = next.get(&state);
                        contained.is_some()
                    } {
                        next.insert(state, depth + sign(depth));
                    } else if sign(depth) != sign(*contained.unwrap()) {
                        return depth.unsigned_abs() + contained.unwrap().unsigned_abs();
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
    use crate::day_11::{parse, solve_generic};

    pub fn solve(input: &str) -> usize {
        let (start, end) = parse(input);
        solve_generic(start, end)
    }
}

pub mod part2 {
    use crate::day_11::{gm, parse, solve_generic, FIRST_FLOOR, LAST_FLOOR};

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
