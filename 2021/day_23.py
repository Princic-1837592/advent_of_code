# https://adventofcode.com/2021/day/23
# https://adventofcode.com/2021/day/23/input
from typing import Tuple, List
from collections import deque


class State:
    def __init__(self, positions: int, amphipods: int):
        self.positions = [-1] * positions
        self.amphipods = [-1] * amphipods

    def __str__(self):
        return f"State({self.positions}, {self.amphipods})"


NEIGHBORS = ((0, 1), (0, -1), (1, 0), (-1, 0))


def parse(data: str) -> State:
    lines = data.splitlines()
    depth = len(lines) - 3
    positions = 7 + depth * 4
    state = State(positions, depth * 4)
    for room in range(4):
        for row in range(depth):
            position_index = room * depth + row + 7
            input_i = row + 2
            input_j = room * 2 + 3
            amphipod_type = ord(lines[input_i][input_j]) - ord('A')
            state.positions[position_index] = amphipod_type
    return state


def part1(data: str):
    # 12348 too low
    # 12872 too high
    state = parse(data)
    print(state)
    # return solve(state)


def part2(data: str):
    pass


if __name__ == "__main__":
    test = True
    # test = False
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
