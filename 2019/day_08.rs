//! https://adventofcode.com/2019/day/8
//! https://adventofcode.com/2019/day/8/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect()
}

fn make_layers(pixels: Vec<usize>, w: usize, h: usize) -> (Vec<Vec<Vec<usize>>>, usize) {
    let dim = w * h;
    let mut image = vec![vec![vec![0; w]; h]; pixels.len() / dim];
    let mut fewest_0_digits = (usize::MAX, usize::MAX);
    for l in 0..image.len() {
        let mut zeroes = 0;
        for i in 0..h {
            for j in 0..w {
                let pixel = pixels[l * dim + i * w + j];
                if pixel == 0 {
                    zeroes += 1;
                }
                image[l][i][j] = pixel;
            }
        }
        if zeroes < fewest_0_digits.1 {
            fewest_0_digits = (l, zeroes)
        }
    }
    (image, fewest_0_digits.0)
}

pub mod part1 {
    use super::{make_layers, parse};

    pub fn solve(input: &str, w: usize, h: usize) -> usize {
        let pixels = parse(input);
        let (layers, layer) = make_layers(pixels, w, h);
        layers[layer]
            .iter()
            .flatten()
            .filter(|&&pixel| pixel == 1)
            .count()
            * layers[layer]
                .iter()
                .flatten()
                .filter(|&&pixel| pixel == 2)
                .count()
    }
}

pub mod part2 {
    use itertools::Itertools;

    use super::{make_layers, parse};

    pub fn solve(input: &str, w: usize, h: usize) -> String {
        let pixels = parse(input);
        let (layers, _) = make_layers(pixels, w, h);
        let mut image = vec![vec![0; w]; h];
        for (i, row) in image.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let mut color = 0;
                for layer in &layers {
                    if layer[i][j] != 2 {
                        color = layer[i][j];
                        break;
                    }
                }
                *pixel = color;
            }
        }
        image
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&pixel| if pixel == 1 { "#" } else { " " })
                    .join("")
            })
            .join("\n")
    }
}

pub fn main(test: bool) {
    let test_input = "0222112222120000".to_owned();
    let (puzzle_input, (w, h)) = if test {
        (test_input, (2, 2))
    } else {
        (read_to_string("../inputs/2019/day_08_input.txt").unwrap(), (25, 6))
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, w, h));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, w, h));
    println!("Run in {:?}", start.elapsed());
}
