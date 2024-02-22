//! https://adventofcode.com/2023/day/17
//! https://adventofcode.com/2023/day/17/input

use std::{
    cmp::Ordering,
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - '0' as usize).collect())
        .collect()
}

type Coord = (usize, usize);

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    position: Coord,
    heat_loss: usize,
    horizontal: bool,
}

impl State {
    fn new(position: Coord, heat_loss: usize, direction: bool) -> Self {
        Self {
            position,
            heat_loss,
            horizontal: direction,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            (self.position.0 + self.position.1)
                .cmp(&(other.position.0 + other.position.1))
                .then((-(self.heat_loss as isize)).cmp(&-(other.heat_loss as isize))),
        )
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.position.0 + self.position.1)
            .cmp(&(other.position.0 + other.position.1))
            .then((-(self.heat_loss as isize)).cmp(&-(other.heat_loss as isize)))
    }
}

pub mod part1 {
    use std::collections::BinaryHeap;

    use super::{Parsed, State};

    pub fn solve(map: Parsed) -> usize {
        let (h, w) = (map.len(), map[0].len());
        let target = (h - 1, w - 1);
        let mut queue =
            BinaryHeap::from([State::new((0, 0), 0, false), State::new((0, 0), 0, true)]);
        let mut min = usize::MAX;
        let mut visited = vec![vec![[usize::MAX; 2]; w]; h];
        while let Some(State {
                           position: position @ (i, j),
                           heat_loss,
                           horizontal,
                       }) = queue.pop()
        {
            if heat_loss >= min {
                continue;
            }
            if position == target {
                min = heat_loss;
            }
            if heat_loss > visited[i][j][horizontal as usize] {
                continue;
            }
            visited[i][j][horizontal as usize] = heat_loss;
            if horizontal {
                let (mut heat_loss_up, mut heat_loss_down) = (heat_loss, heat_loss);
                for di in 1..=3 {
                    if i >= di {
                        let ni = i - di;
                        heat_loss_up += map[ni][j];
                        if heat_loss_up < visited[ni][j][false as usize] {
                            queue.push(State::new((ni, j), heat_loss_up, false));
                        }
                    }
                    if h - i > di {
                        let ni = i + di;
                        heat_loss_down += map[ni][j];
                        if heat_loss_down < visited[ni][j][false as usize] {
                            queue.push(State::new((ni, j), heat_loss_down, false));
                        }
                    }
                }
            } else {
                let (mut heat_loss_left, mut heat_loss_right) = (heat_loss, heat_loss);
                for dj in 1..=3 {
                    if j >= dj {
                        let nj = j - dj;
                        heat_loss_left += map[i][nj];
                        if heat_loss_left < visited[i][nj][true as usize] {
                            queue.push(State::new((i, nj), heat_loss_left, true));
                        }
                    }
                    if w - j > dj {
                        let nj = j + dj;
                        heat_loss_right += map[i][nj];
                        if heat_loss_right < visited[i][nj][true as usize] {
                            queue.push(State::new((i, nj), heat_loss_right, true));
                        }
                    }
                }
            }
        }
        min
    }
}

pub mod part2 {
    use std::collections::BinaryHeap;

    use super::{Parsed, State};

    pub fn solve(map: Parsed) -> usize {
        let (h, w) = (map.len(), map[0].len());
        let target = (h - 1, w - 1);
        let mut queue =
            BinaryHeap::from([State::new((0, 0), 0, false), State::new((0, 0), 0, true)]);
        let mut min = usize::MAX;
        let mut visited = vec![vec![[usize::MAX; 2]; w]; h];
        while let Some(State {
                           position: position @ (i, j),
                           heat_loss,
                           horizontal,
                       }) = queue.pop()
        {
            if heat_loss >= min {
                continue;
            }
            if position == target {
                min = heat_loss;
            }
            if heat_loss > visited[i][j][horizontal as usize] {
                continue;
            }
            visited[i][j][horizontal as usize] = heat_loss;
            if horizontal {
                let (mut heat_loss_up, mut heat_loss_down) = (heat_loss, heat_loss);
                for di in 1..=10 {
                    if i >= di {
                        let ni = i - di;
                        heat_loss_up += map[ni][j];
                        if di >= 4 && heat_loss_up < visited[ni][j][false as usize] {
                            queue.push(State::new((ni, j), heat_loss_up, false));
                        }
                    }
                    if h - i > di {
                        let ni = i + di;
                        heat_loss_down += map[ni][j];
                        if di >= 4 && heat_loss_down < visited[ni][j][false as usize] {
                            queue.push(State::new((ni, j), heat_loss_down, false));
                        }
                    }
                }
            } else {
                let (mut heat_loss_left, mut heat_loss_right) = (heat_loss, heat_loss);
                for dj in 1..=10 {
                    if j >= dj {
                        let nj = j - dj;
                        heat_loss_left += map[i][nj];
                        if dj >= 4 && heat_loss_left < visited[i][nj][true as usize] {
                            queue.push(State::new((i, nj), heat_loss_left, true));
                        }
                    }
                    if w - j > dj {
                        let nj = j + dj;
                        heat_loss_right += map[i][nj];
                        if dj >= 4 && heat_loss_right < visited[i][nj][true as usize] {
                            queue.push(State::new((i, nj), heat_loss_right, true));
                        }
                    }
                }
            }
        }
        min
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_17_input.txt").unwrap()
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
