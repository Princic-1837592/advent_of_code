//! https://adventofcode.com/2018/day/22
//! https://adventofcode.com/2018/day/22/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl Region {
    fn allows(&self, tool: Tool) -> bool {
        match (self, tool) {
            (Region::Rocky, Tool::ClimbingGear | Tool::Torch) => true,
            (Region::Wet, Tool::ClimbingGear | Tool::None) => true,
            (Region::Narrow, Tool::Torch | Tool::None) => true,
            (_, _) => false,
        }
    }
}

impl From<usize> for Region {
    fn from(risk_level: usize) -> Self {
        match risk_level % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            _ => Region::Narrow,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tool {
    Torch,
    ClimbingGear,
    None,
}

impl From<usize> for Tool {
    fn from(risk_level: usize) -> Self {
        match risk_level % 3 {
            0 => Tool::Torch,
            1 => Tool::ClimbingGear,
            _ => Tool::None,
        }
    }
}

fn parse(input: &str) -> (usize, (usize, usize)) {
    let mut lines = input.lines();
    let depth = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let mut target = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',');
    (
        depth,
        (
            target.next().unwrap().parse().unwrap(),
            target.next().unwrap().parse().unwrap(),
        ),
    )
}

fn build_cave(depth: usize, target: (usize, usize), (i, j): (usize, usize)) -> Vec<Vec<Region>> {
    let mut cave = vec![vec![0; j + 1]; i + 1];
    for i in 0..cave.len() {
        for j in 0..cave[0].len() {
            cave[i][j] = (match (i, j) {
                (0, 0) => 0,
                _ if (i, j) == target => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (x, y) => cave[x - 1][y] * cave[x][y - 1],
            } + depth)
                % 20183;
        }
    }
    let mut result = vec![vec![Region::Narrow; cave[0].len()]; cave.len()];
    for i in 0..cave.len() {
        for j in 0..cave[0].len() {
            result[i][j] = (cave[i][j] % 3).into();
        }
    }
    result
}

pub mod part1 {
    use crate::day_22::{build_cave, parse};

    pub fn solve(input: &str) -> usize {
        let (depth, target) = parse(input);
        let cave = build_cave(depth, target, target);
        let mut risk_level = 0;
        for i in 0..cave.len() {
            for j in 0..cave[0].len() {
                risk_level += cave[i][j] as usize;
            }
        }
        risk_level
    }
}

pub mod part2 {
    use crate::day_22::{build_cave, parse, Region, Tool};

    const ADJACENT: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn explore(
        cave: &[Vec<Region>],
        coord @ (i, j): (usize, usize),
        target @ (ti, tj): (usize, usize),
        tool: Tool,
        minutes: usize,
        mut min_global: usize,
        visited: &mut [Vec<Vec<usize>>],
    ) -> usize {
        if i >= cave.len() || j >= cave[0].len() {
            return usize::MAX;
        }
        let manhattan = i.max(ti) - i.min(ti) + j.max(tj) - j.min(tj);
        let prev_min = visited[i][j][tool as usize];
        if minutes + manhattan >= min_global || !cave[i][j].allows(tool) || prev_min <= minutes {
            return usize::MAX;
        }
        visited[i][j][tool as usize] = minutes;
        if coord == target {
            return minutes + 7 * (tool != Tool::Torch) as usize;
        }
        for next in
            ADJACENT.map(|(ai, aj)| ((i as isize + ai) as usize, (j as isize + aj) as usize))
        {
            for tool_diff in 0..=2 {
                let next_tool = (tool as usize + tool_diff).into();
                if cave[i][j].allows(next_tool) {
                    min_global = min_global.min(explore(
                        cave,
                        next,
                        target,
                        next_tool,
                        minutes + 7 * (tool_diff != 0) as usize + 1,
                        min_global,
                        visited,
                    ));
                }
            }
        }
        min_global
    }

    pub fn solve(input: &str) -> usize {
        let (depth, target) = parse(input);
        //   smaller is better        smaller is better          higher is better, but not too much
        let (width_coefficient, height_coefficient, len_coefficient) = (5, 1, 5);
        let cave = build_cave(
            depth,
            target,
            (target.0 * width_coefficient, target.1 * height_coefficient),
        );
        explore(
            &cave,
            (0, 0),
            target,
            Tool::Torch,
            0,
            target.0 * target.1 / len_coefficient,
            &mut vec![vec![vec![usize::MAX; 3]; cave[0].len()]; cave.len()],
        )
    }
}

pub fn main(test: bool) {
    let test_input = "depth: 510
target: 10,10"
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
