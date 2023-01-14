use std::{
    fmt::{Debug, Formatter},
    time::Instant,
};

#[derive(Clone, Copy)]
struct Node {
    val: usize,
    prev: usize,
    next: usize,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.prev, self.val, self.next)
    }
}

fn parse(input: &str) -> Vec<Node> {
    let mut result: Vec<_> = "0"
        .chars()
        .chain(input.chars())
        .map(|char| Node {
            prev: 0,
            val: char.to_digit(10).unwrap() as usize,
            next: 0,
        })
        .collect();
    for i in 1..result.len() {
        let prev_i = if i == 1 { result.len() - 1 } else { i - 1 };
        result[i].prev = result[prev_i].val;
        let next_i = if i == result.len() - 1 { 1 } else { i + 1 };
        result[i].next = result[next_i].val;
    }
    result.sort_by_key(|node| node.val);
    result[0].val = input.chars().next().unwrap().to_digit(10).unwrap() as usize;
    result
}

fn remove_after(current: usize, cups: &mut [Node]) -> usize {
    let to_remove = cups[current].next;
    let after_to_remove = cups[to_remove].next;
    cups[current].next = cups[to_remove].next;
    cups[after_to_remove].prev = current;
    to_remove
}

fn insert_after(to_insert: usize, current: usize, cups: &mut [Node]) {
    let after_current = cups[current].next;
    cups[to_insert].prev = current;
    cups[to_insert].next = cups[current].next;
    cups[current].next = to_insert;
    cups[after_current].prev = to_insert;
}

fn do_moves(
    how_many: usize,
    cups: &mut Vec<Node>,
    removed_stack: &mut Vec<usize>,
    present: &mut [bool],
) {
    let mut current = cups[0].val;
    for _ in 0..how_many {
        for _ in 0..3 {
            let removed = remove_after(current, cups);
            removed_stack.push(removed);
            present[removed] = false;
        }
        let mut destination = current;
        loop {
            destination -= 1;
            if destination == 0 {
                destination = 9;
            }
            if present[destination] {
                break;
            }
        }
        for _ in 0..3 {
            let to_insert = removed_stack.pop().unwrap();
            insert_after(to_insert, destination, cups);
            present[to_insert] = true;
        }
        current = cups[cups[current].next].val;
    }
}

fn to_string(cups: &Vec<Node>) -> String {
    let mut result = String::with_capacity(cups.len() - 2);
    let mut cup = cups[1].next;
    while cup != 1 {
        result.push(char::from_digit(cup as u32, 10).unwrap());
        cup = cups[cup].next;
    }
    result
}

pub mod part1 {
    use crate::day_23::{do_moves, parse, to_string};

    pub fn solve(input: &str) -> String {
        let mut cups = parse(input);
        let mut removed_stack = Vec::with_capacity(3);
        let mut present = vec![true; cups.len()];
        do_moves(100, &mut cups, &mut removed_stack, &mut present);
        to_string(&cups)
    }
}

pub mod part2 {
    pub fn solve(_input: &str) -> usize {
        0
    }
}

pub fn main(test: bool) {
    let test_input = "389125467".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_23_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
