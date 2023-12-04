//! https://adventofcode.com/2019/day/12
//! https://adventofcode.com/2019/day/12/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    dx: isize,
    dy: isize,
    dz: isize,
}

impl From<&str> for Moon {
    fn from(string: &str) -> Self {
        let pattern = Regex::new(r"-?\d+").unwrap();
        let mut numbers = pattern.find_iter(string);
        Moon {
            x: numbers.next().unwrap().as_str().parse().unwrap(),
            y: numbers.next().unwrap().as_str().parse().unwrap(),
            z: numbers.next().unwrap().as_str().parse().unwrap(),
            ..Default::default()
        }
    }
}

fn parse(input: &str) -> [Moon; 4] {
    let mut result = [Moon::default(); 4];
    for (i, line) in input.lines().enumerate() {
        result[i] = line.into();
    }
    result
}

fn step(moons: &mut [Moon; 4]) {
    for m in 0..moons.len() {
        for n in m + 1..moons.len() {
            let gx = (moons[m].x - moons[n].x).signum();
            moons[m].dx -= gx;
            moons[n].dx += gx;
            let gy = (moons[m].y - moons[n].y).signum();
            moons[m].dy -= gy;
            moons[n].dy += gy;
            let gz = (moons[m].z - moons[n].z).signum();
            moons[m].dz -= gz;
            moons[n].dz += gz;
        }
    }
    for moon in moons {
        moon.x += moon.dx;
        moon.y += moon.dy;
        moon.z += moon.dz;
    }
}

pub mod part1 {
    use super::{parse, step};

    pub fn solve(input: &str, steps: usize) -> usize {
        let mut moons = parse(input);
        for _ in 0..steps {
            step(&mut moons);
        }
        moons
            .iter()
            .map(|moon| {
                (moon.x.unsigned_abs() + moon.y.unsigned_abs() + moon.z.unsigned_abs())
                    * (moon.dx.unsigned_abs() + moon.dy.unsigned_abs() + moon.dz.unsigned_abs())
            })
            .sum()
    }
}

pub mod part2 {
    use super::{parse, step};

    fn gcd(a: usize, b: usize) -> usize {
        if a == 0 {
            b
        } else {
            gcd(b % a, a)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    pub fn solve(input: &str) -> usize {
        let mut moons = parse(input);
        let initial_x: Vec<_> = moons.iter().map(|moon| (moon.x, moon.dx)).collect();
        let initial_y: Vec<_> = moons.iter().map(|moon| (moon.y, moon.dy)).collect();
        let initial_z: Vec<_> = moons.iter().map(|moon| (moon.z, moon.dz)).collect();
        let (mut x_cycle, mut y_cycle, mut z_cycle) = (0, 0, 0);
        let mut steps = 0;
        while x_cycle == 0 || y_cycle == 0 || z_cycle == 0 {
            step(&mut moons);
            steps += 1;
            let (xs, (ys, zs)): (Vec<_>, (Vec<_>, Vec<_>)) = moons
                .iter()
                .map(|moon| ((moon.x, moon.dx), ((moon.y, moon.dy), (moon.z, moon.dz))))
                .unzip();
            if x_cycle == 0 && xs == initial_x {
                x_cycle = steps;
            }
            if y_cycle == 0 && ys == initial_y {
                y_cycle = steps;
            }
            if z_cycle == 0 && zs == initial_z {
                z_cycle = steps;
            }
        }
        lcm(x_cycle, lcm(y_cycle, z_cycle))
    }
}

pub fn main(test: bool) {
    let test_input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"
        .to_owned();
    let (puzzle_input, steps) = if test {
        (test_input, 100)
    } else {
        (read_to_string("inputs/day_12_input.txt").unwrap(), 1000)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, steps));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
