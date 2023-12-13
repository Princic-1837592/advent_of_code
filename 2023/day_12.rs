//! https://adventofcode.com/2023/day/12
//! https://adventofcode.com/2023/day/12/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Operational,
            '?' => Self::Unknown,
            '#' => Self::Damaged,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    springs: Vec<Spring>,
    sizes: Vec<u8>,
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let springs = parts.next().unwrap().chars().map(Spring::from).collect();
        let sizes = parts
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Self { springs, sizes }
    }
}

type Parsed = Vec<Group>;

fn parse(input: &str) -> Parsed {
    input.lines().map(Group::from).collect()
}

// https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd18cl9
fn dynamic(springs: &[Spring], damaged_group: &[bool]) -> usize {
    let (n, m) = (springs.len(), damaged_group.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[n][m] = 1;
    for i in (0..=n - 1).rev() {
        let spring = springs[i];
        for j in (0..=m - 1).rev() {
            dp[i][j] = match (spring, damaged_group[j]) {
                (Spring::Operational | Spring::Unknown, false) => dp[i + 1][j + 1] + dp[i + 1][j],
                (Spring::Damaged | Spring::Unknown, true) => dp[i + 1][j + 1],
                _ => 0,
            };
        }
    }
    dp[0][0]
}

pub mod part1 {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    use super::{dynamic, Group, Parsed, Spring};

    fn arrangements(Group { mut springs, sizes }: Group) -> usize {
        let mut damaged =
            Vec::with_capacity(sizes.len() + sizes.iter().cloned().sum::<u8>() as usize + 2);
        damaged.push(false);
        for size in sizes.iter().copied() {
            damaged.resize(damaged.len() + size as usize, true);
            damaged.push(false);
        }
        springs.insert(0, Spring::Operational);
        springs.push(Spring::Operational);
        dynamic(&springs, &damaged)
    }

    pub fn solve(groups: Parsed) -> usize {
        groups.into_par_iter().map(arrangements).sum()
    }
}

pub mod part2 {
    use std::iter::repeat;

    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    use super::{dynamic, Group, Parsed, Spring};

    fn arrangements(Group { mut springs, sizes }: Group) -> usize {
        let mut damaged =
            Vec::with_capacity((sizes.len() + sizes.iter().cloned().sum::<u8>() as usize) * 5 + 2);
        damaged.push(false);
        for size in [sizes]
            .into_iter()
            .flat_map(|n| repeat(n).take(5))
            .flatten()
        {
            damaged.resize(damaged.len() + size as usize, true);
            damaged.push(false);
        }
        springs.push(Spring::Unknown);
        springs = repeat(springs).take(5).flatten().collect();
        springs.pop();
        springs.insert(0, Spring::Operational);
        springs.push(Spring::Operational);
        dynamic(&springs, &damaged)
    }

    pub fn solve(groups: Parsed) -> usize {
        groups.into_par_iter().map(arrangements).sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
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
