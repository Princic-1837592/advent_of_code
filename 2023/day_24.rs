//! https://adventofcode.com/2023/day/24
//! https://adventofcode.com/2023/day/24/input

use std::{
    fmt::Debug,
    fs::read_to_string,
    str::FromStr,
    time::{Duration, Instant},
};

use utils::{parsing::parse_lines, FromStr};

#[derive(Copy, Clone, Debug, FromStr)]
#[separator(',')]
struct Triple<T>
where
    T: Copy + FromStr,
    <T as FromStr>::Err: Debug,
{
    x: T,
    y: T,
    z: T,
}

#[derive(Copy, Clone, Debug, FromStr)]
#[separator('@')]
pub struct Hail {
    position: Triple<isize>,
    velocity: Triple<isize>,
}

type Parsed = Vec<Hail>;

fn parse(input: &str) -> Parsed {
    parse_lines(input)
}

pub mod part1 {
    use super::{Parsed, Triple};

    fn intersection(
        (p1, p2): (Triple<isize>, Triple<isize>),
        (p3, p4): (Triple<isize>, Triple<isize>),
    ) -> Option<Triple<f64>> {
        let a1 = (p2.y - p1.y) as f64;
        let b1 = (p1.x - p2.x) as f64;
        let c1 = a1 * p1.x as f64 + b1 * p1.y as f64;
        let a2 = (p4.y - p3.y) as f64;
        let b2 = (p3.x - p4.x) as f64;
        let c2 = a2 * p3.x as f64 + b2 * p3.y as f64;
        let determinant = a1 * b2 - a2 * b1;
        if determinant == 0.0 {
            None
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            Some(Triple { x, y, z: 0.0 })
        }
    }

    pub fn solve(hails: Parsed) -> usize {
        let mut intersections = 0;
        let points: Vec<_> = hails
            .iter()
            .map(|hail| {
                (
                    hail.position,
                    Triple {
                        x: hail.position.x + hail.velocity.x,
                        y: hail.position.y + hail.velocity.y,
                        z: 0,
                    },
                )
            })
            .collect();
        for (i, &first) in points.iter().enumerate() {
            for (j, &second) in (i + 1..).zip(&points[i + 1..]) {
                if let Some(Triple { x, y, .. }) = intersection(first, second) {
                    if (200000000000000.0..=400000000000000.0).contains(&x)
                        && (200000000000000.0..=400000000000000.0).contains(&y)
                        && hails[i].velocity.x.signum()
                            == (x - hails[i].position.x as f64).signum() as isize
                        && hails[i].velocity.y.signum()
                            == (y - hails[i].position.y as f64).signum() as isize
                        && hails[j].velocity.x.signum()
                            == (x - hails[j].position.x as f64).signum() as isize
                        && hails[j].velocity.y.signum()
                            == (y - hails[j].position.y as f64).signum() as isize
                    {
                        intersections += 1;
                    }
                }
            }
        }
        intersections
    }
}

// https://github.com/tckmn/polyaoc-2023/blob/97689dc6b5ff38c557cd885b10be425e14928958/24/rb/24.rb#L22
pub mod part2 {
    use super::Parsed;

    fn mat(hails: &[[f64; 6]], x: usize, y: usize, dx: usize, dy: usize) -> Vec<Vec<f64>> {
        let m: Vec<_> = hails
            .iter()
            .map(|h| [-h[dy], h[dx], h[y], -h[x], h[y] * h[dx] - h[x] * h[dy]])
            .collect();
        m.iter()
            .take(4)
            .map(|r| r.iter().zip(&m[m.len() - 1]).map(|(a, b)| a - b).collect())
            .collect()
    }

    fn elimination(mut m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        for i in 0..m.len() {
            let t = m[i][i];
            m[i].iter_mut().for_each(|x| *x /= t);
            for j in 1 + i..m.len() {
                let t = m[j][i];
                for k in 0..m[j].len() {
                    m[j][k] -= t * m[i][k]
                }
            }
        }
        for i in (0..m.len()).rev() {
            for j in 0..i {
                let t = m[j][i];
                for k in 0..m[j].len() {
                    m[j][k] -= t * m[i][k]
                }
            }
        }
        m
    }

    pub fn solve(hails: Parsed) -> usize {
        let array: Vec<_> = hails
            .iter()
            .map(|h| {
                [
                    h.position.x,
                    h.position.y,
                    h.position.z,
                    h.velocity.x,
                    h.velocity.y,
                    h.velocity.z,
                ]
                .map(|i| i as f64)
            })
            .collect();
        let x_y: usize = elimination(mat(&array, 0, 1, 3, 4))
            .iter()
            .take(2)
            .map(|v| *v.last().unwrap() as usize)
            .sum();
        x_y + elimination(mat(&array, 2, 1, 5, 4))
            .iter()
            .take(1)
            .map(|v| *v.last().unwrap() as usize)
            .next()
            .unwrap()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_24_input.txt").unwrap()
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
