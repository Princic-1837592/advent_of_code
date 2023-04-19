# https://adventofcode.com/2021/day/23
# https://adventofcode.com/2021/day/23/input
from typing import Tuple, List
from collections import deque


class State:
    def __init__(self, spaces: int, amphipods: int):
        self.matrix = [-1] * spaces
        self.amphipods = [-1] * amphipods


def parse(data: str) -> State:
    spaces = []
    matrix = [list(line) for line in data.splitlines()]
    for i, row in enumerate(matrix):
        for j, c in enumerate(row):
            if c in "ABCD" or c == "." and i + 1 < len(matrix) and matrix[i + 1][j] not in "ABCD":
                spaces.append((i, j))
    print(spaces)
    # return result


def part1(data: str):
    # 12348 too low
    # 12872 too high
    state = parse(data)
    print(state)
    # return solve(state)


def part2(data: str):
    pass


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_23_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
