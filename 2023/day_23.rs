//! https://adventofcode.com/2023/day/23
//! https://adventofcode.com/2023/day/23/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use utils::from_char;

#[from_char]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Path = '.',
    Forest = '#',
    Up = '^',
    Right = '>',
    Down = 'v',
    Left = '<',
}

type Parsed = Vec<Vec<Cell>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(Cell::from).collect())
        .collect()
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{Cell, Parsed};

    pub fn solve(map: Parsed) -> usize {
        let (h, w) = (map.len(), map[0].len());
        let mut queue = VecDeque::from([((0, 0), (0, 1), 1)]);
        let mut seen = vec![vec![0; w]; h];
        let mut result = 0;
        while let Some((from, coord @ (i, j), steps)) = queue.pop_front() {
            if steps <= seen[i][j] {
                continue;
            }
            seen[i][j] = steps;
            if coord == (h - 1, w - 2) {
                result = result.max(steps);
            }
            match map[i][j] {
                Cell::Path => {
                    let to @ (ni, nj) = (i.wrapping_sub(1), j);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Down) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i + 1, j);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Up) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i, j.wrapping_sub(1));
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Right) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i, j + 1);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Left) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                }
                Cell::Forest => unreachable!(),
                Cell::Up => queue.push_back((coord, (i - 1, j), steps + 1)),
                Cell::Right => queue.push_back((coord, (i, j + 1), steps + 1)),
                Cell::Down => queue.push_back((coord, (i + 1, j), steps + 1)),
                Cell::Left => queue.push_back((coord, (i, j - 1), steps + 1)),
            }
        }
        result - 1
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use utils::coords::iter_cross_near;

    use super::{Cell, Parsed};

    #[derive(Copy, Clone, Debug)]
    struct Edge {
        dst: usize,
        steps: usize,
    }

    fn compress_graph(map: Parsed) -> Vec<Vec<Edge>> {
        let mut crossings = vec![(0, 1)];
        for (i, row) in map.iter().enumerate().skip(1).take(map.len() - 2) {
            for j in (1..row.len() - 1).filter(|&j| row[j] != Cell::Forest) {
                if [map[i - 1][j], map[i][j + 1], map[i + 1][j], map[i][j - 1]]
                    .iter()
                    .filter(|&&cell| cell != Cell::Forest)
                    .count()
                    >= 3
                {
                    crossings.push((i, j));
                }
            }
        }
        crossings.push((map.len() - 1, map[0].len() - 2));
        let mut distances = vec![vec![]; crossings.len()];
        for (c, &crossing) in crossings.iter().enumerate() {
            let mut queue = VecDeque::from([(crossing, 0)]);
            let mut visited = vec![vec![false; map[0].len()]; map.len()];
            while let Some((crossing @ (i, j), steps)) = queue.pop_front() {
                visited[i][j] = true;
                if let Some(p) = crossings.iter().position(|&c| c == crossing) {
                    if steps > 0 {
                        distances[c].push(Edge { dst: p, steps });
                        continue;
                    }
                }
                for to @ (ni, nj) in iter_cross_near(i as isize, j as isize)
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                {
                    if ni < map.len()
                        && nj < map[0].len()
                        && map[ni][nj] != Cell::Forest
                        && !visited[ni][nj]
                    {
                        queue.push_back((to, steps + 1));
                    }
                }
            }
        }
        distances
    }

    fn explore(
        graph: &Vec<Vec<Edge>>,
        seen: &mut Vec<bool>,
        target: usize,
        node: usize,
        steps: usize,
    ) -> usize {
        seen[node] = true;
        if node == target {
            seen[node] = false;
            return steps;
        }
        let mut result = 0;
        for edge in &graph[node] {
            if !seen[edge.dst] {
                result = result.max(explore(graph, seen, target, edge.dst, steps + edge.steps));
            }
        }
        seen[node] = false;
        result
    }

    pub fn solve(map: Parsed) -> usize {
        let graph = compress_graph(map);
        let mut seen = vec![false; graph.len()];
        let target = graph.len() - 1;
        explore(&graph, &mut seen, target, 0, 0)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_23_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
