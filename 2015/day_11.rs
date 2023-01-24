//! https://adventofcode.com/2015/day/11

use std::{fs::read_to_string, time::Instant};

const NEXT: [usize; 26] = [
    1, 2, 3, 4, 5, 6, 7, 9, 0, 10, 12, 12, 13, 15, 0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0,
];

type Password = Vec<usize>;

fn parse(input: &str) -> Password {
    input
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
    use crate::day_11::{find_next_nth, parse, to_string};

    pub fn solve(input: &str) -> String {
        let mut password = parse(input);
        find_next_nth(&mut password, 1);
        to_string(&password)
    }
}

pub mod part2 {
    use crate::day_11::{find_next_nth, parse, to_string};

    pub fn solve(input: &str) -> String {
        let mut password = parse(input);
        find_next_nth(&mut password, 2);
        to_string(&password)
    }
}

pub fn main(test: bool) {
    let test_input = "abcdefgh".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_11_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
