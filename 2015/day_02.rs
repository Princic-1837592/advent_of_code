//! https://adventofcode.com/2015/day/2
//! https://adventofcode.com/2015/day/2/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split('x').map(|n| n.parse().unwrap());
            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .collect()
}

pub mod part1 {
    use super::Parsed;

    fn find_wrapping((l, w, h): &(usize, usize, usize)) -> usize {
        2 * l * w + 2 * w * h + 2 * h * l + (l * w).min((w * h).min(h * l))
    }

    pub fn solve(_input: &str, parsed: Parsed) -> usize {
        parsed.iter().map(find_wrapping).sum()
    }
}

pub mod part2 {
    use super::Parsed;

    fn find_ribbon((l, w, h): &(usize, usize, usize)) -> usize {
        let total = l + w + h;
        let perimeter = total - l.max(w.max(h));
        perimeter * 2 + l * w * h
    }

    pub fn solve(_input: &str, parsed: Parsed) -> usize {
        parsed.iter().map(find_ribbon).sum()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "2x3x4".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
