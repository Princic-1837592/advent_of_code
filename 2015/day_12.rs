//! https://adventofcode.com/2015/day/12
//! https://adventofcode.com/2015/day/12/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

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

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_12_input.txt").unwrap()
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
