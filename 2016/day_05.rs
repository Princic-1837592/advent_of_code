//! https://adventofcode.com/2016/day/5
//! https://adventofcode.com/2016/day/5/input

use std::{fs::read_to_string, time::Instant};

const CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub mod part1 {
    use md5::compute;
    use rayon::prelude::*;

    use super::CHARS;

    pub fn solve(input: &str) -> String {
        let result: String = (0..20_000_000)
            .into_par_iter()
            .flat_map(|i| {
                let digest = compute(format!("{}{}", input, i));
                ((((digest[0] as u32) << 16) | ((digest[1] as u32) << 8) | (digest[2] as u32))
                    & 0b11111111_11111111_11110000
                    == 0)
                    .then(|| CHARS[(digest[2] & 0b1111) as usize])
            })
            .collect();
        result.chars().take(8).collect()
    }
}

pub mod part2 {
    use md5::compute;
    use rayon::prelude::*;

    use super::CHARS;

    pub fn solve(input: &str) -> String {
        let input = input.to_owned();
        let hashes: Vec<_> = (0..40_000_000)
            .into_par_iter()
            .flat_map(|i| {
                let digest = compute(format!("{}{}", input, i));
                if (((digest[0] as u32) << 16) | ((digest[1] as u32) << 8) | (digest[2] as u32))
                    & 0b11111111_11111111_11110000
                    == 0
                {
                    let index = digest[2] & 0b1111;
                    if index < 10 {
                        return Some((index as usize, CHARS[(digest[3] >> 4) as usize]));
                    }
                }
                None
            })
            .collect();
        let mut result = [' '; 8];
        for (i, char) in hashes {
            if i < result.len() && result[i] == ' ' {
                result[i] = char
            }
        }
        result.iter().collect()
    }
}

pub fn main(test: bool) {
    let test_input = "abc".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2016/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
