//! https://adventofcode.com/2015/day/11
//! https://adventofcode.com/2015/day/11/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

const NEXT: [usize; 26] = [
    1, 2, 3, 4, 5, 6, 7, 9, 99, 10, 12, 99, 13, 15, 99, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0,
];

type Password = Vec<usize>;

type Parsed = Password;

fn parse(input: &str) -> Parsed {
    input
        .trim()
        .chars()
        .rev()
        .map(|char| (char as u8 - b'a') as usize)
        .collect()
}

fn next_password(password: &mut Password) {
    let mut carry = true;
    let mut i = 0;
    while carry && i < password.len() {
        password[i] = NEXT[password[i]];
        carry = password[i] == 0;
        i += 1;
    }
    if carry {
        password.push(0);
    }
}

fn is_valid(password: &Password) -> bool {
    let (first, second, ..) = password
        .iter()
        .enumerate()
        .map(|(i, &char)| (i as isize, char))
        .fold(
            (false, 0, (usize::MAX - 1, usize::MAX - 2), -2),
            |(mut first, mut second, (second_prev, prev), mut last_pair_index), (i, char)| {
                if !first {
                    first = char == prev.wrapping_sub(1) && prev == second_prev.wrapping_sub(1);
                }
                if char == prev && i - last_pair_index >= 2 {
                    second += 1;
                    last_pair_index = i;
                }
                (first, second, (prev, char), last_pair_index)
            },
        );
    first && second >= 2
}

fn find_next_nth(password: &mut Password, nth: usize) {
    for _ in 0..nth {
        next_password(password);
        while !is_valid(password) {
            next_password(password);
        }
    }
}

fn to_string(password: &Password) -> String {
    password
        .iter()
        .rev()
        .map(|&char| (char as u8 + b'a') as char)
        .collect()
}

pub mod part1 {
    use super::{find_next_nth, to_string, Parsed};

    pub fn solve(mut password: Parsed) -> String {
        find_next_nth(&mut password, 1);
        to_string(&password)
    }
}

pub mod part2 {
    use super::{find_next_nth, to_string, Parsed};

    pub fn solve(mut password: Parsed) -> String {
        find_next_nth(&mut password, 2);
        to_string(&password)
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "abcdefgh".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_11_input.txt").unwrap()
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
