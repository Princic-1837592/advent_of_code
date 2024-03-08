use std::{fs::read_to_string, time::Instant};

use crate::LINE_ENDING;

#[derive(Debug, Copy, Clone, Default)]
enum Rotation {
    #[default]
    None,
    Left,
    Double,
    Right,
}

#[derive(Debug, Copy, Clone, Default)]
struct Transformation {
    rotation: Rotation,
    flipped: bool,
}

const TRANSFORMATIONS: [Transformation; 8] = [
    Transformation {
        rotation: Rotation::None,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Left,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Double,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Right,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::None,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Left,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Double,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Right,
        flipped: true,
    },
];

const MONSTER: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Debug, Copy, Clone, Default)]
struct Tile {
    id: usize,
    lines: [u16; 10],
    edges: [u16; 8],
    transformation: Transformation,
}

impl Tile {
    fn get_edge(&self, index: usize) -> u16 {
        let rotation_index = self.transformation.rotation as usize;
        let flip_index = if self.transformation.flipped { 4 } else { 0 };
        self.edges[flip_index + (rotation_index + index) % 4]
    }

    fn top(&self) -> u16 {
        self.get_edge(0)
    }

    fn right(&self) -> u16 {
        self.get_edge(1)
    }

    fn bottom(&self) -> u16 {
        self.get_edge(2)
    }

    fn left(&self) -> u16 {
        self.get_edge(3)
    }

    fn can_go_right(&self, left: &Self) -> bool {
        self.left() == reverse_edge(left.right())
    }

    fn can_go_below(&self, top: &Self) -> bool {
        self.top() == reverse_edge(top.bottom())
    }

    fn to_matrix(self) -> Vec<Vec<bool>> {
        let mut result = vec![vec![false; 8]; 8];
        for (l, line) in self.lines.iter().skip(1).take(8).enumerate() {
            let mut mask = 0b100000000;
            for m in 0..8 {
                result[l][m] = (mask & line) != 0;
                mask >>= 1;
            }
        }
        result
    }
}

fn reverse_edge(edge: u16) -> u16 {
    let mut result = 0;
    let mut mask = 1;
    for _ in 0..10 {
        result <<= 1;
        result += if edge & mask != 0 { 1 } else { 0 };
        mask <<= 1;
    }
    result
}

impl From<&str> for Tile {
    fn from(string: &str) -> Self {
        fn side(lines: &[u16], right: bool) -> u16 {
            let mask = if right { 1 } else { 0b1000000000 };
            let mut result = 0;
            for line in lines {
                result <<= 1;
                result += if line & mask != 0 { 1 } else { 0 };
            }
            if right {
                result
            } else {
                reverse_edge(result)
            }
        }
        let mut str_lines = string.lines();
        let id: usize = str_lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();
        let mut lines = [0; 10];
        str_lines.enumerate().for_each(|(i, line)| {
            let mut result = 0;
            line.chars().for_each(|char| {
                result <<= 1;
                result |= match char {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid char: {}", char),
                };
            });
            lines[i] = result;
        });
        let mut edges = [0; 8];
        edges[0] = lines[0];
        edges[1] = side(&lines, true);
        edges[2] = reverse_edge(lines[9]);
        edges[3] = side(&lines, false);
        edges[4] = reverse_edge(edges[0]);
        edges[5] = reverse_edge(edges[3]);
        edges[6] = reverse_edge(edges[2]);
        edges[7] = reverse_edge(edges[1]);
        Tile {
            id,
            lines,
            edges,
            transformation: Transformation::default(),
        }
    }
}

fn parse(input: &str) -> Vec<Tile> {
    let separator = LINE_ENDING.repeat(2);
    input.split(&separator).map(Tile::from).collect()
}

