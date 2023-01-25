//! https://adventofcode.com/2015/day/4
//! https://adventofcode.com/2015/day/4/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use md5::compute;

    pub fn solve(input: &str) -> usize {
        let input = input.to_owned();
        let mut hash;
        let mut i = 0;
        loop {
            hash = format!("{:?}", compute(input.clone() + &*i.to_string()));
            if hash.starts_with("00000") {
                return i;
            }
            i += 1;
        }
    }
}

pub mod part2 {
    use md5::compute;

    pub fn solve(input: &str) -> usize {
        let input = input.to_owned();
        let mut hash;
        let mut i = 0;
        loop {
            hash = format!("{:?}", compute(input.clone() + &*i.to_string()));
            if hash.starts_with("000000") {
                return i;
            }
            i += 1;
        }
    }
}

pub fn main(test: bool) {
    let test_input = "abcdef".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_04_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
