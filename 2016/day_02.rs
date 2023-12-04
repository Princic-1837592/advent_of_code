//! https://adventofcode.com/2016/day/2
//! https://adventofcode.com/2016/day/2/input

use std::{fs::read_to_string, time::Instant};

enum Movement {
    U,
    D,
    L,
    R,
}

impl From<char> for Movement {
    fn from(char: char) -> Self {
        match char {
            'U' => Self::U,
            'D' => Self::D,
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("Invalid char: {}", char),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Movement>> {
    input
        .lines()
        .map(|line| line.chars().map(Movement::from).collect())
        .collect()
}

pub mod part1 {
    use super::{parse, Movement};

    pub fn solve(input: &str) -> String {
        let movements = parse(input);
        let keyboard = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
        let (mut x, mut y) = (0, 0);
        let mut result = String::with_capacity(movements.len());
        for line in movements {
            for movement in line {
                match movement {
                    Movement::U => {
                        if x > 0 {
                            x -= 1
                        }
                    }
                    Movement::D => {
                        if x < 2 {
                            x += 1
                        }
                    }
                    Movement::L => {
                        if y > 0 {
                            y -= 1
                        }
                    }
                    Movement::R => {
                        if y < 2 {
                            y += 1
                        }
                    }
                }
            }
            result.push(keyboard[x][y]);
        }
        result
    }
}

pub mod part2 {
    use super::{parse, Movement};

    pub fn solve(input: &str) -> String {
        let movements = parse(input);
        let keyboard = [
            [' ', ' ', '1', ' ', ' '],
            [' ', '2', '3', '4', ' '],
            ['5', '6', '7', '8', '9'],
            [' ', 'A', 'B', 'C', ' '],
            [' ', ' ', 'D', ' ', ' '],
        ];
        let (mut x, mut y) = (2, 0);
        let mut result = String::with_capacity(movements.len());
        for line in movements {
            for movement in line {
                match movement {
                    Movement::U => {
                        if x > 0 && keyboard[x - 1][y] != ' ' {
                            x -= 1
                        }
                    }
                    Movement::D => {
                        if x < 4 && keyboard[x + 1][y] != ' ' {
                            x += 1
                        }
                    }
                    Movement::L => {
                        if y > 0 && keyboard[x][y - 1] != ' ' {
                            y -= 1
                        }
                    }
                    Movement::R => {
                        if y < 4 && keyboard[x][y + 1] != ' ' {
                            y += 1
                        }
                    }
                }
            }
            result.push(keyboard[x][y]);
        }
        result
    }
}

pub fn main(test: bool) {
    let test_input = "ULL
RRDDD
LURDL
UUUUD"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
