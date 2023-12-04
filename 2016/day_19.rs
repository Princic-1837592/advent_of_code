//! https://adventofcode.com/2016/day/19
//! https://adventofcode.com/2016/day/19/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug, Default)]
struct Elf {
    prev: usize,
    next: usize,
    presents: usize,
}

fn parse(input: &str) -> Vec<Elf> {
    let elves = input.parse().unwrap();
    let mut result = vec![Elf::default(); elves];
    for (i, elf) in result.iter_mut().enumerate() {
        elf.prev = if i == 0 { elves } else { i } - 1;
        elf.next = (i + 1) % elves;
        elf.presents = 1;
    }
    result
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let mut elves = parse(input);
        let mut current = 0;
        while elves[current].next != current {
            let next = elves[current].next;
            let next_next = elves[next].next;
            elves[current].presents += elves[next].presents;
            elves[next].presents = 0;
            elves[current].next = next_next;
            elves[next_next].prev = current;
            current = next_next;
        }
        (current + 1) % elves.len()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let elves = input.parse().unwrap();
        let split = elves / 2;
        let mut left = VecDeque::with_capacity(split);
        let mut right = VecDeque::with_capacity(split + 1);
        for e in 0..elves {
            if e < split {
                left.push_back(e);
            } else {
                right.push_front(e);
            }
        }
        while !left.is_empty() && !right.is_empty() {
            if left.len() > right.len() {
                left.pop_back();
            } else {
                right.pop_back();
            }
            right.push_front(left.pop_front().unwrap());
            left.push_back(right.pop_back().unwrap());
        }
        (left
            .pop_front()
            .unwrap_or_else(|| right.pop_front().unwrap())
            + 1)
            % elves
    }
}

pub fn main(test: bool) {
    let test_input = "5".to_owned();
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
