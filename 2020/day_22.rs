use std::{collections::VecDeque, time::Instant};
use std::fs::read_to_string;

use crate::LINE_ENDING;

fn parse(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let separator = LINE_ENDING.repeat(2);
    let mut players = input.split(&separator);
    let player1 = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    let player2 = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    (player1, player2)
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::parse;

    fn turn(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    pub fn solve(input: &str) -> usize {
        let (mut player1, mut player2) = parse(input);
        while !player1.is_empty() && !player2.is_empty() {
            turn(&mut player1, &mut player2);
        }
        let winner = if player1.is_empty() { player2 } else { player1 };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use super::parse;

    fn game(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> usize {
        let mut previous_rounds = Vec::new();
        while !player1.is_empty() && !player2.is_empty() {
            if previous_rounds.contains(&(player1.clone(), player2.clone())) {
                return 1;
            }
            previous_rounds.push((player1.clone(), player2.clone()));
            turn(player1, player2);
        }
        if player1.is_empty() {
            2
        } else {
            1
        }
    }

    fn turn(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 <= player1.len() && card2 <= player2.len() {
            let mut recursive_player1 = player1.iter().take(card1).copied().collect();
            let mut recursive_player2 = player2.iter().take(card2).copied().collect();
            let winner = game(&mut recursive_player1, &mut recursive_player2);
            if winner == 1 {
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                player2.push_back(card2);
                player2.push_back(card1);
            }
        } else if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    pub fn solve(input: &str) -> usize {
        let (mut player1, mut player2) = parse(input);
        let winner = game(&mut player1, &mut player2);
        let winner = if winner == 1 { player1 } else { player2 };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum()
    }
}

pub fn main(test: bool) {
    let test_input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
    .to_owned()
    .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_22_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
