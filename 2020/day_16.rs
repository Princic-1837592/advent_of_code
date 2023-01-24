use std::{
    hash::{Hash, Hasher},
    ops::RangeInclusive,
    time::Instant,
};
use std::fs::read_to_string;

use crate::LINE_ENDING;

#[derive(Debug, Clone, Eq)]
struct Rule {
    name: String,
    left: RangeInclusive<usize>,
    right: RangeInclusive<usize>,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn parse(input: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    fn parse_rule(rule: &str) -> Rule {
        let mut parts = rule.split(": ");
        let name = parts.next().unwrap().to_owned();
        let mut parts = parts.next().unwrap().split(' ');
        let mut left = parts.next().unwrap().split('-');
        let left = left.next().unwrap().parse().unwrap()..=left.next().unwrap().parse().unwrap();
        let mut right = parts.nth(1).unwrap().split('-');
        let right = right.next().unwrap().parse().unwrap()..=right.next().unwrap().parse().unwrap();
        Rule { name, left, right }
    }
    let separator = LINE_ENDING.repeat(2);
    let mut sections = input.split(&separator);
    let rules = sections.next().unwrap().lines().map(parse_rule).collect();
    let ticket: Vec<_> = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let tickets = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, ticket, tickets)
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (rules, _ticket, tickets) = parse(input);
        tickets
            .iter()
            .flatten()
            .filter(|&&n| {
                !rules
                    .iter()
                    .any(|rule| rule.left.contains(&n) || rule.right.contains(&n))
            })
            .sum()
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (rules, ticket, mut tickets) = parse(input);
        tickets.retain(|ticket| {
            ticket.iter().all(|&n| {
                rules
                    .iter()
                    .any(|rule| rule.left.contains(&n) || rule.right.contains(&n))
            })
        });
        let mut valid: Vec<HashSet<_>> = vec![rules.iter().collect(); ticket.len()];
        let mut bound = HashMap::new();
        valid.iter_mut().enumerate().for_each(|(i, set)| {
            set.retain(|rule| {
                tickets
                    .iter()
                    .all(|ticket| rule.left.contains(&ticket[i]) || rule.right.contains(&ticket[i]))
            });
            set.iter().for_each(|rule| {
                bound.entry(rule).or_insert_with(HashSet::new).insert(i);
            });
        });
        while bound.values().map(|set| set.len()).sum::<usize>() != rules.len() {
            let fixed: HashSet<_> = bound
                .iter()
                .filter_map(|(_, set)| (set.len() == 1).then_some(*set.iter().next().unwrap()))
                .collect();
            bound.iter_mut().for_each(|(_, set)| {
                if set.len() > 1 {
                    set.retain(|&i| !fixed.contains(&i));
                }
            });
        }
        bound
            .iter()
            .filter_map(|(rule, set)| {
                rule.name
                    .starts_with("departure")
                    .then_some(*set.iter().next().unwrap())
            })
            .map(|i| ticket[i])
            .product()
    }
}

pub fn main(test: bool) {
    let test_input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
        .to_owned()
        .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_16_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
