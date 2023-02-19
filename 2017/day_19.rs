//! https://adventofcode.com/2017/day/19
//! https://adventofcode.com/2017/day/19/input

use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn parse(input: &str) -> HashMap<(isize, isize), char> {
    let mut result = HashMap::new();
    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char != ' ' {
                result.insert((i as isize, j as isize), char);
            }
        }
    }
    result
}

fn move_packet(map: HashMap<(isize, isize), char>) -> (String, usize) {
    let (mut x, mut y) = (0, 0);
    for j in 0.. {
        if let Some('|') = map.get(&(0, j)) {
            (x, y) = (0, j);
            break;
        }
    }
    let mut steps = 0;
    let (mut dx, mut dy) = (1, 0);
    let mut result = String::with_capacity(9);
    while let Some(char) = map.get(&(x, y)) {
        match char {
            'A'..='Z' => result.push(*char),
            '+' => {
                for direction @ (ndx, ndy) in [(dy, dx), (-dy, -dx)] {
                    let next = (x + ndx, y + ndy);
                    if map.get(&next).is_some() {
                        (dx, dy) = direction;
                        break;
                    }
                }
            }
            _ => {}
        }
        (x, y) = (x + dx, y + dy);
        steps += 1;
    }
    (result, steps)
}

pub mod part1 {
    use crate::day_19::{move_packet, parse};

    pub fn solve(input: &str) -> String {
        let map = parse(input);
        move_packet(map).0
    }
}

pub mod part2 {
    use crate::day_19::{move_packet, parse};

    pub fn solve(input: &str) -> usize {
        let map = parse(input);
        move_packet(map).1
    }
}

pub fn main(test: bool) {
    let test_input = "    |
    |  +--+
    A  |  C
F---|----E|--+
    |  |  |  D
    +B-+  +--+"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_19_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
