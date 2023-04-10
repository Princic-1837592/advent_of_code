//! https://adventofcode.com/2019/day/18
//! https://adventofcode.com/2019/day/18/input

use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap, VecDeque},
    fs::read_to_string,
    time::Instant,
};

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

fn build_graph(
    maze: &Vec<Vec<State>>,
    entrances: Vec<Coord>,
    total_keys: u32,
) -> Vec<Vec<(usize, usize, usize)>> {
    fn keys_bfs(
        maze: &Vec<Vec<State>>,
        coord: Coord,
        total_keys: u32,
    ) -> Vec<(usize, usize, usize)> {
        let mut queue = VecDeque::from([(coord, 0_usize, 0_usize, 0)]);
        let mut result = vec![(usize::MAX, 0, 0); total_keys as usize];
        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
        let mut keys_found = 0;
        while let Some(((i, j), mut doors, mut keys, steps)) = queue.pop_front() {
            if visited[i][j] || keys_found == total_keys {
                continue;
            }
            visited[i][j] = true;
            match maze[i][j] {
                State::Wall => continue,
                State::Space => {}
                State::Key(key) => {
                    keys |= 1 << key;
                    result[key] = (steps, doors, keys);
                    keys_found += 1;
                }
                State::Door(door) => doors |= 1 << door,
            }
            for next in
                NEIGHBOURS.map(|(di, dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
            {
                queue.push_back((next, doors, keys, steps + 1));
            }
        }
        result
    }
    let mut graph = vec![vec![]; total_keys as usize + entrances.len()];
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let &State::Key(key) = cell {
                graph[key] = keys_bfs(maze, (i, j), total_keys);
            }
        }
    }
    for (i, entrance) in (total_keys..).zip(entrances) {
        graph[i as usize] = keys_bfs(maze, entrance, total_keys);
    }
    graph
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BfsState {
    steps: usize,
    current_keys: Vec<u32>,
    keys: usize,
    last_robot: usize,
}

impl BfsState {
    fn new(steps: usize, current_key: Vec<u32>, keys: usize, last_robot: usize) -> Self {
        Self {
            steps,
            current_keys: current_key,
            keys,
            last_robot,
        }
    }
}

impl PartialOrd<Self> for BfsState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.steps
                .cmp(&other.steps)
                .reverse()
                .then_with(|| self.keys.count_ones().cmp(&other.keys.count_ones())),
        )
    }
}

impl Ord for BfsState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve_with_robots(
    total_keys: u32,
    graph: Vec<Vec<(usize, usize, usize)>>,
    robots: Vec<u32>,
) -> usize {
    let mut queue = BinaryHeap::from([BfsState::new(0, robots.clone(), 0, 0)]);
    let mut seen: HashMap<(usize, u32), usize> = HashMap::new();
    while let Some(BfsState {
        steps,
        mut current_keys,
        mut keys,
        last_robot,
    }) = queue.pop()
    {
        let current_key = current_keys[last_robot];
        if current_key < total_keys {
            keys |= 1 << current_key;
        }
        if keys.count_ones() == total_keys {
            return steps;
        }
        match seen.entry((keys, current_key)) {
            Entry::Occupied(entry) => {
                if *entry.get() <= steps {
                    continue;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(steps);
            }
        }
        for robot in 0..robots.len() {
            for (next_key, &(steps_needed, doors, keys_on_path)) in
                (0..total_keys).zip(graph[current_keys[robot] as usize].iter())
            {
                if steps_needed == usize::MAX
                    || keys & (1 << next_key) != 0
                    || keys & doors != doors
                {
                    continue;
                }
                current_keys[robot] = next_key;
                queue.push(BfsState::new(
                    steps.saturating_add(steps_needed),
                    current_keys.clone(),
                    keys | keys_on_path,
                    robot,
                ));
                current_keys[robot] = current_key;
            }
        }
    }
    unreachable!()
}

pub mod part1 {
    use crate::day_18::{build_graph, parse, solve_with_robots};

    pub fn solve(input: &str) -> usize {
        let (maze, entrance, total_keys) = parse(input);
        let graph = build_graph(&maze, vec![entrance], total_keys);
        solve_with_robots(total_keys, graph, vec![total_keys])
    }
}

pub mod part2 {
    use crate::day_18::{build_graph, parse, solve_with_robots, State};

    pub fn solve(input: &str) -> usize {
        let (mut maze, (ei, ej), total_keys) = parse(input);
        for (i, j) in [
            (ei, ej),
            (ei, ej + 1),
            (ei + 1, ej),
            (ei - 1, ej),
            (ei, ej - 1),
        ] {
            maze[i][j] = State::Wall;
        }
        let graph = build_graph(
            &maze,
            vec![
                (ei - 1, ej - 1),
                (ei - 1, ej + 1),
                (ei + 1, ej - 1),
                (ei + 1, ej + 1),
            ],
            total_keys,
        );
        solve_with_robots(
            total_keys,
            graph,
            vec![total_keys, total_keys + 1, total_keys + 2, total_keys + 3],
        )
    }
}

pub fn main(test: bool) {
    let test_input = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############"
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
