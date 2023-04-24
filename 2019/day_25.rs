//! https://adventofcode.com/2019/day/25
//! https://adventofcode.com/2019/day/25/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use itertools::Itertools;
    use regex::Regex;

    use crate::int_code::parse;

    pub fn solve(input: &str) -> usize {
        let mut vm = parse(input);
        let take_items = r"west
take hypercube
west
take space law space brochure
west
north
take shell
west
take mug
south
take festive hat
north
east
south
east
east
east
east
north
west
north
take whirled peas
west
west
take astronaut ice cream
south
drop astronaut ice cream
drop whirled peas
drop festive hat
drop mug
drop shell
drop space law space brochure
drop hypercube
";
        let items = [
            "astronaut ice cream",
            "whirled peas",
            "festive hat",
            "mug",
            "shell",
            "space law space brochure",
            "hypercube",
        ];
        take_items
            .chars()
            .for_each(|char| vm.push_input(char as i64));
        vm.run_until_complete();
        for k in 1..=items.len() {
            if let Some(result) = items
                .iter()
                .combinations(k)
                .filter_map(|items| {
                    let mut vm = vm.clone();
                    items
                        .into_iter()
                        .map(|item| format!("take {}\n", item))
                        .join("")
                        .chars()
                        .for_each(|char| vm.push_input(char as i64));
                    "south\n"
                        .chars()
                        .for_each(|char| vm.push_input(char as i64));
                    vm.run_until_complete();
                    let result: String = vm
                        .get_output()
                        .iter()
                        .rev()
                        .take(500)
                        .rev()
                        .map(|&char| char as u8 as char)
                        .collect();
                    if !result.contains("ejected back to the checkpoint") {
                        let pattern = Regex::new(r"typing (\d+) on the keypad").unwrap();
                        return Some(
                            pattern
                                .captures(&result)
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .as_str()
                                .parse()
                                .unwrap(),
                        );
                    }
                    None
                })
                .next()
            {
                return result;
            }
        }
        unreachable!();
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
