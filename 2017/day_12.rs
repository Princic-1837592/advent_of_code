//! https://adventofcode.com/2017/day/12
//! https://adventofcode.com/2017/day/12/input

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

type Graph = Vec<HashSet<usize>>;

fn parse(input: &str) -> Graph {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" <-> ");
            let pipes = parts.nth(1).unwrap();
            pipes.split(", ").map(|p| p.parse().unwrap()).collect()
        })
        .collect()
}

fn bfs(graph: &Graph, visited: &mut Vec<bool>) -> usize {
    let mut count = 0;
    let mut queue = VecDeque::from([(0..visited.len()).find(|&i| !visited[i]).unwrap()]);
    while let Some(program) = queue.pop_front() {
        visited[program] = true;
        count += 1;
        for &pipe in &graph[program] {
            if !visited[pipe] {
                queue.push_back(pipe);
            }
        }
    }
    count
}

pub mod part1 {
    use super::{bfs, parse};

    pub fn solve(input: &str) -> usize {
        let graph = parse(input);
        let mut visited = vec![false; graph.len()];
        bfs(&graph, &mut visited)
    }
}

pub mod part2 {
    use super::{bfs, parse};

    pub fn solve(input: &str) -> usize {
        let graph = parse(input);
        let mut count = 0;
        let mut groups = 0;
        let mut visited = vec![false; graph.len()];
        while count < graph.len() {
            groups += 1;
            count += bfs(&graph, &mut visited)
        }
        groups
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
