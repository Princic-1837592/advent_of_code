//! https://adventofcode.com/2019/day/20
//! https://adventofcode.com/2019/day/20/input

use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

type Coord = (usize, usize);

const NEIGHBOURS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (Vec<Vec<bool>>, HashMap<Coord, (Coord, isize)>, Coord, Coord) {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut open =
        vec![vec![false; matrix.iter().map(|line| line.len()).max().unwrap()]; matrix.len()];
    let mut tmp_portals = HashMap::new();
    let mut portals_heap = BinaryHeap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            match cell {
                '#' => open[i][j] = false,
                '.' => open[i][j] = true,
                char @ 'A'..='Z' => {
                    tmp_portals.insert((i, j), char);
                    portals_heap.push(Reverse((i, j)));
                }
                _ => {}
            }
        }
    }
    let mut portal_pairs = HashMap::new();
    while let Some(Reverse((i, j))) = portals_heap.pop() {
        if !tmp_portals.contains_key(&(i, j)) {
            continue;
        }
        let portal_fst_char = tmp_portals.remove(&(i, j)).unwrap();
        let (portal_snd_char, coord) = if tmp_portals.contains_key(&(i, j + 1)) {
            let coord = if j + 2 < open[i].len() && open[i][j + 2] {
                (i, j + 2)
            } else {
                (i, j - 1)
            };
            (tmp_portals.remove(&(i, j + 1)).unwrap(), coord)
        } else {
            let coord = if i + 2 < open.len() && open[i + 2][j] {
                (i + 2, j)
            } else {
                (i - 1, j)
            };
            (tmp_portals.remove(&(i + 1, j)).unwrap(), coord)
        };
        match portal_pairs.entry((portal_fst_char, portal_snd_char)) {
            Entry::Vacant(entry) => {
                entry.insert((coord, (usize::MAX, usize::MAX)));
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().1 = coord;
            }
        }
    }
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut portals: HashMap<_, (Coord, isize)> = HashMap::new();
    let center = (open.len() / 2, open[0].len() / 2);
    let mut inner = HashSet::new();
    let mut queue = VecDeque::from([center]);
    let mut visited = vec![vec![false; open[0].len()]; open.len()];
    while let Some(coord @ (i, j)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        match matrix[i][j] {
            ' ' | 'A'..='Z' => {
                for (ni, nj) in NEIGHBOURS.iter().filter_map(|(di, dj)| {
                    let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
                    (ni < open.len() && nj < open[0].len() && !visited[ni][nj]).then_some((ni, nj))
                }) {
                    queue.push_back((ni, nj));
                }
            }
            '.' => {
                inner.insert(coord);
            }
            _ => {}
        }
    }
    for ((fst_char, snd_char), (from, to)) in portal_pairs {
        if fst_char == 'A' && snd_char == 'A' {
            start = from;
        } else if fst_char == 'Z' && snd_char == 'Z' {
            end = from;
        } else {
            let level_diff = if inner.contains(&from) { 1 } else { -1 };
            portals.insert(from, (to, level_diff));
            portals.insert(to, (from, -level_diff));
        }
    }
    (open, portals, start, end)
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{parse, NEIGHBOURS};

    pub fn solve(input: &str) -> usize {
        let (open, portals, start, end) = parse(input);
        let mut visited = vec![vec![false; open[0].len()]; open.len()];
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((coord @ (i, j), distance)) = queue.pop_front() {
            if visited[i][j] {
                continue;
            }
            if coord == end {
                return distance;
            }
            visited[i][j] = true;
            for (ni, nj) in NEIGHBOURS.iter().filter_map(|(di, dj)| {
                let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
                (ni < open.len() && nj < open[0].len() && open[ni][nj]).then_some((ni, nj))
            }) {
                queue.push_back(((ni, nj), distance + 1));
            }
            if let Some(&(warp @ (wi, wj), _)) = portals.get(&coord) {
                if wi < visited.len() && wj < visited[0].len() {
                    queue.push_back((warp, distance + 1));
                }
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use std::collections::{HashMap, VecDeque};

    use super::{parse, NEIGHBOURS};

    pub fn solve(input: &str) -> usize {
        let (open, portals, start, end) = parse(input);
        let mut visited = vec![vec![HashMap::new(); open[0].len()]; open.len()];
        let mut queue = VecDeque::from([(start, 0, 0)]);
        while let Some((coord @ (i, j), distance, level)) = queue.pop_front() {
            if level < 0 || visited[i][j].get(&level).unwrap_or(&usize::MAX) <= &distance {
                continue;
            }
            if coord == end {
                if level == 0 {
                    return distance;
                }
                continue;
            }
            visited[i][j].insert(level, distance);
            for (ni, nj) in NEIGHBOURS.iter().filter_map(|(di, dj)| {
                let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
                (ni < open.len() && nj < open[0].len() && open[ni][nj]).then_some((ni, nj))
            }) {
                queue.push_back(((ni, nj), distance + 1, level));
            }
            if let Some(&(warp @ (wi, wj), level_diff)) = portals.get(&coord) {
                if wi < visited.len() && wj < visited[0].len() {
                    queue.push_back((warp, distance + 1, level + level_diff));
                }
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     "
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_20_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
