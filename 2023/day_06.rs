//! https://adventofcode.com/2023/day/6
//! https://adventofcode.com/2023/day/6/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

pub mod part1 {
    use super::Race;

    type Parsed = Vec<Race>;

    pub(crate) fn parse(input: &str) -> Parsed {
        let mut lines = input.lines();
        let time = lines.next().unwrap();
        let distance = lines.next().unwrap();
        time.split_whitespace()
            .zip(distance.split_whitespace())
            .skip(1)
            .map(|(time, distance)| Race {
                time: time.parse().unwrap(),
                distance: distance.parse().unwrap(),
            })
            .collect()
    }

    fn find_min_win_time(race: &Race) -> usize {
        (0..=race.time)
            .find(|t| t * (race.time - t) > race.distance)
            .unwrap()
    }

    pub fn solve(races: Parsed) -> usize {
        races
            .into_iter()
            .map(|race| {
                let min = find_min_win_time(&race);
                race.time + 1 - min * 2
            })
            .product()
    }
}

pub mod part2 {
    use std::ops::Add;

    use super::Race;

    type Parsed = Race;

    pub(crate) fn parse(input: &str) -> Parsed {
        let mut lines = input.lines();
        let time = lines.next().unwrap();
        let distance = lines.next().unwrap();
        Race {
            time: time
                .split_whitespace()
                .to_owned()
                .skip(1)
                .fold("".to_owned(), Add::add)
                .parse()
                .unwrap(),
            distance: distance
                .split_whitespace()
                .to_owned()
                .skip(1)
                .fold("".to_owned(), Add::add)
                .parse()
                .unwrap(),
        }
    }

    pub(crate) fn solve(race: Parsed) -> usize {
        let mut size = race.time / 2;
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let distance = mid * (race.time - mid);
            if distance > race.distance {
                right = mid;
            } else {
                left = mid + 1;
            }
            size = right - left;
        }
        race.time + 1 - left * 2
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "Time:      7  15   30
Distance:  9  40  200"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_06_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = part1::parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let parsed = part2::parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
