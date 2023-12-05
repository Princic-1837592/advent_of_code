//! https://adventofcode.com/2023/day/5
//! https://adventofcode.com/2023/day/5/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    source: usize,
    source_end: usize,
    destination: usize,
    destination_end: usize,
    length: usize,
}

impl Range {
    fn new(source: usize, destination: usize, length: usize) -> Self {
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

type Parsed = (Vec<usize>, Vec<Vec<Range>>);

fn parse(input: &str) -> Parsed {
    let separator = "\n".repeat(2);
    let mut parts = input.split(&separator);
    let seeds = parts.next().unwrap()[6..]
        .split_whitespace()
        .map(|s| s.parse().expect(&format!("{}", s)))
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
    use super::Parsed;

    pub fn solve((seeds, maps): Parsed) -> usize {
        println!("{:?}", seeds);
        println!("{:#?}", maps);
        0
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {
        0
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
