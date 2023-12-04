//! https://adventofcode.com/2018/day/10
//! https://adventofcode.com/2018/day/10/input

use std::{fs::read_to_string, time::Instant};

use itertools::Itertools;
use regex::Regex;

struct Light {
    x: isize,
    y: isize,
    horizontal: isize,
    vertical: isize,
}

impl From<&str> for Light {
    fn from(string: &str) -> Self {
        let pattern = Regex::new(r"-?\d+").unwrap();
        let mut numbers = pattern.find_iter(string);
        Self {
            x: numbers.next().unwrap().as_str().parse().unwrap(),
            y: numbers.next().unwrap().as_str().parse().unwrap(),
            horizontal: numbers.next().unwrap().as_str().parse().unwrap(),
            vertical: numbers.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Light> {
    input.lines().map(Light::from).collect()
}

fn is_aligned(lights: &[Light]) -> bool {
    (lights.iter().map(|light| light.y).max().unwrap()
        - lights.iter().map(|light| light.y).min().unwrap())
        <= 10
}

fn move_lights(lights: &mut Vec<Light>) -> (usize, String) {
    let mut steps = 0;
    while !is_aligned(lights) {
        lights.iter_mut().for_each(|light| {
            light.x += light.horizontal;
            light.y += light.vertical;
        });
        steps += 1;
    }
    let (min_x, min_y, max_x, max_y) = lights.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(min_x, min_y, max_x, max_y), light| {
            (
                min_x.min(light.x),
                min_y.min(light.y),
                max_x.max(light.x),
                max_y.max(light.y),
            )
        },
    );
    let mut result = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for light in lights {
        result[(light.y - min_y) as usize][(light.x - min_x) as usize] = '#';
    }
    (
        steps,
        result
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n"),
    )
}

pub mod part1 {
    use super::{move_lights, parse};

    pub fn solve(input: &str) -> String {
        let mut lights = parse(input);
        move_lights(&mut lights).1
    }
}

pub mod part2 {
    use super::{move_lights, parse};

    pub fn solve(input: &str) -> usize {
        let mut lights = parse(input);
        move_lights(&mut lights).0
    }
}

pub fn main(test: bool) {
    let test_input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_10_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
