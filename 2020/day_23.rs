use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone)]
struct Node {
    val: usize,
    prev: usize,
    next: usize,
}

fn parse(input: impl Iterator<Item = usize>) -> Vec<Node> {
    let mut result: Vec<_> = input
        .map(|n| Node {
            val: n,
            prev: 0,
            next: 0,
        })
        .collect();
    for i in 1..result.len() {
        let prev_i = if i == 1 { result.len() - 1 } else { i - 1 };
        result[i].prev = result[prev_i].val;
        let next_i = if i == result.len() - 1 { 1 } else { i + 1 };
        result[i].next = result[next_i].val;
    }
    result[0].next = result[1].val;
    let mut i = 1;
    while i < 10 {
        if result[i].val != i {
            let tmp = result[i];
            result[i] = result[result[i].val];
            result[tmp.val] = tmp;
        } else {
            i += 1;
        }
    }
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

fn do_moves(how_many: usize, cups: &mut [Node]) {
    let mut removed_stack = Vec::with_capacity(3);
    let mut present = vec![true; cups.len()];
    let mut current = cups[0].next;
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
                destination = cups.len() - 1;
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
        current = cups[current].next;
    }
}

pub mod part1 {
    use super::{do_moves, parse, Node};

    fn to_string(cups: &Vec<Node>) -> String {
        let mut result = String::with_capacity(cups.len() - 2);
        let mut cup = cups[1].next;
        while cup != 1 {
            result.push_str(&cup.to_string());
            cup = cups[cup].next;
        }
        result
    }

    pub fn solve(input: &str) -> String {
        let mut cups = parse(
            "0".chars()
                .chain(input.chars())
                .map(|char| char.to_digit(10).unwrap() as usize),
        );
        do_moves(100, &mut cups);
        to_string(&cups)
    }
}

pub mod part2 {
    use super::{do_moves, parse};

    pub fn solve(input: &str) -> usize {
        let numbers = "0"
            .chars()
            .chain(input.chars())
            .map(|char| char.to_digit(10).unwrap() as usize);
        let mut cups = parse(
            numbers
                .clone()
                .chain((numbers.max().unwrap() + 1)..=1000000),
        );
        do_moves(10000000, &mut cups);
        let first = cups[1].next;
        let second = cups[first].next;
        first * second
    }
}

pub fn main(test: bool) {
    let test_input = "389125467".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_23_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
