//! https://adventofcode.com/2016/day/22
//! https://adventofcode.com/2016/day/22/input

use std::{fs::read_to_string, time::Instant};

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
        let mut parts = string.split_whitespace();
        let mut location = parts.next().unwrap().split('-');
        let x = location.nth(1).unwrap()[1..].parse().unwrap();
        let y = location.next().unwrap()[1..].parse().unwrap();
        let mut part = parts.next().unwrap();
        let size = part[..part.len() - 1].parse().unwrap();
        part = parts.next().unwrap();
        let used = part[..part.len() - 1].parse().unwrap();
        part = parts.next().unwrap();
        let avail = part[..part.len() - 1].parse().unwrap();
        part = parts.next().unwrap();
        let used_percentage = part[..part.len() - 1].parse().unwrap();
        Node {
            x,
            y,
            size,
            used,
            avail,
            used_percentage,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Node>> {
    let nodes: Vec<_> = input.lines().skip(2).map(Node::from).collect();
    let width = nodes.iter().map(|node| node.y).max().unwrap() + 1;
    let height = nodes.iter().map(|node| node.x).max().unwrap() + 1;
    let mut result = vec![vec![Node::default(); width]; height];
    for node in nodes {
        result[node.x][node.y] = node;
    }
    result
}

pub mod part1 {
    use std::cmp::Ordering;

    use crate::day_22::parse;

    pub fn solve(input: &str) -> usize {
        let nodes = parse(input);
        let mut ordered: Vec<_> = nodes.iter().flatten().collect();
        ordered.sort_by_key(|node| -(node.used as isize));
        let last = ordered
            .binary_search_by(|node| node.used.cmp(&0).reverse())
            .unwrap_or(ordered.len());
        ordered.truncate(last);
        ordered.reverse();
        let mut pairs = 0;
        for node in nodes.iter().flatten() {
            let can_contain = ordered.binary_search_by(|other| match node.avail.cmp(&other.used) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal | Ordering::Greater => Ordering::Less,
            });
            pairs += can_contain.unwrap_err();
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
