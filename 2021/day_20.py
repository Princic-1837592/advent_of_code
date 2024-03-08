# https://adventofcode.com/2021/day/20
# https://adventofcode.com/2021/day/20/input
from typing import Tuple, List, Set


def parse(data: str) -> Tuple[List[bool], Set[Tuple[int, int]]]:
    algorithm, image = data.split("\n\n")
    algorithm = [char == "#" for char in algorithm]
    image = image.splitlines()
    result = set()
    for i, line in enumerate(image):
        for j, char in enumerate(line):
            if char == "#":
                result.add((i, j))
    return algorithm, result


NEIGHBOURS = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)]


def build_matrix(image: Set[Tuple[int, int]], extra_steps: int) -> List[List[bool]]:
    min_i, min_j = min(image)
    max_i, max_j = max(image)
    width = max_j - min_j + 1 + extra_steps * 2
    height = max_i - min_i + 1 + extra_steps * 2
    result = [[False for _ in range(width)] for _ in range(height)]
    for i, j in image:
        result[i - min_i + extra_steps][j - min_j + extra_steps] = True
    return result


def solve(data: str, steps: int):
    algorithm, image = parse(data)
    matrix = build_matrix(image, steps)
    for step in range(steps):
        new_matrix = [row.copy() for row in matrix]
        for i in range(len(matrix)):
            for j in range(len(matrix[i])):
                number = 0
                for di, dj in NEIGHBOURS:
                    number *= 2
                    ni, nj = i + di, j + dj
                    if 0 <= ni < len(matrix) and 0 <= nj < len(matrix[ni]):
                        if matrix[ni][nj]:
                            number += 1
                    elif step % 2 == 1:
                        number += 1
                if algorithm[number]:
                    new_matrix[i][j] = True
                else:
                    new_matrix[i][j] = False
        matrix = new_matrix
    result = 0
    for row in matrix:
        for cell in row:
            if cell:
                result += 1
    return result


def part1(data: str):
    return solve(data, 2)


def part2(data: str):
    return solve(data, 50)


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2021/day_20_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
