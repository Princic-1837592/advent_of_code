//! https://adventofcode.com/2015/day/5
//! https://adventofcode.com/2015/day/5/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

pub mod part1 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| {
                let rules = line
                    .chars()
                    .fold(
                        ((0, false, false), ' '),
                        |((mut vowels, mut twice, mut contains), prev), char| {
                            if vowels < 3 && char == 'a'
                                || char == 'e'
                                || char == 'i'
                                || char == 'o'
                                || char == 'u'
                            {
                                vowels += 1;
                            }
                            if !twice && char == prev {
                                twice = true;
                            }
                            if !contains
                                && (prev == 'a' && char == 'b'
                                    || prev == 'c' && char == 'd'
                                    || prev == 'p' && char == 'q'
                                    || prev == 'x' && char == 'y')
                            {
                                contains = true;
                            }
                            ((vowels, twice, contains), char)
                        },
                    )
                    .0;
                rules.0 >= 3 && rules.1 && !rules.2
            })
            .count()
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap};

    pub fn solve(input: &str) -> usize {
        let mut result = 0;
        let mut pairs = HashMap::new();
        let mut letters = HashMap::new();
        for string in input.lines() {
            pairs.clear();
            letters.clear();
            let mut first = false;
            for i in 0..string.len() - 1 {
                if pairs.contains_key(&string[i..=i + 1]) {
                    if i - pairs.get(&string[i..=i + 1]).unwrap() > 1 {
                        first = true;
                        break;
                    }
                } else {
                    pairs.insert(&string[i..=i + 1], i);
                }
            }
            if !first {
                continue;
            }
            let mut second = false;
            for (i, char) in string.chars().enumerate() {
                if let Entry::Vacant(e) = letters.entry(char) {
                    e.insert(i);
                } else if i - letters.get(&char).unwrap() == 2 {
                    second = true;
                    break;
                } else {
                    letters.insert(char, i);
                }
            }
            if second {
                result += 1;
            }
        }
        result
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_05_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
