//! https://adventofcode.com/2018/day/2
//! https://adventofcode.com/2018/day/2/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let (twos, threes): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let mut map = HashMap::new();
                for letter in line.chars() {
                    *map.entry(letter).or_insert(0) += 1;
                }
                (map.values().any(|&v| v == 2), map.values().any(|v| *v == 3))
            })
            .unzip();
        twos.iter().filter(|id| **id).count() * threes.iter().filter(|id| **id).count()
    }
}

pub mod part2 {
    use itertools::Itertools;

    pub fn solve(input: &str) -> String {
        input
            .lines()
            .cartesian_product(input.lines())
            .filter_map(|(left, right)| {
                let mut diffs = 0;
                let mut index = 0;
                let mut right = right.chars();
                for (i, left) in left.chars().enumerate() {
                    if left != right.next().unwrap() {
                        diffs += 1;
                        index = i;
                    }
                }
                (diffs == 1).then_some({
                    let mut result = String::from(&left[0..index]);
                    result.push_str(&left[index + 1..]);
                    result
                })
            })
            .next()
            .unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
