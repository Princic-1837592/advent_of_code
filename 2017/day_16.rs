//! https://adventofcode.com/2017/day/16
//! https://adventofcode.com/2017/day/16/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

impl From<&str> for Move {
    fn from(string: &str) -> Self {
        match string.chars().next().unwrap() {
            's' => Move::Spin(string[1..].parse().unwrap()),
            'x' => {
                let mut parts = string[1..]
                    .split('/')
                    .map(|program| program.parse().unwrap());
                Move::Exchange(parts.next().unwrap(), parts.next().unwrap())
            }
            'p' => Move::Partner(
                string.chars().nth(1).unwrap() as usize - 'a' as usize,
                string.chars().nth(3).unwrap() as usize - 'a' as usize,
            ),
            _ => panic!("Invalid dance move: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Move> {
    input.split(',').map(Move::from).collect()
}

fn dance(programs: &mut [usize], positions: &mut [usize], moves: &[Move]) {
    for &dance_move in moves {
        match dance_move {
            Move::Spin(right) => {
                let left = programs.len() - right;
                for position in left..programs.len() {
                    positions[programs[position]] -= left;
                }
                for position in 0..left {
                    positions[programs[position]] += right;
                }
                for (program, &position) in positions.iter().enumerate() {
                    programs[position] = program;
                }
            }
            Move::Exchange(position_a, position_b) => {
                (programs[position_a], programs[position_b]) =
                    (programs[position_b], programs[position_a]);
                (
                    positions[programs[position_a]],
                    positions[programs[position_b]],
                ) = (position_a, position_b);
            }
            Move::Partner(program_a, program_b) => {
                (positions[program_a], positions[program_b]) =
                    (positions[program_b], positions[program_a]);
                (
                    programs[positions[program_a]],
                    programs[positions[program_b]],
                ) = (program_a, program_b);
            }
        }
    }
}

pub mod part1 {
    use super::{dance, parse};

    pub fn solve(input: &str, last_letter: char) -> String {
        let moves = parse(input);
        let mut programs = vec![0; last_letter as usize - 'a' as usize + 1];
        let mut positions = vec![0; last_letter as usize - 'a' as usize + 1];
        for i in 0..positions.len() {
            programs[i] = i;
            positions[i] = i;
        }
        dance(&mut programs, &mut positions, &moves);
        programs
            .iter()
            .map(|program| (program + 'a' as usize) as u8 as char)
            .collect()
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap};

    use super::{dance, parse};

    pub fn solve(input: &str, last_letter: char) -> String {
        let moves = parse(input);
        let mut programs = vec![0; last_letter as usize - 'a' as usize + 1];
        let mut positions = vec![0; last_letter as usize - 'a' as usize + 1];
        for i in 0..positions.len() {
            programs[i] = i;
            positions[i] = i;
        }
        let mut seen = HashMap::new();
        let target = 1_000_000_000;
        let mut i = 0;
        while i < target {
            match seen.entry(programs.clone()) {
                Entry::Occupied(entry) => {
                    let last = entry.get();
                    let interval = i - last;
                    let left = target - i;
                    let cycles = left / interval;
                    i += cycles * interval;
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
            dance(&mut programs, &mut positions, &moves);
            i += 1;
        }
        programs
            .iter()
            .map(|program| (program + 'a' as usize) as u8 as char)
            .collect()
    }
}

pub fn main(test: bool) {
    let test_input = "s1,x3/4,pe/b".to_owned();
    let (puzzle_input, last_letter) = if test {
        (test_input, 'e')
    } else {
        (read_to_string("../inputs/2017/day_16_input.txt").unwrap(), 'p')
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, last_letter));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, last_letter));
    println!("Run in {:?}", start.elapsed());
}
