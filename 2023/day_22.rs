//! https://adventofcode.com/2023/day/22
//! https://adventofcode.com/2023/day/22/input

use std::{
    collections::HashSet,
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Cube {
    z: usize,
    x: usize,
    y: usize,
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Brick {
    start: Cube,
    end: Cube,
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let mut ends: Vec<_> = value.split('~').map(Cube::from).collect();
        ends.sort();
        Self {
            start: ends[0],
            end: ends[1],
        }
    }
}

type Parsed = Vec<Brick>;

fn parse(input: &str) -> Parsed {
    let mut result: Vec<_> = input.lines().map(Brick::from).collect();
    result.sort();
    result
}

fn make_space(bricks: &mut Parsed) -> Vec<Vec<Vec<Option<usize>>>> {
    let max_x = bricks.iter().map(|b| b.start.x.max(b.end.x)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.start.y.max(b.end.y)).max().unwrap();
    let max_z = bricks.iter().map(|b| b.start.z.max(b.end.z)).max().unwrap();
    let mut space = vec![vec![vec![None; max_z + 2]; max_y + 1]; max_x + 1];
    for (b, brick) in bricks.iter().enumerate() {
        for x in &mut space[brick.start.x..=brick.end.x] {
            for y in &mut x[brick.start.y..=brick.end.y] {
                for z in &mut y[brick.start.z..=brick.end.z] {
                    *z = Some(b);
                }
            }
        }
    }
    space
}

fn drop((b, brick): (usize, &mut Brick), space: &mut [Vec<Vec<Option<usize>>>]) {
    let mut max_z = 0;
    for x in &space[brick.start.x..=brick.end.x] {
        for y in &x[brick.start.y..=brick.end.y] {
            let mut z = brick.start.z;
            while z > 1 && y[z - 1].is_none() {
                z -= 1;
            }
            max_z = max_z.max(z);
        }
    }
    for x in &mut space[brick.start.x..=brick.end.x] {
        for y in &mut x[brick.start.y..=brick.end.y] {
            for z in &mut y[brick.start.z..=brick.end.z] {
                *z = None;
            }
        }
    }
    brick.end.z -= brick.start.z - max_z;
    brick.start.z = max_z;
    for x in &mut space[brick.start.x..=brick.end.x] {
        for y in &mut x[brick.start.y..=brick.end.y] {
            for z in &mut y[brick.start.z..=brick.end.z] {
                *z = Some(b);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Connection {
    above: HashSet<usize>,
    below: HashSet<usize>,
}

fn make_connections(bricks: &mut Parsed, space: &mut [Vec<Vec<Option<usize>>>]) -> Vec<Connection> {
    let mut connections = vec![Connection::default(); bricks.len()];
    for (b, brick) in bricks.iter().enumerate() {
        for x in &space[brick.start.x..=brick.end.x] {
            for y in &x[brick.start.y..=brick.end.y] {
                if let Some(above) = y[brick.end.z + 1] {
                    connections[b].above.insert(above);
                    connections[above].below.insert(b);
                }
            }
        }
    }
    connections
}

pub mod part1 {
    use super::{drop, make_connections, make_space, Parsed};

    pub fn solve(mut bricks: Parsed) -> usize {
        let mut space = make_space(&mut bricks);
        bricks
            .iter_mut()
            .enumerate()
            .for_each(|brick| drop(brick, &mut space));
        let connections = make_connections(&mut bricks, &mut space);
        let mut result = 0;
        'brick: for connection in &connections {
            for &above in &connection.above {
                if connections[above].below.len() == 1 {
                    continue 'brick;
                }
            }
            result += 1;
        }
        result
    }
}

pub mod part2 {
    use std::collections::{HashSet, VecDeque};

    use super::{drop, make_connections, make_space, Parsed};

    pub fn solve(mut bricks: Parsed) -> usize {
        let mut space = make_space(&mut bricks);
        bricks
            .iter_mut()
            .enumerate()
            .for_each(|brick| drop(brick, &mut space));
        let connections = make_connections(&mut bricks, &mut space);
        let mut result = 0;
        for b in 0..bricks.len() {
            let mut queue = VecDeque::from([b]);
            let mut would_fall: HashSet<usize> = HashSet::new();
            while let Some(brick) = queue.pop_front() {
                would_fall.insert(brick);
                for &above in &connections[brick].above {
                    if connections[above]
                        .below
                        .iter()
                        .filter(|b| !would_fall.contains(b))
                        .count()
                        == 0
                    {
                        queue.push_back(above);
                    }
                }
            }
            result += would_fall.len() - 1;
        }
        result
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_22_input.txt").unwrap()
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
