//! https://adventofcode.com/2017/day/25
//! https://adventofcode.com/2017/day/25/input

use std::{collections::HashMap, fs::read_to_string, time::Instant};

use crate::LINE_ENDING;

type TM = HashMap<(u8, usize), (usize, isize, u8)>;

fn parse(input: &str) -> (TM, u8, usize) {
    let mut result = HashMap::new();
    let separator = LINE_ENDING.repeat(2);
    let mut parts = input.split(&separator);
    let mut begin = parts.next().unwrap().lines();
    let initial_state = begin
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .chars()
        .next()
        .unwrap() as u8;
    let steps = begin
        .next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();
    for state in parts {
        let mut lines = state.lines();
        let state = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .next()
            .unwrap() as u8;
        for _ in 0..2 {
            let current = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            let write = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            let movement = (lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .next()
                .unwrap() as isize
                - 'm' as isize)
                .signum();
            let next_state = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .next()
                .unwrap() as u8;
            result.insert((state, current), (write, movement, next_state));
        }
    }
    (result, initial_state, steps)
}

pub mod part1 {
    use std::collections::HashSet;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (tm, mut state, mut steps) = parse(input);
        let mut cp = 0;
        let mut ones = HashSet::new();
        while steps > 0 {
            steps -= 1;
            let &(write, movement, new_state) = tm
                .get(&(state, if ones.contains(&cp) { 1 } else { 0 }))
                .unwrap();
            if write == 1 {
                ones.insert(cp);
            } else {
                ones.remove(&cp);
            }
            cp += movement;
            state = new_state;
        }
        ones.len()
    }
}

pub fn main(test: bool) {
    let test_input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A."
        .to_owned()
        .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
