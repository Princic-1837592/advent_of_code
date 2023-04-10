//! https://adventofcode.com/2019/day/18
//! https://adventofcode.com/2019/day/18/input

use std::{fs::read_to_string, time::Instant};

const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Copy, Clone, Debug)]
enum State {
    Wall,
    Space,
    Key(usize),
    Door(usize),
}

type Coord = (usize, usize);

fn parse(input: &str) -> (Vec<Vec<State>>, Coord, u32) {
    let mut entrance = (0, 0);
    let mut maze =
        vec![vec![State::Wall; input.lines().next().unwrap().len()]; input.lines().count()];
    let mut keys = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            maze[i][j] = match char {
                '.' => State::Space,
                '@' => {
                    entrance = (i, j);
                    State::Space
                }
                key @ 'a'..='z' => {
                    keys += 1;
                    State::Key(key as usize - 'a' as usize)
                }
                door @ 'A'..='Z' => State::Door(door as usize - 'A' as usize),
                _wall => State::Wall,
            };
        }
    }
    (maze, entrance, keys)
}

pub mod part1 {
    use std::collections::VecDeque;

    use crate::day_18::{parse, Coord, State, NEIGHBOURS};

    fn build_graph(
        maze: &Vec<Vec<State>>,
        entrance: Coord,
        total_keys: u32,
    ) -> Vec<Vec<(usize, usize)>> {
        fn keys_bfs(maze: &Vec<Vec<State>>, coord: Coord, total_keys: u32) -> Vec<(usize, usize)> {
            let mut queue = VecDeque::from([(coord, 0_usize, 0)]);
            let mut result = vec![(0, 0); total_keys as usize];
            let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
            let mut keys_found = 0;
            while let Some(((i, j), mut doors, steps)) = queue.pop_front() {
                if visited[i][j] || keys_found == total_keys {
                    continue;
                }
                visited[i][j] = true;
                match maze[i][j] {
                    State::Wall => continue,
                    State::Space => {}
                    State::Key(key) => {
                        result[key] = (steps, doors);
                        keys_found += 1;
                    }
                    State::Door(door) => doors |= 1 << door,
                }
                for next in NEIGHBOURS
                    .map(|(di, dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
                {
                    queue.push_back((next, doors, steps + 1));
                }
            }
            result
        }
        let mut graph = vec![vec![(0, 0); total_keys as usize]; total_keys as usize + 1];
        for (i, row) in maze.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let &State::Key(key) = cell {
                    graph[key] = keys_bfs(maze, (i, j), total_keys);
                }
            }
        }
        graph[total_keys as usize] = keys_bfs(maze, entrance, total_keys);
        graph
    }

    pub fn solve(input: &str) -> usize {
        let (maze, entrance, total_keys) = parse(input);
        let graph = build_graph(&maze, entrance, total_keys);
        let mut queue = VecDeque::from([(total_keys, 0_usize, 0)]);
        while let Some((current_key, mut keys, steps)) = queue.pop_front() {
            if current_key != total_keys {
                keys |= 1 << current_key;
            }
            if keys.count_ones() == total_keys {
                return steps;
            }
            for (next_key, &(steps_needed, doors)) in
                (0..total_keys).zip(graph[current_key as usize].iter().take(total_keys as usize))
            {
                if keys & (1 << next_key) == 1 {
                    continue;
                }
                if keys & doors != doors {
                    continue;
                }
                queue.push_back((next_key, keys, steps + steps_needed))
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use crate::day_18::parse;

    pub fn solve(input: &str) -> usize {
        0
    }
}

pub fn main(test: bool) {
    let test_input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_18_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
