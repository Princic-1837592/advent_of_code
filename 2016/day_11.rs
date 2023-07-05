//! https://adventofcode.com/2016/day/11
//! https://adventofcode.com/2016/day/11/input

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Item {
    G(usize),
    M(usize),
}

type StateQueue = VecDeque<(usize, usize, [Vec<Item>; 4], usize)>;

fn parse(input: &str) -> [Vec<Item>; 4] {
    let pattern = Regex::new(r"(\w+)(?:-compatible)? (generator|microchip)").unwrap();
    let mut result = [vec![], vec![], vec![], vec![]];
    let mut types: Vec<&str> = vec![];
    for (i, line) in input.lines().enumerate() {
        let floor = &mut result[i];
        for capture in pattern.captures_iter(line) {
            let (name, item_type) = (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            );
            let index = if let Some(i) = types.iter().position(|&item| name == item) {
                i
            } else {
                let i = types.len();
                types.push(name);
                i
            };
            floor.push(if item_type.starts_with('g') {
                Item::G(index)
            } else {
                Item::M(index)
            });
        }
    }
    result
}

#[allow(unused)]
fn to_string(step: usize, floor: usize, state: &[Vec<Item>]) -> String {
    let mut result = String::from(&format!("{}\n", step));
    for i in (0..state.len()).rev() {
        let mut line = String::new();
        line.push('F');
        line.push_str(&(i + 1).to_string());
        line.push(' ');
        line.push_str(if floor == i { "E " } else { "  " });
        line.push_str(&format!("{:?}\n", state[i]));
        result.push_str(&line);
    }
    result
}

fn is_valid(floors: &[Vec<Item>; 4]) -> bool {
    for floor in floors {
        for i in floor.iter().filter_map(|item| {
            if let &Item::M(i) = item {
                Some(i)
            } else {
                None
            }
        }) {
            if !floor.contains(&Item::G(i))
                && floor
                    .iter()
                    .filter_map(|item| {
                        if let &Item::G(i) = item {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .any(|other| other != i)
            {
                return false;
            }
        }
    }
    true
}

fn hash_state(state: &[Vec<Item>; 4]) -> [Vec<Item>; 4] {
    let mut hash = [vec![], vec![], vec![], vec![]];
    let mut types: Vec<usize> = vec![];
    for (i, floor) in state.iter().enumerate() {
        for &item in floor {
            let id = match item {
                Item::G(i) | Item::M(i) => i,
            };
            let index = if let Some(i) = types.iter().position(|&item| id == item) {
                i
            } else {
                let i = types.len();
                types.push(id);
                i
            };
            hash[i].push(match item {
                Item::G(_) => Item::G(index),
                Item::M(_) => Item::M(index),
            });
        }
    }
    hash
}

fn solve_generic(floors: [Vec<Item>; 4]) -> usize {
    let mut queue = VecDeque::from([(0, 0, floors, 0)]);
    let mut visited = HashSet::new();
    while let Some((step, floor, state, min_floor)) = queue.pop_front() {
        let hash = hash_state(&state);
        if visited.contains(&(floor, hash.clone())) {
            continue;
        }
        visited.insert((floor, hash));
        if min_floor == state.len() - 1 {
            return step;
        }
        if floor != state.len() - 1 {
            move_up(&mut queue, step, floor, &state, min_floor);
        }
        if floor > min_floor {
            move_down(&mut queue, step, floor, &state, min_floor);
        }
    }
    unreachable!()
}

fn move_up(
    queue: &mut StateQueue,
    step: usize,
    floor: usize,
    state: &[Vec<Item>; 4],
    min_floor: usize,
) {
    let next_floor = floor + 1;
    if move_two(queue, step, floor, next_floor, min_floor, state) == 0 {
        move_one(queue, step, floor, next_floor, min_floor, state);
    }
}

fn move_down(
    queue: &mut StateQueue,
    step: usize,
    floor: usize,
    state: &[Vec<Item>; 4],
    min_floor: usize,
) {
    let next_floor = floor - 1;
    if move_one(queue, step, floor, next_floor, min_floor, state) == 0 {
        move_two(queue, step, floor, next_floor, min_floor, state);
    }
}

fn move_one(
    queue: &mut StateQueue,
    step: usize,
    floor: usize,
    next_floor: usize,
    mut min_floor: usize,
    state: &[Vec<Item>; 4],
) -> usize {
    for (i, &item) in state[floor].clone().iter().enumerate() {
        let mut next_state = state.clone();
        next_state[floor].swap_remove(i);
        next_state[next_floor].push(item);
        if next_floor > floor && floor == min_floor && next_state[floor].is_empty() {
            min_floor += 1;
        }
        if is_valid(&next_state) {
            queue.push_back((step + 1, next_floor, next_state, min_floor));
            return 1;
        }
    }
    0
}

fn move_two(
    queue: &mut StateQueue,
    step: usize,
    floor: usize,
    next_floor: usize,
    mut min_floor: usize,
    state: &[Vec<Item>; 4],
) -> usize {
    let mut pairs_moved = 0;
    for (i, &first) in state[floor].clone().iter().enumerate() {
        for (j, &second) in state[floor].clone().iter().enumerate().skip(i + 1) {
            if match (first, second) {
                (Item::M(_), Item::M(_)) | (Item::G(_), Item::G(_)) => true,
                (Item::M(i), Item::G(j)) | (Item::G(i), Item::M(j)) if i == j => true,
                _ => false,
            } {
                let mut next_state = state.clone();
                next_state[floor].swap_remove(j);
                next_state[floor].swap_remove(i);
                next_state[next_floor].push(first);
                next_state[next_floor].push(second);
                if next_floor > floor && floor == min_floor && next_state[floor].is_empty() {
                    min_floor += 1;
                }
                if is_valid(&next_state) {
                    queue.push_back((step + 1, next_floor, next_state, min_floor));
                    pairs_moved += 1;
                }
            }
        }
    }
    pairs_moved
}

pub mod part1 {
    use crate::day_11::{parse, solve_generic};

    pub fn solve(input: &str) -> usize {
        let floors = parse(input);
        solve_generic(floors)
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
    let first_part = part1::solve(&puzzle_input);
    println!("{}", first_part);
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", first_part + 12 + 12);
    println!("Run in {:?}", start.elapsed());
}
