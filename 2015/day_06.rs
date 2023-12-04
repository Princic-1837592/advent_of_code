//! https://adventofcode.com/2015/day/6
//! https://adventofcode.com/2015/day/6/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    On,
    Off,
    Toggle,
}

type Instruction = (Action, (usize, usize), (usize, usize));

type Parsed = Vec<Instruction>;

fn parse(input: &str) -> Parsed {
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
    use super::{Action, Parsed};

    pub fn solve(parsed: Parsed) -> usize {
        let mut lights = vec![[false; 1000]; 1000];
        let mut lit = 0;
        for instr in parsed {
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
    use super::{Action, Parsed};

    pub fn solve(parsed: Parsed) -> usize {
        let mut lights = vec![[0; 1000]; 1000];
        let mut lit = 0;
        for instr in parsed {
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

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_06_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
