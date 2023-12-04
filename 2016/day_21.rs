//! https://adventofcode.com/2016/day/21
//! https://adventofcode.com/2016/day/21/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Op {
    SwapPos(usize, usize),
    SwapLetter(usize, usize),
    RotSteps(isize),
    RotLetter(usize),
    Rev(usize, usize),
    Mov(usize, usize),
}

impl From<&str> for Op {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        if string.starts_with("swap p") {
            Op::SwapPos(parts[2].parse().unwrap(), parts[5].parse().unwrap())
        } else if string.starts_with("swap l") {
            Op::SwapLetter(
                parts[2].chars().next().unwrap() as usize - 'a' as usize,
                parts[5].chars().next().unwrap() as usize - 'a' as usize,
            )
        } else if string.starts_with("rotate l") {
            Op::RotSteps(-parts[2].parse::<isize>().unwrap())
        } else if string.starts_with("rotate r") {
            Op::RotSteps(parts[2].parse().unwrap())
        } else if string.starts_with("rotate b") {
            Op::RotLetter(parts[6].chars().next().unwrap() as usize - 'a' as usize)
        } else if string.starts_with("reverse") {
            Op::Rev(parts[2].parse().unwrap(), parts[4].parse().unwrap())
        } else if string.starts_with("move") {
            Op::Mov(parts[2].parse().unwrap(), parts[5].parse().unwrap())
        } else {
            panic!("Invalid move: {}", string);
        }
    }
}

fn parse(input: &str) -> Vec<Op> {
    input.lines().map(Op::from).collect()
}

pub mod part1 {
    use std::cmp::Ordering;

    use super::{parse, Op};

    pub fn solve(input: &str, len: usize) -> String {
        let operations = parse(input);
        let mut pos2letter: Vec<_> = (0..len).collect();
        let mut letter2pos: Vec<_> = (0..len).collect();
        let len = len as isize;
        let mut rot_offset = 0;
        for operation in operations {
            match operation {
                Op::SwapPos(left, right) => {
                    let (left, right) = (
                        (left as isize - rot_offset).rem_euclid(len) as usize,
                        (right as isize - rot_offset).rem_euclid(len) as usize,
                    );
                    (letter2pos[pos2letter[left]], letter2pos[pos2letter[right]]) =
                        (letter2pos[pos2letter[right]], letter2pos[pos2letter[left]]);
                    (pos2letter[left], pos2letter[right]) = (pos2letter[right], pos2letter[left]);
                }
                Op::SwapLetter(l_letter, r_letter) => {
                    (letter2pos[l_letter], letter2pos[r_letter]) =
                        (letter2pos[r_letter], letter2pos[l_letter]);
                    let (left, right) = (letter2pos[l_letter], letter2pos[r_letter]);
                    (pos2letter[left], pos2letter[right]) = (pos2letter[right], pos2letter[left]);
                }
                Op::RotSteps(steps) => {
                    rot_offset = (rot_offset + steps).rem_euclid(len);
                }
                Op::RotLetter(letter) => {
                    let index = (letter2pos[letter] as isize + rot_offset).rem_euclid(len);
                    rot_offset += 1 + index;
                    if index >= 4 {
                        rot_offset += 1;
                    }
                    rot_offset = rot_offset.rem_euclid(len);
                }
                Op::Rev(left, right) => {
                    for i in left..=(left + right) / 2 {
                        let (i, right_i) = (
                            (i as isize - rot_offset).rem_euclid(len) as usize,
                            ((right - i + left) as isize - rot_offset).rem_euclid(len) as usize,
                        );
                        let (letter_i, letter_right_i) = (pos2letter[i], pos2letter[right_i]);
                        (pos2letter[i], pos2letter[right_i]) = (pos2letter[right_i], pos2letter[i]);
                        (letter2pos[letter_i], letter2pos[letter_right_i]) =
                            (letter2pos[letter_right_i], letter2pos[letter_i]);
                    }
                }
                Op::Mov(left, right) => match left.cmp(&right) {
                    Ordering::Less => {
                        let letter =
                            pos2letter[(left as isize - rot_offset).rem_euclid(len) as usize];
                        for pos in left..right {
                            let pos = (pos as isize - rot_offset).rem_euclid(len) as usize;
                            let next_pos = (pos + 1).rem_euclid(len as usize);
                            letter2pos[pos2letter[next_pos]] = pos;
                            pos2letter[pos] = pos2letter[next_pos];
                        }
                        let right = (right as isize - rot_offset).rem_euclid(len) as usize;
                        letter2pos[letter] = right;
                        pos2letter[right] = letter;
                    }
                    Ordering::Greater => {
                        let letter =
                            pos2letter[(left as isize - rot_offset).rem_euclid(len) as usize];
                        for pos in (right + 1..=left).rev() {
                            let pos = (pos as isize - rot_offset).rem_euclid(len) as usize;
                            let next_pos = (pos as isize - 1).rem_euclid(len) as usize;
                            letter2pos[pos2letter[next_pos]] = pos;
                            pos2letter[pos] = pos2letter[next_pos];
                        }
                        let right = (right as isize - rot_offset).rem_euclid(len) as usize;
                        letter2pos[letter] = right;
                        pos2letter[right] = letter;
                    }
                    Ordering::Equal => {}
                },
            }
        }
        (0..len as usize)
            .map(|i| {
                (pos2letter[(i as isize - rot_offset).rem_euclid(len) as usize] as u8 + b'a')
                    as char
            })
            .collect()
    }
}

