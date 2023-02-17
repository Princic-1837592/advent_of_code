//! https://adventofcode.com/2017/day/5
//! https://adventofcode.com/2017/day/5/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub mod part1 {
    use crate::day_05::parse;

    pub fn solve(input: &str) -> usize {
        let mut jumps = parse(input);
        let mut steps = 0;
        let mut ip = 0;
        while (ip as usize) < jumps.len() {
            steps += 1;
            jumps[ip as usize] += 1;
            ip += jumps[ip as usize] - 1;
        }
        steps
    }
}

pub mod part2 {
    use crate::day_05::parse;

    pub fn solve(input: &str) -> usize {
        let mut jumps = parse(input);
        let mut steps = 0;
        let mut ip = 0;
        while (ip as usize) < jumps.len() {
            steps += 1;
            let offset = if jumps[ip as usize] >= 3 { -1 } else { 1 };
            jumps[ip as usize] += offset;
            ip += jumps[ip as usize] - offset;
        }
        steps
    }
}

pub fn main(test: bool) {
    let test_input = "0
3
0
1
-3"
    .to_owned()
    .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
