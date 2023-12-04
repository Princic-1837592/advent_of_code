//! https://adventofcode.com/2016/day/6
//! https://adventofcode.com/2016/day/6/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_message(messages: Vec<Vec<char>>, coefficient: isize) -> String {
    let mut result = String::with_capacity(messages[0].len());
    for i in 0..messages[0].len() {
        let mut frequencies = [0; 26];
        for message in &messages {
            frequencies[message[i] as usize - 'a' as usize] += 1;
        }
        result.push(
            (frequencies
                .iter()
                .enumerate()
                .max_by_key(|(_, &f)| f * coefficient)
                .unwrap()
                .0 as u8
                + b'a') as char,
        );
    }
    result
}

pub mod part1 {
    use super::{find_message, parse};

    pub fn solve(input: &str) -> String {
        let messages = parse(input);
        find_message(messages, 1)
    }
}

pub mod part2 {
    use super::{find_message, parse};

    pub fn solve(input: &str) -> String {
        let messages = parse(input);
        find_message(messages, -1)
    }
}

pub fn main(test: bool) {
    let test_input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
        .to_owned();
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
