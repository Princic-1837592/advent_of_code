//! https://adventofcode.com/2017/day/6
//! https://adventofcode.com/2017/day/6/input

use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|blocks| blocks.parse().unwrap())
        .collect()
}

fn to_number(memory: &Vec<usize>) -> usize {
    let mut sum = 0;
    for (i, bank) in memory.iter().enumerate() {
        sum += bank * memory.len().pow(i as u32);
    }
    sum
}

fn find_loop(memory: &mut Vec<usize>) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut cycles = 0;
    loop {
        let number = to_number(memory);
        if let Some(seen) = seen.get(&number) {
            return (cycles, cycles - seen);
        }
        seen.insert(number, cycles);
        let max = memory
            .iter()
            .enumerate()
            .max_by_key(|&(i, &blocks)| (blocks, -(i as isize)))
            .unwrap()
            .0;
        let mut blocks = memory[max];
        memory[max] = 0;
        let mut b = max + 1;
        while blocks > 0 {
            if b >= memory.len() {
                b = 0;
            }
            memory[b] += 1;
            b += 1;
            blocks -= 1;
        }
        cycles += 1;
    }
}

pub mod part1 {
    use super::{find_loop, parse};

    pub fn solve(input: &str) -> usize {
        let mut memory = parse(input);
        find_loop(&mut memory).0
    }
}

pub mod part2 {
    use super::{find_loop, parse};

    pub fn solve(input: &str) -> usize {
        let mut memory = parse(input);
        find_loop(&mut memory).1
    }
}

pub fn main(test: bool) {
    let test_input = "0	2	7	0".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_06_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
