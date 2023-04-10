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

fn max_distance_bfs(maze: &Vec<Vec<State>>, coord: Coord) -> usize {
    unimplemented!()
}

pub mod part1 {
    use std::collections::VecDeque;

    use crate::day_18::{max_distance_bfs, parse, State, NEIGHBOURS};

    pub fn solve(input: &str) -> usize {
        let (maze, entrance, total_keys) = parse(input);
        let mut max_distance = vec![usize::MAX; total_keys as usize + 1];
        for (i, key) in max_distance
            .iter_mut()
            .enumerate()
            .take(total_keys as usize)
        {
            *key = max_distance_bfs(&maze, (0, 0));
        }
        max_distance[total_keys as usize] = max_distance_bfs(&maze, entrance);
        let mut queue = VecDeque::from([(entrance, 0_usize, 0, (usize::MAX, usize::MAX))]);
        let mut max_collected_keys: u32 = 0;
        while let Some((coord @ (i, j), mut keys, steps, from)) = queue.pop_front() {
            if steps > 5 {
                break;
            }
            dbg!((coord, keys.count_ones(), steps));
            if keys.count_ones() < max_collected_keys.saturating_sub(5) {
                continue;
            } else if keys.count_ones() > max_collected_keys {
                max_collected_keys = keys.count_ones();
                dbg!(max_collected_keys);
            }
            let mut picked_key = false;
            match maze[i][j] {
                State::Wall => continue,
                State::Space => {}
                State::Key(key) => {
                    if keys & (1 << key) == 0 {
                        keys |= 1 << key;
                        picked_key = true;
                    }
                }
                State::Door(door) => {
                    if keys & (1 << door) == 0 {
                        continue;
                    }
                }
            }
            if picked_key && keys.count_ones() == total_keys {
                return steps;
            }
            for next in
                NEIGHBOURS.map(|(di, dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
            {
                if next == from && !picked_key {
                    continue;
                }
                queue.push_back((next, keys, steps + 1, coord));
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
    let test_input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
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
