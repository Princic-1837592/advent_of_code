//! https://adventofcode.com/2018/day/13
//! https://adventofcode.com/2018/day/13/input

use std::{cmp::Ordering, collections::BinaryHeap, fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Cart {
    position: (isize, isize),
    direction: (isize, isize),
    next_turn: Turn,
}

impl Eq for Cart {}

impl PartialEq<Self> for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl PartialOrd<Self> for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.position.cmp(&other.position).reverse())
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position).reverse()
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, BinaryHeap<Cart>) {
    let mut heap = BinaryHeap::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| match char {
                    '>' => {
                        heap.push(Cart {
                            position: (i as isize, j as isize),
                            direction: (0, 1),
                            next_turn: Turn::Left,
                        });
                        '-'
                    }
                    '<' => {
                        heap.push(Cart {
                            position: (i as isize, j as isize),
                            direction: (0, -1),
                            next_turn: Turn::Left,
                        });
                        '-'
                    }
                    '^' => {
                        heap.push(Cart {
                            position: (i as isize, j as isize),
                            direction: (-1, 0),
                            next_turn: Turn::Left,
                        });
                        '|'
                    }
                    'v' => {
                        heap.push(Cart {
                            position: (i as isize, j as isize),
                            direction: (1, 0),
                            next_turn: Turn::Left,
                        });
                        '|'
                    }
                    _ => char,
                })
                .collect()
        })
        .collect();
    (map, heap)
}

fn tick(
    map: &[Vec<char>],
    carts: &mut BinaryHeap<Cart>,
    support: &mut BinaryHeap<Cart>,
) -> Vec<(isize, isize)> {
    support.clear();
    let mut crashes = Vec::new();
    while let Some(mut cart) = carts.pop() {
        if crashes.contains(&cart.position) {
            continue;
        }
        match map[cart.position.0 as usize][cart.position.1 as usize] {
            '+' => {
                match cart.next_turn {
                    Turn::Left => cart.direction = (-cart.direction.1, cart.direction.0),
                    Turn::Straight => {}
                    Turn::Right => cart.direction = (cart.direction.1, -cart.direction.0),
                };
                cart.next_turn = cart.next_turn.next();
            }
            '\\' => match cart.direction {
                (0, _) => cart.direction = (cart.direction.1, -cart.direction.0),
                (_, 0) => cart.direction = (-cart.direction.1, cart.direction.0),
                _ => {}
            },
            '/' => match cart.direction {
                (0, _) => cart.direction = (-cart.direction.1, cart.direction.0),
                (_, 0) => cart.direction = (cart.direction.1, -cart.direction.0),
                _ => {}
            },
            _ => {}
        }
        cart.position = (
            cart.position.0 + cart.direction.0,
            cart.position.1 + cart.direction.1,
        );
        if support
            .iter()
            .chain(carts.iter())
            .any(|other| cart.position == other.position)
        {
            crashes.push(cart.position);
        } else {
            support.push(cart);
        }
    }
    carts.extend(support.iter());
    crashes
}

pub mod part1 {
    use std::collections::BinaryHeap;

    use crate::day_13::{parse, tick};

    pub fn solve(input: &str) -> String {
        let (map, mut carts) = parse(input);
        let mut support = BinaryHeap::with_capacity(carts.len());
        loop {
            if let Some((y, x)) = tick(&map, &mut carts, &mut support).pop() {
                return format!("{},{}", x, y);
            }
        }
    }
}

pub mod part2 {
    use std::collections::BinaryHeap;

    use crate::day_13::{parse, tick};

    pub fn solve(input: &str) -> String {
        let (map, mut carts) = parse(input);
        let mut support = BinaryHeap::with_capacity(carts.len());
        while carts.len() > 1 {
            let crashes = tick(&map, &mut carts, &mut support);
            if !crashes.is_empty() {
                carts = carts
                    .iter()
                    .filter(|cart| !crashes.contains(&cart.position))
                    .cloned()
                    .collect();
            }
        }
        let last = carts.pop().unwrap();
        format!("{},{}", last.position.1, last.position.0)
    }
}

pub fn main(test: bool) {
    let test_input = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
