//! https://adventofcode.com/2017/day/20
//! https://adventofcode.com/2017/day/20/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Triple {
    fn from(string: &str) -> Self {
        let mut parts = string[1..string.len() - 1]
            .split(',')
            .map(|v| v.parse().unwrap());
        Triple {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    }
}

impl Triple {
    fn abs_sum(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct Particle {
    position: Triple,
    velocity: Triple,
    acceleration: Triple,
}

impl From<&str> for Particle {
    fn from(string: &str) -> Self {
        let mut parts = string.split(", ").map(|a| Triple::from(&a[2..]));
        Particle {
            position: parts.next().unwrap(),
            velocity: parts.next().unwrap(),
            acceleration: parts.next().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Particle> {
    input.lines().map(Particle::from).collect()
}

pub mod part1 {
    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        /*let mut particles = parse(input);
        let mut steps = vec![0; particles.len()];
        for (i, particle) in particles.iter_mut().enumerate() {
            while particle.acceleration.x != 0
                && particle.velocity.x.signum() != particle.acceleration.x.signum()
                || particle.acceleration.y != 0
                    && particle.velocity.y.signum() != particle.acceleration.y.signum()
                || particle.acceleration.z != 0
                    && particle.velocity.z.signum() != particle.acceleration.z.signum()
            {
                particle.velocity.x += particle.acceleration.x;
                particle.velocity.y += particle.acceleration.y;
                particle.velocity.z += particle.acceleration.z;
                particle.position.x += particle.velocity.x;
                particle.position.y += particle.velocity.y;
                particle.position.z += particle.velocity.z;
                steps[i] += 1;
            }
        }
        let max_steps = *steps.iter().max().unwrap() * 100;
        for (i, particle) in particles.iter_mut().enumerate() {
            while steps[i] < max_steps {
                particle.velocity.x += particle.acceleration.x;
                particle.velocity.y += particle.acceleration.y;
                particle.velocity.z += particle.acceleration.z;
                particle.position.x += particle.velocity.x;
                particle.position.y += particle.velocity.y;
                particle.position.z += particle.velocity.z;
                steps[i] += 1;
            }
        }
        particles
            .iter()
            .enumerate()
            .min_by_key(|(_, particle)| {
                particle.position.x.unsigned_abs()
                    + particle.position.y.unsigned_abs()
                    + particle.position.z.unsigned_abs()
            })
            .unwrap()
            .0*/
        let particles = parse(input);
        let mut particles: Vec<_> = particles.iter().enumerate().collect();
        particles.sort_by_key(|(_, particle)| {
            (
                particle.acceleration.abs_sum(),
                particle.velocity.abs_sum(),
                particle.position.abs_sum(),
            )
        });
        particles[0].0
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};

    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        let mut particles = parse(input);
        let mut alive = vec![true; particles.len()];
        let mut points = HashMap::new();
        for _ in 0..1000 {
            points.clear();
            for (i, particle) in particles.iter_mut().enumerate().filter(|&(i, _)| alive[i]) {
                particle.velocity.x += particle.acceleration.x;
                particle.velocity.y += particle.acceleration.y;
                particle.velocity.z += particle.acceleration.z;
                particle.position.x += particle.velocity.x;
                particle.position.y += particle.velocity.y;
                particle.position.z += particle.velocity.z;
                points
                    .entry(particle.position)
                    .or_insert_with(HashSet::new)
                    .insert(i);
            }
            for particles in points.values() {
                if particles.len() > 1 {
                    for &particle in particles {
                        alive[particle] = false;
                    }
                }
            }
        }
        alive.iter().filter(|&&alive| alive).count()
    }
}

pub fn main(test: bool) {
    let test_input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"
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