pub mod part2 {
    use std::cmp::Ordering;

    use super::{parse, Op};

    pub fn solve(input: &str, password: &str) -> String {
        let operations = parse(input);
        let len = password.len();
        let mut pos2letter: Vec<_> = vec![0; len];
        let mut letter2pos: Vec<_> = vec![0; len];
        for (pos, letter) in password
            .chars()
            .map(|char| char as usize - 'a' as usize)
            .enumerate()
        {
            pos2letter[pos] = letter;
            letter2pos[letter] = pos;
        }
        let len = len as isize;
        let mut rot_offset = 0;
        let mut rot_letter_inv = vec![0; len as usize];
        for i in 0..len {
            let mut inv = 1 + i;
            if i >= 4 {
                inv += 1;
            }
            rot_letter_inv[(inv + i).rem_euclid(len) as usize] = (inv + i).rem_euclid(len) - i;
        }
        for operation in operations.into_iter().rev() {
            match operation {
                Op::SwapPos(left, right) => {
                    let (left, right) = (
                        (left as isize - rot_offset).rem_euclid(len) as usize,
                        (right as isize - rot_offset).rem_euclid(len) as usize,
                    );
                    (letter2pos[pos2letter[left]], letter2pos[pos2letter[right]]) =
                        (letter2pos[pos2letter[right]], letter2pos[pos2letter[left]]);
                    (pos2letter[left], pos2letter[right]) = (pos2letter[right], pos2letter[left]);
                }
                Op::SwapLetter(l_letter, r_letter) => {
                    (letter2pos[l_letter], letter2pos[r_letter]) =
                        (letter2pos[r_letter], letter2pos[l_letter]);
                    let (left, right) = (letter2pos[l_letter], letter2pos[r_letter]);
                    (pos2letter[left], pos2letter[right]) = (pos2letter[right], pos2letter[left]);
                }
                Op::RotSteps(steps) => {
                    rot_offset = (rot_offset - steps).rem_euclid(len);
                }
                Op::RotLetter(letter) => {
                    let index = (letter2pos[letter] as isize + rot_offset).rem_euclid(len) as usize;
                    rot_offset = (rot_offset - rot_letter_inv[index]).rem_euclid(len);
                }
                Op::Rev(left, right) => {
                    for i in left..=(left + right) / 2 {
                        let (i, right_i) = (
                            (i as isize - rot_offset).rem_euclid(len) as usize,
                            ((right - i + left) as isize - rot_offset).rem_euclid(len) as usize,
                        );
                        let (letter_i, letter_right_i) = (pos2letter[i], pos2letter[right_i]);
                        (pos2letter[i], pos2letter[right_i]) = (pos2letter[right_i], pos2letter[i]);
                        (letter2pos[letter_i], letter2pos[letter_right_i]) =
                            (letter2pos[letter_right_i], letter2pos[letter_i]);
                    }
                }
                Op::Mov(right, left) => match left.cmp(&right) {
                    Ordering::Less => {
                        let letter =
                            pos2letter[(left as isize - rot_offset).rem_euclid(len) as usize];
                        for pos in left..right {
                            let pos = (pos as isize - rot_offset).rem_euclid(len) as usize;
                            let next_pos = (pos + 1).rem_euclid(len as usize);
                            letter2pos[pos2letter[next_pos]] = pos;
                            pos2letter[pos] = pos2letter[next_pos];
                        }
                        let right = (right as isize - rot_offset).rem_euclid(len) as usize;
                        letter2pos[letter] = right;
                        pos2letter[right] = letter;
                    }
                    Ordering::Greater => {
                        let letter =
                            pos2letter[(left as isize - rot_offset).rem_euclid(len) as usize];
                        for pos in (right + 1..=left).rev() {
                            let pos = (pos as isize - rot_offset).rem_euclid(len) as usize;
                            let next_pos = (pos as isize - 1).rem_euclid(len) as usize;
                            letter2pos[pos2letter[next_pos]] = pos;
                            pos2letter[pos] = pos2letter[next_pos];
                        }
                        let right = (right as isize - rot_offset).rem_euclid(len) as usize;
                        letter2pos[letter] = right;
                        pos2letter[right] = letter;
                    }
                    Ordering::Equal => {}
                },
            }
        }
        (0..len as usize)
            .map(|i| {
                (pos2letter[(i as isize - rot_offset).rem_euclid(len) as usize] as u8 + b'a')
                    as char
            })
            .collect()
    }
}

pub fn main(test: bool) {
    let test_input = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"
        .to_owned();
    let (puzzle_input, password, scrambled) = if test {
        (test_input, "abcde", "decab")
    } else {
        (
            read_to_string("inputs/day_21_input.txt").unwrap(),
            "abcdefgh",
            "fbgdceah",
        )
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, password.len()));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, scrambled));
    println!("Run in {:?}", start.elapsed());
}
