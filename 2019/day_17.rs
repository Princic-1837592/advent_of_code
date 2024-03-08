//! https://adventofcode.com/2019/day/17
//! https://adventofcode.com/2019/day/17/input

use std::{fs::read_to_string, time::Instant};

use crate::int_code::IntCode;

type Coord = (usize, usize);
type Direction = (isize, isize);
const NEIGHBOURS: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn make_image(mut vm: IntCode) -> (Vec<Vec<char>>, Coord, Direction) {
    vm.run_until_complete();
    let mut image = vec![vec![]];
    let mut position = (0, 0);
    let mut direction = (0, 0);
    for &value in vm.get_output() {
        match value as u8 as char {
            '\n' => image.push(vec![]),
            char => {
                if char == '<' || char == '>' || char == 'v' || char == '^' {
                    position = (image.len() - 1, image.last().unwrap().len());
                    direction = match char {
                        '<' => (0, -1),
                        '>' => (0, 1),
                        '^' => (-1, 0),
                        _v => (1, 0),
                    };
                }
                image.last_mut().unwrap().push(char)
            }
        }
    }
    while image.last().unwrap().is_empty() {
        image.pop();
    }
    (image, position, direction)
}

pub mod part1 {
    use crate::{
        day_17::{make_image, NEIGHBOURS},
        int_code::parse,
    };

    pub fn solve(input: &str) -> usize {
        let vm = parse(input);
        let (image, _, _) = make_image(vm);
        let mut result = 0;
        for (i, row) in image.iter().enumerate().skip(1).take(image.len() - 2) {
            for (j, &pixel) in row.iter().enumerate().skip(1).take(image[i].len() - 2) {
                if NEIGHBOURS
                    .iter()
                    .filter(|(di, dj)| {
                        let (ni, nj) = ((i as isize + di) as usize, (j as isize + dj) as usize);
                        image[ni][nj] == '#'
                    })
                    .count()
                    == 4
                    && pixel == '#'
                {
                    result += i * j;
                }
            }
        }
        result
    }
}

pub mod part2 {
    use crate::int_code::parse_with_input;

    pub fn solve(input: &str) -> usize {
        let inputs: Vec<_> = [
            "A,A,B,C,C,A,C,B,C,B\n",
            "L,4,L,4,L,6,R,10,L,6\n",
            "L,12,L,6,R,10,L,6\n",
            "R,8,R,10,L,6\n",
            "n\n",
        ]
        .into_iter()
        .flat_map(|line| line.chars().map(|char| char as i64))
        .collect();
        let mut robot = parse_with_input(
            &"2".chars().chain(input.chars().skip(1)).collect::<String>(),
            inputs.into(),
        );
        robot.run_until_complete();
        robot.last_output().unwrap() as usize
    }

    #[allow(unused)]
    mod by_hand {
        use itertools::Itertools;

        use crate::{
            day_17::{make_image, Coord, Direction},
            int_code::parse,
        };

        pub(crate) fn solve(input: &str) {
            let vm = parse(input);
            let (image, (mut i, mut j), (mut di, mut dj)) = make_image(vm);
            let mut moves = vec![];
            let mut steps = 0;
            while let Some(((ni, nj), (ndi, ndj), rotations)) = rotate(&image, i, j, di, dj) {
                (i, j) = (ni, nj);
                (di, dj) = (ndi, ndj);
                if rotations == 0 {
                    steps += 1;
                } else {
                    if steps > 0 {
                        moves.push(steps + 1);
                        // moves.push(if rotations == 1 {
                        //     'L' as isize
                        // } else {
                        //     'R' as isize
                        // });
                    }
                    steps = 0;
                }
            }
            if steps > 0 {
                moves.push(steps + 1);
            }
            println!("{}", moves.into_iter().map(|n| n.to_string()).join("\n"));
        }

        fn rotate(
            image: &Vec<Vec<char>>,
            i: usize,
            j: usize,
            mut di: isize,
            mut dj: isize,
        ) -> Option<(Coord, Direction, usize)> {
            for rotations in 0..=3 {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if rotations != 2 && is_floor(image, ni, nj) {
                    return Some(((ni as usize, nj as usize), (di, dj), rotations));
                }
                (di, dj) = (-dj, di);
            }
            None
        }

        fn is_floor(image: &Vec<Vec<char>>, ni: isize, nj: isize) -> bool {
            (0..image.len() as isize).contains(&ni)
                && (0..image[ni as usize].len() as isize).contains(&nj)
                && image[ni as usize][nj as usize] == '#'
        }
    }
}

/*
..........####^..............................
..........#..................................
..........#..................................
..........#..................................
..........#######............................
................#............................
................#.......#############........
................#.......#...........#........
#######.........#.......#...........#........
#.....#.........#.......#...........#........
#.....#.........#.#####.#.....#########......
#.....#.........#.#...#.#.....#.....#.#......
#.....#.......###########.###########.#......
#.....#.......#.#.#...#...#...#.......#......
#.....#.......#.#######...#...#.......#......
#.....#.......#...#.......#...#.......#......
#.....#.###########.###########.......#......
#.....#.#.....#.....#.....#...........#......
#.....#########.....#.....#...........#......
#.......#...........#.....#...........#......
#######.#...........#.....#######.....#######
......#.#...........#...........#...........#
......#.#############...........#...........#
......#.........................#...........#
......#.........................#...........#
......#.........................#...........#
......#.........................#...........#
......#.........................#...........#
......#.........................#.###########
......#.........................#.#..........
......#######...................#######......
..................................#...#......
..................................#...#......
..................................#...#......
..................................#####......
*/

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_17_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
