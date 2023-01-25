//! https://adventofcode.com/2015/day/6
//! https://adventofcode.com/2015/day/6/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

enum Action {
    On,
    Off,
    Toggle,
}

type Instruction = (Action, (usize, usize), (usize, usize));

fn parse(input: &str) -> Vec<Instruction> {
    let pattern = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                match line.chars().nth(6).unwrap() {
                    'n' => Action::On,
                    'f' => Action::Off,
                    ' ' => Action::Toggle,
                    _ => panic!("Invalid instruction: {}", line),
                },
                (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}

pub mod part1 {
    use crate::day_06::{parse, Action};

    pub fn solve(input: &str) -> usize {
        let mut lights = vec![[false; 1000]; 1000];
        let mut lit = 0;
        for instr in parse(input) {
            match instr {
                (Action::On, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            if !*light {
                                *light = true;
                                lit += 1;
                            }
                        }
                    }
                }
                (Action::Off, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            if *light {
                                *light = false;
                                lit -= 1;
                            }
                        }
                    }
                }
                (Action::Toggle, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            *light = !*light;
                            if !*light {
                                lit -= 1
                            } else {
                                lit += 1
                            }
                        }
                    }
                }
            }
        }
        lit
    }
}

pub mod part2 {
    use crate::day_06::{parse, Action};

    pub fn solve(input: &str) -> usize {
        let mut lights = vec![[0; 1000]; 1000];
        let mut lit = 0;
        for instr in parse(input) {
            match instr {
                (Action::On, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            *light += 1;
                            lit += 1;
                        }
                    }
                }
                (Action::Off, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            if *light >= 1 {
                                *light -= 1;
                                lit -= 1;
                            }
                        }
                    }
                }
                (Action::Toggle, (left, top), (right, bottom)) => {
                    for row in &mut lights[left..=right] {
                        for light in &mut row[top..=bottom] {
                            *light += 2;
                            lit += 2;
                        }
                    }
                }
            }
        }
        lit
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_06_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
