//! https://adventofcode.com/2016/day/22
//! https://adventofcode.com/2016/day/22/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

#[derive(Copy, Clone, Debug, Default)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    #[allow(unused)]
    used_percentage: usize,
}

impl From<&str> for Node {
    fn from(string: &str) -> Self {
        let pattern =
            Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%")
                .unwrap();
        let captures = pattern.captures(string).unwrap();
        Node {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
            size: captures.get(3).unwrap().as_str().parse().unwrap(),
            used: captures.get(4).unwrap().as_str().parse().unwrap(),
            avail: captures.get(5).unwrap().as_str().parse().unwrap(),
            used_percentage: captures.get(6).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Node>> {
    let nodes: Vec<_> = input.lines().skip(2).map(Node::from).collect();
    let mut result = vec![];
    for node in nodes {
        if node.x >= result.len() {
            result.resize(node.x + 1, vec![]);
        }
        if node.y >= result[node.x].len() {
            result[node.x].resize(node.y + 1, Node::default());
        }
        result[node.x][node.y] = node;
    }
    result
}

pub mod part1 {
    use crate::day_22::parse;

    pub fn solve(input: &str) -> usize {
        let nodes = parse(input);
        let mut pairs = 0;
        for (i, a) in nodes.iter().flatten().enumerate() {
            for b in nodes.iter().flatten().skip(i + 1) {
                if a.used != 0 && b.avail >= a.used {
                    pairs += 1
                }
                if b.used != 0 && a.avail >= b.used {
                    pairs += 1
                }
            }
        }
        pairs
    }
}

pub mod part2 {
    use std::collections::{HashSet, VecDeque};

    use crate::day_22::parse;

    const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn solve(input: &str) -> usize {
        let nodes = parse(input);
        let mut initial = (0, 0);
        for (i, row) in nodes.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if node.used == 0 {
                    initial = (i, j);
                    break;
                }
            }
        }
        let target = (nodes.len() - 2, 0);
        let mut queue = VecDeque::from([(initial, 0)]);
        let mut first_distance = usize::MAX;
        let mut visited = HashSet::new();
        while let Some((coord @ (i, j), distance)) = queue.pop_front() {
            if coord == target {
                first_distance = distance;
                break;
            }
            if visited.contains(&coord) {
                continue;
            }
            visited.insert(coord);
            let current = nodes[i][j];
            for (ni, nj) in NEIGHBOURS.map(|(di, dj)| ((i as isize + di), (j as isize + dj))) {
                if ni < 0 || ni >= nodes.len() as isize || nj < 0 || nj >= nodes[0].len() as isize {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                let next = nodes[ni][nj];
                if current.size >= next.used {
                    queue.push_back(((ni, nj), distance + 1));
                }
            }
        }
        first_distance + 1 + (nodes.len() - 2) * 5
    }
}

pub fn main(test: bool) {
    let test_input = "Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%"
        .to_owned();
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