fn solve_puzzle(tiles: &mut [Tile]) -> Vec<Vec<usize>> {
    fn recursive(
        tiles: &mut [Tile],
        used: &mut [bool],
        puzzle: &mut [Vec<usize>],
        mut row: usize,
        mut column: usize,
    ) -> bool {
        if column >= puzzle[0].len() {
            column = 0;
            row += 1;
        }
        if row >= puzzle.len() {
            return true;
        }
        for i in 0..tiles.len() {
            if !used[i] {
                used[i] = true;
                puzzle[row][column] = i;
                for transformation in TRANSFORMATIONS.iter() {
                    tiles[i].transformation = *transformation;
                    if (row == 0 || tiles[i].can_go_below(&tiles[puzzle[row - 1][column]]))
                        && (column == 0 || tiles[i].can_go_right(&tiles[puzzle[row][column - 1]]))
                        && recursive(tiles, used, puzzle, row, column + 1)
                    {
                        return true;
                    }
                }
                tiles[i].transformation = Transformation::default();
                used[i] = false;
            }
        }
        false
    }
    let dim = (tiles.len() as f32).sqrt() as usize;
    let mut puzzle = vec![vec![0; dim]; dim];
    recursive(tiles, &mut vec![false; dim * dim], &mut puzzle, 0, 0);
    puzzle
}

pub mod part1 {
    use super::{parse, solve_puzzle};

    pub fn solve(input: &str) -> usize {
        let mut tiles = parse(input);
        let puzzle = solve_puzzle(&mut tiles);
        tiles[puzzle[0][0]].id
            * tiles[puzzle[0][puzzle[0].len() - 1]].id
            * tiles[puzzle[puzzle.len() - 1][0]].id
            * tiles[puzzle[puzzle.len() - 1][puzzle[0].len() - 1]].id
    }
}

pub mod part2 {
    use super::{parse, solve_puzzle, Tile, Transformation, MONSTER, TRANSFORMATIONS};

    fn get_transformed_coord<T>(
        matrix: &[Vec<T>],
        transformation: Transformation,
        mut i: usize,
        mut j: usize,
    ) -> T
    where
        T: Copy,
    {
        for _ in 0..transformation.rotation as u8 {
            (i, j) = (j, matrix.len() - i - 1);
        }
        if transformation.flipped {
            j = matrix.len() - j - 1;
        }
        matrix[i][j]
    }

    fn create_image(tiles: &[Tile], puzzle: Vec<Vec<usize>>) -> Vec<Vec<char>> {
        let mut image = vec![vec![' '; puzzle[0].len() * 8]; puzzle.len() * 8];
        for (i, row) in puzzle.iter().enumerate() {
            for (j, tile_id) in row.iter().enumerate() {
                let tile = tiles[*tile_id];
                let matrix = tile.to_matrix();
                for r in 0..8 {
                    for c in 0..8 {
                        image[i * 8 + r][j * 8 + c] =
                            if get_transformed_coord(&matrix, tile.transformation, r, c) {
                                '#'
                            } else {
                                '.'
                            }
                    }
                }
            }
        }
        image
    }

    fn find_monster(
        image: &[Vec<char>],
        transformation: Transformation,
        is_monster: &mut [Vec<bool>],
        i: usize,
        j: usize,
    ) -> usize {
        for (di, dj) in MONSTER {
            if get_transformed_coord(image, transformation, i + di, j + dj) != '#' {
                return 0;
            }
        }
        let mut new = 0;
        for (di, dj) in MONSTER {
            if !is_monster[i + di][j + dj] {
                is_monster[i + di][j + dj] = true;
                new += 1;
            }
        }
        new
    }

    pub fn solve(input: &str) -> usize {
        let mut tiles = parse(input);
        let puzzle = solve_puzzle(&mut tiles);
        let image = create_image(&tiles, puzzle);
        let start: usize = image
            .iter()
            .map(|line| line.iter().filter(|char| **char == '#').count())
            .sum();
        let mut counter;
        for transformation in TRANSFORMATIONS {
            counter = start;
            let mut is_monster = vec![vec![false; image[0].len()]; image.len()];
            for i in 0..image.len() - 2 {
                for j in 0..image[0].len() - 19 {
                    counter -= find_monster(&image, transformation, &mut is_monster, i, j);
                }
            }
            if counter != start {
                return counter;
            }
        }
        start
    }
}

pub fn main(test: bool) {
    let test_input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
        .to_owned()
        .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_20_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
