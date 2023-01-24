//! https://adventofcode.com/2015/day/12

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use regex::Regex;

    pub fn solve(input: &str) -> isize {
        let pattern = Regex::new(r"-?\d+").unwrap();
        pattern
            .find_iter(input)
            .map(|number| number.as_str().parse::<isize>().unwrap())
            .sum()
    }
}

pub mod part2 {
    use itertools::Itertools;
    use serde_json::Value;

    fn explore(json: &Value) -> isize {
        if json.is_array() {
            json.as_array().unwrap().iter().map(explore).sum()
        } else if json.is_object() {
            let json = json.as_object().unwrap();
            if json.values().contains(&Value::String(String::from("red"))) {
                0
            } else {
                json.values().map(explore).sum()
            }
        } else if json.is_i64() {
            json.as_i64().unwrap() as isize
        } else {
            0
        }
    }

    pub fn solve(input: &str) -> isize {
        let json: Value = serde_json::from_str(input).unwrap();
        explore(&json)
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
