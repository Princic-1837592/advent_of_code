//! https://adventofcode.com/2016/day/13
//! https://adventofcode.com/2016/day/13/input

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

fn parse(input: &str) -> usize {
    input.parse().unwrap()
}

fn bfs(number: usize, target: (usize, usize), max_distance: usize) -> (usize, usize) {
    let mut queue = VecDeque::from([(0, (1, 1))]);
    let mut visited = HashSet::new();
    while let Some((mut distance, coord @ (x, y))) = queue.pop_front() {
        if distance > max_distance {
            continue;
        }
        if coord == target {
            return (distance, visited.len());
        }
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);
        distance += 1;
        if is_open(x + 1, y, number) {
            queue.push_back((distance, (x + 1, y)));
        }
        if is_open(x, y + 1, number) {
            queue.push_back((distance, (x, y + 1)));
        }
        if x > 0 && is_open(x - 1, y, number) {
            queue.push_back((distance, (x - 1, y)));
        }
        if y > 0 && is_open(x, y - 1, number) {
            queue.push_back((distance, (x, y - 1)));
        }
    }
    (usize::MAX, visited.len())
}

fn is_open(x: usize, y: usize, number: usize) -> bool {
    let sum = x * x + 3 * x + 2 * x * y + y + y * y + number;
    let mut mask = 1;
    let mut bits = 0;
    while mask > 0 {
        if sum & mask != 0 {
            bits += 1;
        }
        mask <<= 1
    }
    bits % 2 == 0
}

pub mod part1 {
    use super::{bfs, parse};

    pub fn solve(input: &str) -> usize {
        let number = parse(input);
        bfs(number, (31, 39), usize::MAX).0
    }
}

pub mod part2 {
    use super::{bfs, parse};

    pub fn solve(input: &str) -> usize {
        let number = parse(input);
        bfs(number, (52, 0), 50).1
    }
}

pub fn main(test: bool) {
    let test_input = "10".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
