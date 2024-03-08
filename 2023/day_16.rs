//! https://adventofcode.com/2023/day/16
//! https://adventofcode.com/2023/day/16/input

use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Empty,
    MirrorR,
    MirrorL,
    SplitV,
    SplitH,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '\\' => Tile::MirrorR,
            '/' => Tile::MirrorL,
            '-' => Tile::SplitH,
            '|' => Tile::SplitV,
            _ => unreachable!(),
        }
    }
}

type Coord = (usize, usize);
type Direction = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Beam {
    position: Coord,
    direction: Direction,
}

impl Beam {
    fn new(position: Coord, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

type Parsed = Vec<Vec<Tile>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect()
}

fn find_energized(
    tiles: &Parsed,
    visited: &mut HashSet<Beam>,
    energized: &mut [Vec<bool>],
    queue: &mut VecDeque<Beam>,
    start_from: Beam,
) -> usize {
    visited.clear();
    energized.iter_mut().for_each(|r| r.fill(false));
    queue.clear();
    queue.push_back(start_from);
    let mut energized_count = 0;
    while let Some(
        beam @ Beam {
            position: (i, j),
            direction: direction @ (di, dj),
        },
    ) = queue.pop_front()
    {
        if i >= tiles.len() || j >= tiles[0].len() {
            continue;
        }
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);
        if !energized[i][j] {
            energized_count += 1;
            energized[i][j] = true;
        }
        match tiles[i][j] {
            Tile::MirrorR => {
                let direction @ (di, dj) = (dj, di);
                queue.push_back(Beam::new(
                    (i.wrapping_add(di), j.wrapping_add(dj)),
                    direction,
                ))
            }
            Tile::MirrorL => {
                let direction @ (di, dj) = ((-(dj as isize)) as usize, (-(di as isize)) as usize);
                queue.push_back(Beam::new(
                    (i.wrapping_add(di), j.wrapping_add(dj)),
                    direction,
                ))
            }
            Tile::SplitV if dj != 0 => {
                for direction @ (di, dj) in [(usize::MAX, 0), (1, 0)] {
                    queue.push_back(Beam::new(
                        (i.wrapping_add(di), j.wrapping_add(dj)),
                        direction,
                    ));
                }
            }
            Tile::SplitH if di != 0 => {
                for direction @ (di, dj) in [(0, usize::MAX), (0, 1)] {
                    queue.push_back(Beam::new(
                        (i.wrapping_add(di), j.wrapping_add(dj)),
                        direction,
                    ));
                }
            }
            Tile::SplitV | Tile::SplitH | Tile::Empty => queue.push_back(Beam::new(
                (i.wrapping_add(di), j.wrapping_add(dj)),
                direction,
            )),
        }
    }
    energized_count
}

pub mod part1 {
    use std::collections::{HashSet, VecDeque};

    use super::{find_energized, Beam, Parsed};

    pub fn solve(tiles: Parsed) -> usize {
        let mut visited: HashSet<Beam> = HashSet::new();
        let mut energized = vec![vec![false; tiles[0].len()]; tiles.len()];
        let mut queue = VecDeque::new();
        find_energized(
            &tiles,
            &mut visited,
            &mut energized,
            &mut queue,
            Beam::new((0, 0), (0, 1)),
        )
    }
}

pub mod part2 {
    use std::collections::{HashSet, VecDeque};

    use super::{find_energized, Beam, Parsed};

    pub fn solve(tiles: Parsed) -> usize {
        let mut visited: HashSet<Beam> = HashSet::new();
        let mut energized = vec![vec![false; tiles[0].len()]; tiles.len()];
        let mut queue = VecDeque::new();
        (0..tiles[0].len())
            .flat_map(|j| {
                [
                    find_energized(
                        &tiles,
                        &mut visited,
                        &mut energized,
                        &mut queue,
                        Beam::new((0, j), (1, 0)),
                    ),
                    find_energized(
                        &tiles,
                        &mut visited,
                        &mut energized,
                        &mut queue,
                        Beam::new((tiles.len() - 1, j), (usize::MAX, 0)),
                    ),
                ]
            })
            .max()
            .unwrap()
            .max(
                (0..tiles.len())
                    .flat_map(|i| {
                        [
                            find_energized(
                                &tiles,
                                &mut visited,
                                &mut energized,
                                &mut queue,
                                Beam::new((i, 0), (0, 1)),
                            ),
                            find_energized(
                                &tiles,
                                &mut visited,
                                &mut energized,
                                &mut queue,
                                Beam::new((i, tiles[0].len() - 1), (0, usize::MAX)),
                            ),
                        ]
                    })
                    .max()
                    .unwrap(),
            )
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_16_input.txt").unwrap()
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
