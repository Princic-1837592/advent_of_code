//! https://adventofcode.com/2016/day/5
//! https://adventofcode.com/2016/day/5/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use md5::compute;
    use rayon::prelude::*;

    pub fn solve(input: &str) -> String {
        let input = input.to_owned();
        let result: String = (0..20_000_000)
            .into_par_iter()
            .flat_map(|i| {
                let hash = format!("{:?}", compute(input.clone() + &*i.to_string()));
                hash.starts_with("00000")
                    .then_some(hash.chars().nth(5).unwrap())
            })
            .collect();
        result.chars().take(8).collect()
    }
}

pub mod part2 {
    use md5::compute;
    use rayon::prelude::*;

    pub fn solve(input: &str) -> String {
        let input = input.to_owned();
        let mut result = [' '; 8];
        let hashes: Vec<_> = (0..200_000_000)
            .into_par_iter()
            .flat_map(|i| {
                let hash = format!("{:?}", compute(input.clone() + &*i.to_string()));
                let mut chars = hash.chars();
                let index = chars.nth(5).unwrap();
                (hash.starts_with("00000") && index.is_ascii_digit() && index < '8')
                    .then(|| (index.to_digit(10).unwrap() as usize, chars.next().unwrap()))
            })
            .collect();
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
        read_to_string("inputs/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
