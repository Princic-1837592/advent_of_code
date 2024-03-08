use std::{fs::read_to_string, time::Instant};

const SUBJECT_NUMBER: usize = 7;
const MAGIC_NUMBER: usize = 20201227;

fn parse(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn find_loop_size(target: usize) -> usize {
    let mut number = 1;
    let mut iterations = 0;
    while number != target {
        number *= SUBJECT_NUMBER;
        number %= MAGIC_NUMBER;
        iterations += 1;
    }
    iterations
}

fn find_private(subject_number: usize, loop_size: usize) -> usize {
    let mut number = 1;
    for _ in 0..loop_size {
        number *= subject_number;
        number %= MAGIC_NUMBER;
    }
    number
}

pub mod part1 {
    use super::{find_loop_size, find_private, parse};

    pub fn solve(input: &str) -> usize {
        let (card, door) = parse(input);
        let card_loop = find_loop_size(card);
        find_private(door, card_loop)
    }
}

pub fn main(test: bool) {
    let test_input = "5764801
17807724"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
