//! https://adventofcode.com/2016/day/7
//! https://adventofcode.com/2016/day/7/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub mod part1 {
    use crate::day_07::parse;

    pub fn solve(input: &str) -> usize {
        let ips = parse(input);
        let mut valid = 0;
        for ip in ips {
            if ip
                .chars()
                .fold(
                    (None, ' ', ' ', ' ', false),
                    |acc @ (valid, first, second, third, is_hypernet), char| {
                        if let Some(false) = valid {
                            acc
                        } else if char == '[' {
                            (valid, second, third, char, true)
                        } else if first == ']' {
                            (valid, second, third, char, false)
                        } else if first == char && second == third && first != second {
                            (Some(!is_hypernet), ' ', ' ', ' ', is_hypernet)
                        } else {
                            (valid, second, third, char, is_hypernet)
                        }
                    },
                )
                .0
                .unwrap_or(false)
            {
                valid += 1
            }
        }
        valid
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use crate::day_07::parse;

    pub fn solve(input: &str) -> usize {
        let ips = parse(input);
        let mut valid = 0;
        for ip in ips {
            let (_, _, _, in_sequences, out_sequences) = ip.chars().fold(
                (' ', ' ', false, HashSet::new(), HashSet::new()),
                |(first, second, is_hypernet, mut in_sequences, mut out_sequences), char| {
                    if char == '[' {
                        (second, char, true, in_sequences, out_sequences)
                    } else if first == ']' {
                        (second, char, false, in_sequences, out_sequences)
                    } else if first == char && first != second {
                        if is_hypernet {
                            in_sequences.insert(format!("{}{}{}", first, second, char));
                        } else {
                            out_sequences.insert(format!("{}{}{}", second, first, second));
                        }
                        (second, char, is_hypernet, in_sequences, out_sequences)
                    } else {
                        (second, char, is_hypernet, in_sequences, out_sequences)
                    }
                },
            );
            if in_sequences.intersection(&out_sequences).count() > 0 {
                valid += 1
            }
        }
        valid
    }
}

pub fn main(test: bool) {
    let test_input = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_07_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
