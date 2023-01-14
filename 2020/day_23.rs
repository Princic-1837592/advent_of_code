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

fn parse(input: &str) -> (Vec<Node>, Vec<usize>) {
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
    let mut positions = vec![0; result.len()];
    result
        .iter()
        .enumerate()
        .for_each(|(i, node)| positions[node.val] = i);
    (result, positions)
}

fn remove_after(current: usize, cups: &mut [Node], positions: &[usize]) -> usize {
    let current_position = positions[current];
    let to_remove = cups[current_position].next;
    let to_remove_position = positions[to_remove];
    let after_to_remove_position = positions[cups[to_remove_position].next];
    cups[current_position].next = cups[to_remove_position].next;
    cups[after_to_remove_position].prev = current;
    to_remove
}

fn insert_after(to_insert: usize, current: usize, cups: &mut [Node], positions: &[usize]) {
    let to_insert_position = positions[to_insert];
    let current_position = positions[current];
    let after_current_position = positions[cups[current_position].next];
    cups[to_insert_position].prev = current;
    cups[to_insert_position].next = cups[current_position].next;
    cups[current_position].next = to_insert;
    cups[after_current_position].prev = to_insert;
}

fn do_moves(
    how_many: usize,
    mut cups: &mut Vec<Node>,
    positions: &Vec<usize>,
    removed_stack: &mut Vec<usize>,
    present: &mut Vec<bool>,
) {
    let mut current = cups[1].val;
    for _ in 0..how_many {
        // dbg!(&cups);
        // dbg!(current);
        for _ in 0..3 {
            let removed = remove_after(current, &mut cups, &positions);
            removed_stack.push(removed);
            present[removed] = false;
        }
        // dbg!(&cups);
        // dbg!(&removed_stack);
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
        // dbg!(destination);
        for _ in 0..3 {
            let to_insert = removed_stack.pop().unwrap();
            insert_after(to_insert, destination, &mut cups, &positions);
            present[to_insert] = true;
        }
        current = cups[positions[cups[positions[current]].next]].val;
    }
}

pub mod part1 {
    use crate::day_23::{do_moves, parse, Node};

    pub fn solve(input: &str) -> String {
        let (mut cups, positions) = parse(input);
        let mut removed_stack = Vec::with_capacity(3);
        let mut present = vec![true; cups.len()];
        do_moves(100, &mut cups, &positions, &mut removed_stack, &mut present);
        let mut result = String::with_capacity(cups.len() - 2);
        let mut cup = cups[positions[1]].next;
        while cup != 1 {
            result.push(char::from_digit(cup as u32, 10).unwrap());
            cup = cups[positions[cup]].next;
        }
        result
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
