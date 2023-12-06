//! https://adventofcode.com/2023/day/5
//! https://adventofcode.com/2023/day/5/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use crate::LINE_ENDING;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    source: usize,
    source_end: usize,
    destination: usize,
    destination_end: usize,
}

impl Range {
    fn new(destination: usize, source: usize, length: usize) -> Self {
        Range {
            source,
            source_end: source + length - 1,
            destination,
            destination_end: destination.saturating_add(length - 1),
        }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        Range::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

type Map = Vec<Range>;

type Parsed = (Vec<usize>, Vec<Map>);

fn parse(input: &str) -> Parsed {
    let separator = LINE_ENDING.repeat(2);
    let mut parts = input.split(&separator);
    let seeds = parts.next().unwrap()[6..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let maps = parts
        .map(|p| {
            let mut map: Vec<_> = p.lines().skip(1).map(Range::from).collect();
            map.sort();
            map
        })
        .collect();
    (seeds, maps)
}

pub mod part1 {
    use std::cmp::Ordering;

    use super::{Map, Parsed};

    pub fn map(src: usize, map: &Map) -> usize {
        let target = map.binary_search_by(|r| {
            if src < r.source {
                Ordering::Greater
            } else if src > r.source_end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        target
            .map(|r| {
                let range = map[r];
                range.destination + src - range.source
            })
            .unwrap_or(src)
    }

    pub fn solve((seeds, maps): Parsed) -> usize {
        seeds
            .into_iter()
            .map(|s| maps.iter().fold(s, map))
            .min()
            .unwrap()
    }
}

pub mod part2 {
    use std::cmp::Ordering;

    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    use super::{Map, Parsed, Range};

    fn unmap(dst: usize, map: &Map) -> usize {
        let target = map.binary_search_by(|r| {
            if dst < r.destination {
                Ordering::Greater
            } else if dst > r.destination_end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        target
            .map(|r| {
                let range = map[r];
                range.source + dst - range.destination
            })
            .unwrap_or(dst)
    }

    pub fn solve((seed_ranges, mut maps): Parsed) -> usize {
        maps.iter_mut()
            .for_each(|m| m.sort_by_key(|r| r.destination));
        let seeds: Vec<_> = (0..seed_ranges.len())
            .step_by(2)
            .map(|s| Range::new(usize::MAX, seed_ranges[s], seed_ranges[s + 1]))
            .collect();
        let length = 1_000_000;
        for start in 0.. {
            if let Some(location) = (start * length..(start + 1) * length)
                .into_par_iter()
                .find_first(|&location| {
                    let seed = maps.iter().rfold(location, unmap);
                    seeds
                        .iter()
                        .any(|r| r.source <= seed && seed <= r.source_end)
                })
            {
                return location;
            }
        }
        unreachable!();
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_05_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
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
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
