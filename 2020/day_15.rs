use std::{collections::HashMap, time::Instant};

fn parse(input: &str) -> HashMap<usize, usize> {
    input
        .split(',')
        .enumerate()
        .map(|(i, n)| (n.parse().unwrap(), i + 1))
        .collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let mut turns = parse(input);
        let mut number = 0;
        let mut turn = turns.len() + 1;
        while turn < 2020 {
            let previous = turns.get(&number).unwrap_or(&turn);
            let next_number = turn - previous;
            turns.insert(number, turn);
            turn += 1;
            number = next_number;
        }
        number
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let mut turns = parse(input);
        let mut number = 0;
        let mut turn = turns.len() + 1;
        while turn < 30000000 {
            let previous = turns.get(&number).unwrap_or(&turn);
            let next_number = turn - previous;
            turns.insert(number, turn);
            turn += 1;
            number = next_number;
        }
        number
    }
}

pub fn main(test: bool) {
    let test_input = "0,3,6".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
