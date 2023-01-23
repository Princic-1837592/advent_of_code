//! https://adventofcode.com/2015/day/14

use std::time::Instant;

fn parse(input: &str) -> Vec<(usize, usize, usize)> {
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
    use crate::day_14::parse;

    pub fn solve(input: &str) -> usize {
        let reindeer = parse(input);
        reindeer
            .iter()
            .map(|(speed, fly, rest)| {
                speed * fly * (2503 / (fly + rest)) + speed * ((2503 % (fly + rest)).min(*fly))
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use crate::day_14::parse;

    pub fn solve(input: &str) -> usize {
        let reindeer = parse(input);
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
            for (i, _) in positions
                .iter()
                .filter(|(_, (_, distance, ..))| *distance == max_distance)
            {
                states[*i].2 += 1
            }
        }
        states
            .iter()
            .max_by_key(|&(_, _, points, ..)| points)
            .unwrap()
            .2
    }
}

pub fn main(test: bool) {
    let test_input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
