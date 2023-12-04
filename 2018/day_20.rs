//! https://adventofcode.com/2018/day/20
//! https://adventofcode.com/2018/day/20/input

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

type Coord = (isize, isize);

#[derive(Clone, Debug)]
enum Match {
    N,
    E,
    S,
    W,
    Many(Vec<Match>),
    Or(Vec<Match>),
    None,
}

impl Match {
    fn parse(chars: Vec<char>) -> Self {
        fn push(options: &mut Vec<Match>, option: &mut Vec<Match>) {
            match option.len() {
                0 => {
                    options.push(Match::None);
                }
                1 => {
                    options.push(option.pop().unwrap());
                }
                _ => {
                    options.push(Match::Many(option.clone()));
                    option.clear()
                }
            }
        }
        fn internal(chars: &Vec<char>, mut i: usize) -> (Match, usize) {
            let mut options = Vec::new();
            let mut option = Vec::new();
            loop {
                i = match chars[i] {
                    '|' => {
                        push(&mut options, &mut option);
                        i
                    }
                    '(' => {
                        let (group, i) = internal(chars, i + 1);
                        match group {
                            Match::Many(others) => option.extend(others),
                            _ => option.push(group),
                        }
                        i
                    }
                    ')' | '$' => {
                        push(&mut options, &mut option);
                        break;
                    }
                    literal => {
                        option.push(Match::from(literal));
                        i
                    }
                } + 1;
            }
            (
                match options.len() {
                    0 => Match::None,
                    1 => options.pop().unwrap(),
                    _ => Match::Or(options),
                },
                i,
            )
        }
        internal(&chars, 0).0
    }
}

impl From<char> for Match {
    fn from(char: char) -> Self {
        match char {
            'N' => Match::N,
            'E' => Match::E,
            'S' => Match::S,
            'W' => Match::W,
            _ => panic!("Invalid match: {}", char),
        }
    }
}

fn parse(input: &str) -> Match {
    Match::parse(input.chars().skip(1).collect())
}

#[allow(unused)]
fn to_string(graph: &HashMap<Coord, HashSet<Coord>>) -> String {
    let mut min_i = 0;
    let mut max_i = 0;
    let mut min_j = 0;
    let mut max_j = 0;
    for ((i, j), rooms) in graph.iter() {
        min_i = min_i.min(*i);
        max_i = max_i.max(*i);
        min_j = min_j.min(*j);
        max_j = max_j.max(*j);
        for (i, j) in rooms {
            min_i = min_i.min(*i);
            max_i = max_i.max(*i);
            min_j = min_j.min(*j);
            max_j = max_j.max(*j);
        }
    }
    let width = (max_j - min_j + 1) as usize;
    let height = (max_i - min_i + 1) as usize;
    let mut matrix = vec![vec!['.'; width * 2 + 1]; height * 2 + 1];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if i == matrix.len() - 1 || j == matrix[0].len() - 1 || i % 2 == 0 || j % 2 == 0 {
                matrix[i][j] = '#';
            }
        }
    }
    for ((i, j), rooms) in graph.iter() {
        let fi = (i - min_i) as usize * 2 + 1;
        let fj = (j - min_j) as usize * 2 + 1;
        for (i, j) in rooms {
            let ti = (i - min_i) as usize * 2 + 1;
            let tj = (j - min_j) as usize * 2 + 1;
            matrix[(fi + ti) / 2][(fj + tj) / 2] = if fi == ti { '|' } else { '-' };
        }
    }
    matrix[-min_i as usize * 2 + 1][-min_j as usize * 2 + 1] = 'X';
    matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn explore(regex: &Match, coord: Coord) -> HashMap<Coord, HashSet<Coord>> {
    fn mirror(graph: &mut HashMap<Coord, HashSet<Coord>>) {
        for ((i, j), rooms) in graph.clone().iter() {
            for (ri, rj) in rooms {
                graph
                    .entry((*ri, *rj))
                    .or_insert_with(HashSet::new)
                    .insert((*i, *j));
            }
        }
    }

    fn internal(
        regex: &Match,
        coord @ (i, j): Coord,
        graph: &mut HashMap<Coord, HashSet<Coord>>,
    ) -> HashSet<Coord> {
        match regex {
            Match::N => {
                let next = (i - 1, j);
                graph.entry(coord).or_insert_with(HashSet::new).insert(next);
                HashSet::from([next])
            }
            Match::E => {
                let next = (i, j + 1);
                graph.entry(coord).or_insert_with(HashSet::new).insert(next);
                HashSet::from([next])
            }
            Match::S => {
                let next = (i + 1, j);
                graph.entry(coord).or_insert_with(HashSet::new).insert(next);
                HashSet::from([next])
            }
            Match::W => {
                let next = (i, j - 1);
                graph.entry(coord).or_insert_with(HashSet::new).insert(next);
                HashSet::from([next])
            }
            Match::Many(many) => {
                let mut next = HashSet::from([coord]);
                for path in many {
                    next = internal(path, next.into_iter().next().unwrap(), graph);
                }
                next
            }
            Match::Or(options) => {
                let mut next = HashSet::new();
                for path in options {
                    next.extend(internal(path, coord, graph));
                }
                next
            }
            Match::None => HashSet::from([coord]),
        }
    }

    let mut result = HashMap::new();
    internal(regex, coord, &mut result);
    mirror(&mut result);
    result
}

pub mod part1 {
    use std::collections::{HashMap, HashSet, VecDeque};

    use super::{explore, parse, Coord};

    fn bfs(graph: &HashMap<Coord, HashSet<Coord>>, start: Coord) -> usize {
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::new();
        let mut distance = 0;
        while !queue.is_empty() {
            let mut next = VecDeque::new();
            for coord in queue {
                if visited.contains(&coord) {
                    continue;
                }
                visited.insert(coord);
                if let Some(rooms) = graph.get(&coord) {
                    next.extend(rooms);
                }
            }
            queue = next;
            distance += 1;
        }
        distance
    }

    pub fn solve(input: &str) -> usize {
        let regex = parse(input);
        let graph = explore(&regex, (0, 0));
        bfs(&graph, (0, 0)) - 2
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet, VecDeque};

    use super::{explore, parse, Coord};

    fn bfs(graph: &HashMap<Coord, HashSet<Coord>>, start: Coord) -> usize {
        let mut at_least_1000 = 0;
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::new();
        let mut distance = 0;
        while !queue.is_empty() {
            let mut next = VecDeque::new();
            for coord in queue {
                if visited.contains(&coord) {
                    continue;
                }
                visited.insert(coord);
                if distance >= 1000 {
                    at_least_1000 += 1;
                }
                if let Some(rooms) = graph.get(&coord) {
                    next.extend(rooms);
                }
            }
            queue = next;
            distance += 1;
        }
        at_least_1000
    }

    pub fn solve(input: &str) -> usize {
        let regex = parse(input);
        let graph = explore(&regex, (0, 0));
        bfs(&graph, (0, 0))
    }
}

pub fn main(test: bool) {
    let test_input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".to_owned();
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
