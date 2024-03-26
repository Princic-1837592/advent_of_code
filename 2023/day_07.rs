//! https://adventofcode.com/2023/day/7
//! https://adventofcode.com/2023/day/7/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use utils::{parsing::parse_lines, FromStr};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '?' => Card::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Cards = [Card; 5];

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    strength: Type,
    cards: Cards,
}

impl Hand {
    fn new(cards: Cards) -> Self {
        Self {
            cards,
            strength: Self::compute_score(cards),
        }
    }

    fn compute_score(cards: Cards) -> Type {
        let mut occurrencies = [0; 14];
        let mut jokers = 0;
        for card in cards {
            if let Card::Joker = card {
                jokers += 1;
            } else {
                occurrencies[card as usize] += 1;
            }
        }
        occurrencies.sort();
        occurrencies.reverse();
        match (jokers, occurrencies) {
            (0, [5, ..])
            | (1, [4, ..])
            | (2, [3, ..])
            | (3, [2, ..])
            | (4, [1, ..])
            | (5, [0, ..]) => Type::FiveOfAKind,
            (0, [4, ..]) | (1, [3, ..]) | (2, [2, ..]) | (3, [1, ..]) => Type::FourOfAKind,
            (0, [3, 2, ..]) | (1, [2, 2, ..]) => Type::FullHouse,
            (0, [3, 1, ..]) | (1, [2, 1, ..]) | (2, [1, ..]) => Type::ThreeOfAKind,
            (0, [2, 2, ..]) => Type::TwoPairs,
            (0, [2, 1, ..]) | (1, [1, ..]) => Type::OnePair,
            (0, [1, ..]) => Type::HighCard,
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut cards = [Card::Two; 5];
        for (i, c) in value.chars().enumerate() {
            cards[i] = Card::from(c);
        }
        Self::new(cards)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, FromStr)]
pub struct Play {
    #[into]
    hand: Hand,
    bid: usize,
}

type Parsed = Vec<Play>;

pub fn parse(input: &str) -> Parsed {
    parse_lines(input)
}

fn solve(mut plays: Parsed) -> usize {
    plays.sort();
    plays
        .iter()
        .enumerate()
        .map(|(i, play)| (i + 1) * play.bid)
        .sum()
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_07_input.txt").unwrap()
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
    let result = solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let parsed = parse(&puzzle_input.replace('J', "?"));
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
