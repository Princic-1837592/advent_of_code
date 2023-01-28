//! https://adventofcode.com/2018/day/9
//! https://adventofcode.com/2018/day/9/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> (usize, usize) {
    let mut parts = input.split_whitespace();
    (
        parts.next().unwrap().parse().unwrap(),
        parts.nth(5).unwrap().parse().unwrap(),
    )
}

#[derive(Copy, Clone, Default)]
struct Marble {
    prev: usize,
    next: usize,
}

fn play(players: usize, marbles: usize) -> usize {
    let mut points = vec![0; players];
    let mut nodes = vec![Marble { prev: 0, next: 0 }; marbles + 1];
    let mut current = 0;
    let mut player = 0;
    for marble in 1..=marbles {
        if marble % 23 != 0 {
            let left = nodes[current].next;
            let right = nodes[left].next;
            nodes[left].next = marble;
            nodes[right].prev = marble;
            nodes[marble].prev = left;
            nodes[marble].next = right;
            current = marble;
        } else {
            points[player] += marble;
            let mut to_remove = current;
            for _ in 0..7 {
                to_remove = nodes[to_remove].prev;
            }
            let left = nodes[to_remove].prev;
            let right = nodes[to_remove].next;
            nodes[left].next = right;
            nodes[right].prev = left;
            points[player] += to_remove;
            current = right;
        }
        player = (player + 1) % players;
    }
    *points.iter().max().unwrap()
}

pub mod part1 {
    use crate::day_09::{parse, play};

    pub fn solve(input: &str) -> usize {
        let (players, marbles) = parse(input);
        play(players, marbles)
    }
}

pub mod part2 {
    use crate::day_09::{parse, play};

    pub fn solve(input: &str) -> usize {
        let (players, marbles) = parse(input);
        play(players, marbles * 100)
    }
}

pub fn main(test: bool) {
    let test_input = "10 players; last marble is worth 1618 points".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
