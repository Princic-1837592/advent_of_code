//! https://adventofcode.com/2015/day/14
//! https://adventofcode.com/2015/day/14/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            (
                parts[3].parse().unwrap(),
                parts[6].parse().unwrap(),
                parts[13].parse().unwrap(),
            )
        })
        .collect()
}

pub mod part1 {
    use crate::day_14::Parsed;

    pub fn solve(_input: &str, reindeer: Parsed) -> usize {
        reindeer
            .iter()
            .map(|(speed, fly, rest)| {
                let cycle = fly + rest;
                speed * fly * (2503 / cycle) + speed * ((2503 % cycle).min(*fly))
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use crate::day_14::Parsed;

    pub fn solve(_input: &str, reindeer: Parsed) -> usize {
        let mut states: Vec<_> = reindeer
            .iter()
            .map(|&(_, fly, rest)| (true, 0, 0, fly, rest))
            .collect();
        for _ in 0..2503 {
            for (i, (flying, distance, _, fly, rest)) in states.iter_mut().enumerate() {
                if *flying {
                    *fly -= 1;
                    *distance += reindeer[i].0;
                    if *fly == 0 {
                        *fly = reindeer[i].1;
                        *flying = false;
                    }
                } else {
                    *rest -= 1;
                    if *rest == 0 {
                        *rest = reindeer[i].2;
                        *flying = true;
                    }
                }
            }
            let mut positions: Vec<_> = states.iter().cloned().enumerate().collect();
            positions.sort_by_key(|(_, (_, distance, ..))| *distance);
            let max_distance = positions[positions.len() - 1].1 .1;
            positions
                .iter()
                .filter(|(_, (_, distance, ..))| *distance == max_distance)
                .for_each(|(i, _)| states[*i].2 += 1)
        }
        states
            .iter()
            .max_by_key(|&(_, _, points, ..)| points)
            .unwrap()
            .2
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_14_input.txt").unwrap()
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
