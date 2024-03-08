//! https://adventofcode.com/2017/day/9
//! https://adventofcode.com/2017/day/9/input

use std::{fs::read_to_string, time::Instant};

struct Group {
    internal: Vec<Group>,
    garbage: usize,
}

impl From<&str> for Group {
    fn from(string: &str) -> Self {
        fn recursive(string: &Vec<char>, mut i: usize) -> (Group, usize) {
            let mut internal = Vec::new();
            let mut is_garbage = false;
            let mut garbage = 0;
            while i < string.len() {
                match string[i] {
                    '{' if !is_garbage => {
                        let (group, ni) = recursive(string, i + 1);
                        i = ni;
                        internal.push(group);
                    }
                    '}' if !is_garbage => return (Group { internal, garbage }, i),
                    '<' if !is_garbage => is_garbage = true,
                    '!' if is_garbage => i += 1,
                    '>' if is_garbage => is_garbage = false,
                    _ if is_garbage => garbage += 1,
                    _ => {}
                }
                i += 1;
            }
            unreachable!(r#"Probably a "}}" is missing"#)
        }
        recursive(&string.chars().skip(1).collect(), 0).0
    }
}

fn parse(input: &str) -> Group {
    Group::from(input)
}

pub mod part1 {
    use super::{parse, Group};

    fn score(group: &Group, partial: usize) -> usize {
        let mut total = partial;
        for internal in &group.internal {
            total += score(internal, partial + 1);
        }
        total
    }

    pub fn solve(input: &str) -> usize {
        let group = parse(input);
        score(&group, 1)
    }
}

pub mod part2 {
    use super::{parse, Group};

    fn count_garbage(group: &Group) -> usize {
        group.garbage + group.internal.iter().map(count_garbage).sum::<usize>()
    }

    pub fn solve(input: &str) -> usize {
        let group = parse(input);
        count_garbage(&group)
    }
}

pub fn main(test: bool) {
    let test_input = r#"{<!!>}"#.to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
