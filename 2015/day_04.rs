//! https://adventofcode.com/2015/day/4
//! https://adventofcode.com/2015/day/4/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use md5::compute;

    pub fn solve(input: &str) -> usize {
        for i in 0.. {
            let digest = compute(format!("{}{}", input, i));
            if (((digest[0] as u32) << 16) | ((digest[1] as u32) << 8) | (digest[2] as u32))
                < 0b00000000_00000000_00010000
            {
                return i;
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use md5::compute;
    use rayon::prelude::*;

    pub fn solve(input: &str) -> usize {
        (0..=10_000_000)
            .into_par_iter()
            .filter(|i| {
                let digest = compute(format!("{}{}", input, i));
                (((digest[0] as u32) << 16) | ((digest[1] as u32) << 8) | (digest[2] as u32)) == 0
            })
            .min()
            .unwrap()
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
