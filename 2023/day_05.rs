//! https://adventofcode.com/2023/day/5
//! https://adventofcode.com/2023/day/5/input

use std::{
    cmp::Ordering,
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
    length: usize,
}

impl Range {
    fn new(destination: usize, source: usize, length: usize) -> Self {
        Range {
            source,
            source_end: source + length - 1,
            destination,
            destination_end: destination + length - 1,
            length,
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

fn map(src: usize, map: &Map) -> usize {
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

pub mod part1 {
    use super::{map, Parsed};

    pub fn solve((seeds, maps): Parsed) -> usize {
        seeds
            .into_iter()
            .map(|s| maps.iter().fold(s, map))
            .min()
            .unwrap()
    }
}

pub mod part2 {
    use super::{map, Parsed};

    pub fn solve((seeds, maps): Parsed) -> usize {
        let mut min = usize::MAX;
        for s in (0..seeds.len()).step_by(2) {
            for seed in seeds[s]..seeds[s] + seeds[s + 1] {
                min = min.min(maps.iter().fold(seed, map))
            }
        }
        min
    }
}

pub fn main(test: bool) -> Duration {
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
