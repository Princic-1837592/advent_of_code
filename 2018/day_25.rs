//! https://adventofcode.com/2018/day/25
//! https://adventofcode.com/2018/day/25/input

use std::{fs::read_to_string, time::Instant};

type Point = (isize, isize, isize, isize);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let point: Vec<_> = line.split(',').map(|c| c.parse().unwrap()).collect();
            (point[0], point[1], point[2], point[3])
        })
        .collect()
}

pub mod part1 {
    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::day_25::{parse, Point};

    fn manhattan(p1: &Point, p2: &Point) -> isize {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
    }

    pub fn solve(input: &str) -> usize {
        let points = parse(input);
        let mut graph = HashMap::new();
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate().skip(i + 1) {
                if manhattan(p1, p2) <= 3 {
                    graph.entry(i).or_insert_with(HashSet::new).insert(j);
                    graph.entry(j).or_insert_with(HashSet::new).insert(i);
                }
            }
        }
        let mut clusters = 0;
        let mut visited = vec![false; points.len()];
        for point in 0..points.len() {
            if !visited[point] {
                clusters += 1;
                let mut queue = VecDeque::from([point]);
                while let Some(point) = queue.pop_front() {
                    if !visited[point] {
                        visited[point] = true;
                        if let Some(neighbors) = graph.get(&point) {
                            queue.extend(neighbors);
                        }
                    }
                }
            }
        }
        clusters
    }
}

pub fn main(test: bool) {
    let test_input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
