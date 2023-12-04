//! https://adventofcode.com/2016/day/14
//! https://adventofcode.com/2016/day/14/input

use std::{fs::read_to_string, time::Instant};

use md5::compute;
use rayon::prelude::*;

#[allow(clippy::ptr_arg)]
fn find_hash(input: &String, i: usize, levels: usize) -> Vec<char> {
    let mut hash = input.clone() + &*i.to_string();
    for _ in 0..levels {
        hash = format!("{:?}", compute(hash));
    }
    hash.chars().collect()
}

fn find_64_keys(input: String, levels: usize) -> usize {
    let mut found = 0;
    let mut hashes = vec![vec![]; 100_000];
    for i in 0..100_000 {
        if hashes[i].is_empty() {
            hashes[i] = find_hash(&input, i, levels);
        }
        let chars = &hashes[i];
        'outer: for j in 0..chars.len() - 2 {
            if chars[j + 1] == chars[j] && chars[j + 2] == chars[j] {
                let target = chars[j];
                let new_hashes: Vec<_> = (i + 1..=i + 1000)
                    .into_par_iter()
                    .map(|h| {
                        if hashes[h].is_empty() {
                            find_hash(&input, h, levels)
                        } else {
                            vec![]
                        }
                    })
                    .collect();
                for (h, next_chars) in hashes.iter_mut().enumerate().skip(i + 1).take(1000) {
                    if next_chars.is_empty() {
                        *next_chars = new_hashes[h - i - 1].clone();
                    }
                    for k in 0..next_chars.len() - 4 {
                        if next_chars[k] == target
                            && next_chars[k + 1] == next_chars[k]
                            && next_chars[k + 2] == next_chars[k]
                            && next_chars[k + 3] == next_chars[k]
                            && next_chars[k + 4] == next_chars[k]
                        {
                            found += 1;
                            if found == 64 {
                                return i;
                            }
                            break 'outer;
                        }
                    }
                }
                break;
            }
        }
    }
    unreachable!()
}

pub mod part1 {
    use super::find_64_keys;

    pub fn solve(input: &str) -> usize {
        let input = input.to_owned();
        find_64_keys(input, 1)
    }
}

pub mod part2 {
    use super::find_64_keys;

    pub fn solve(input: &str) -> usize {
        let input = input.to_owned();
        find_64_keys(input, 2017)
    }
}

pub fn main(test: bool) {
    let test_input = "abc".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
