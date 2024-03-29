//! https://adventofcode.com/2023/day/4
//! https://adventofcode.com/2023/day/4/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Card {
    left: [usize; 10],
    right: [usize; 25],
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut result = Card::default();
        let mut parts = value.split('|');
        let (left, right) = (parts.next().unwrap(), parts.next().unwrap());
        for (i, n) in left.split_whitespace().skip(2).enumerate() {
            result.left[i] = n.parse().unwrap();
        }
        for (i, n) in right.split_whitespace().enumerate() {
            result.right[i] = n.parse().unwrap();
        }
        result.left.sort();
        result.right.sort();
        result
    }
}

impl Card {
    fn eval(&self) -> usize {
        let matches = self.matches();
        if matches > 0 {
            2_usize.pow(matches as u32 - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        self.right
            .iter()
            .filter(|&&n| n > 0 && self.left.binary_search(&n).is_ok())
            .count()
    }
}

type Parsed = Vec<Card>;

fn parse(input: &str) -> Parsed {
    input.lines().map(Card::from).collect()
}

pub mod part1 {
    use super::{Card, Parsed};

    pub fn solve(cards: Parsed) -> usize {
        cards.iter().map(Card::eval).sum()
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(cards: Parsed) -> usize {
        let mut cards: Vec<_> = cards.into_iter().map(|c| (1, c)).collect();
        for c in 0..cards.len() {
            let (n, card) = cards[c];
            let matches = card.matches();
            for next in &mut cards[c + 1..c + 1 + matches] {
                next.0 += n
            }
        }
        cards.iter().map(|(n, _)| n).sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_04_input.txt").unwrap()
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

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
