//! https://adventofcode.com/2019/day/22
//! https://adventofcode.com/2019/day/22/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Technique {
    Dins,
    Cut(isize),
    Dwi(usize),
}

impl From<&str> for Technique {
    fn from(string: &str) -> Self {
        if string == "deal into new stack" {
            Technique::Dins
        } else if string.starts_with("cut ") {
            Technique::Cut(string.split_whitespace().last().unwrap().parse().unwrap())
        } else {
            Technique::Dwi(string.split_whitespace().last().unwrap().parse().unwrap())
        }
    }
}

fn parse(input: &str) -> Vec<Technique> {
    input.lines().map(Technique::from).collect()
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{parse, Technique};

    pub fn solve(input: &str, cards: usize, card: usize) -> usize {
        let techniques = parse(input);
        let mut deck: VecDeque<_> = (0..cards).collect();
        let mut support = vec![0; deck.len()];
        for technique in techniques {
            match technique {
                Technique::Dins => deck = deck.into_iter().rev().collect(),
                Technique::Cut(mut n) => {
                    if n < 0 {
                        n = n.abs();
                        for _ in 0..n {
                            let popped = deck.pop_back().unwrap();
                            deck.push_front(popped);
                        }
                    } else {
                        for _ in 0..n {
                            let popped = deck.pop_front().unwrap();
                            deck.push_back(popped);
                        }
                    }
                }
                Technique::Dwi(n) => {
                    let mut i = 0;
                    for _ in 0..support.len() {
                        support[i] = deck.pop_front().unwrap();
                        i = (i + n) % support.len();
                    }
                    deck = support.iter().cloned().collect();
                }
            }
        }
        deck.iter().position(|&c| c == card).unwrap()
    }
}

pub mod part2 {
    use super::{parse, Technique};

    fn modular_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
        if modulus == 1 {
            return 0;
        }
        let mut res = 1;
        base %= modulus;
        while exp > 0 {
            if (exp % 2) == 1 {
                res = res * base % modulus;
            }
            exp >>= 1;
            base = base * base % modulus;
        }
        res as i128
    }

    fn to_linear_polynomial(l: i128, techniques: &[Technique]) -> (i128, i128) {
        let (mut a, mut b) = (1, 0);
        for &technique in techniques.iter().rev() {
            match technique {
                Technique::Dins => {
                    a = -a;
                    b = l - b - 1;
                }
                Technique::Cut(n) => b = (b + n as i128) % l,
                Technique::Dwi(n) => {
                    let z = modular_pow(n as i128, l - 2, l);
                    a = (a * z).rem_euclid(l);
                    b = (b * z).rem_euclid(l);
                }
            }
        }
        (a, b)
    }

    fn polypow(a: i128, b: i128, m: i128, n: i128) -> (i128, i128) {
        if m == 0 {
            return (1, 0);
        }
        if m % 2 == 0 {
            polypow(a * a % n, (a * b + b) % n, m / 2, n)
        } else {
            let (c, d) = polypow(a, b, m - 1, n);
            (a * c % n, (a * d + b) % n)
        }
    }

    pub fn solve(input: &str, cards: i128, card: i128, deck_size: i128) -> i128 {
        let techniques = parse(input);
        let (a, b) = to_linear_polynomial(cards, &techniques);
        let (a, b) = polypow(a, b, deck_size, cards);
        (card * a + b) % cards
    }
}

pub fn main(test: bool) {
    let test_input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"
        .to_owned();
    let (puzzle_input, cards, card) = if test {
        (test_input, 10, 3)
    } else {
        (
            read_to_string("../inputs/2019/day_22_input.txt").unwrap(),
            10007,
            2019,
        )
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, cards, card));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!(
        "{}",
        part2::solve(&puzzle_input, 119315717514047, 2020, 101741582076661)
    );
    println!("Run in {:?}", start.elapsed());
}
