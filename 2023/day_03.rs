//! https://adventofcode.com/2023/day/3
//! https://adventofcode.com/2023/day/3/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<Vec<char>>;

fn parse(input: &str) -> Parsed {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_numbers_near(lines: [Option<&Vec<char>>; 3], j: usize) -> Vec<usize> {
    let mut result = vec![];
    for (l, line) in lines.iter().enumerate() {
        if l % 2 == 0 {
            if let Some(line) = line {
                if j > 0 && line[j - 1].is_ascii_digit() {
                    let parsed = expand_number(line, j - 1);
                    result.push(parsed);
                    if !line[j].is_ascii_digit()
                        && j + 1 < line.len()
                        && line[j + 1].is_ascii_digit()
                    {
                        result.push(expand_number(line, j + 1));
                    }
                } else if line[j].is_ascii_digit() {
                    result.push(expand_number(line, j));
                } else if j + 1 < line.len() && line[j + 1].is_ascii_digit() {
                    result.push(expand_number(line, j + 1));
                }
            }
        } else {
            let line = line.unwrap();
            if j > 0 && line[j - 1].is_ascii_digit() {
                result.push(expand_number(line, j - 1));
            }
            if j + 1 < line.len() && line[j + 1].is_ascii_digit() {
                result.push(expand_number(line, j + 1));
            }
        }
    }
    result
}

fn expand_number(line: &[char], j: usize) -> usize {
    let mut result = line[j].to_digit(10).unwrap();
    let mut left_exponent = 10;
    let mut left = j;
    while let Some(left_j) = left.checked_sub(1) {
        if line[left_j].is_ascii_digit() {
            result += line[left_j].to_digit(10).unwrap() * left_exponent;
            left_exponent *= 10;
            left = left_j;
        } else {
            break;
        }
    }
    let mut right = j + 1;
    while right < line.len() {
        if line[right].is_ascii_digit() {
            result = result * 10 + line[right].to_digit(10).unwrap();
            right += 1;
        } else {
            break;
        }
    }
    result as usize
}

pub mod part1 {
    use super::{find_numbers_near, Parsed};

    pub fn solve(chars: Parsed) -> usize {
        let mut result = 0;
        for (i, line) in chars.iter().enumerate() {
            for (j, &char) in line.iter().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    result +=
                        find_numbers_near([chars.get(i - 1), Some(&chars[i]), chars.get(i + 1)], j)
                            .into_iter()
                            .sum::<usize>();
                }
            }
        }
        result
    }
}

pub mod part2 {
    use super::{find_numbers_near, Parsed};

    pub fn solve(chars: Parsed) -> usize {
        let mut result = 0;
        for (i, line) in chars.iter().enumerate() {
            for (j, &char) in line.iter().enumerate() {
                if char == '*' {
                    let near =
                        find_numbers_near([chars.get(i - 1), Some(&chars[i]), chars.get(i + 1)], j);
                    if near.len() == 2 {
                        result += near.into_iter().product::<usize>();
                    }
                }
            }
        }
        result
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_03_input.txt").unwrap()
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
