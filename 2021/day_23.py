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
    adjacency = [
        [(1, 1)],
        [(0, 1), (2, 2)],
        [(1, 2), (3, 2)],
        [(2, 2), (4, 2)],
        [(3, 2), (5, 2)],
        [(4, 2), (6, 1)],
        [(5, 1)],
    ]
    for room in range(4):
        room_index = room * depth + 7
        adjacency.append([(room + 1, 2), (room + 2, 2)])
        adjacency[room + 1].append((room_index, 2))
        adjacency[room + 2].append((room_index, 2))
        for row in range(depth):
            position_index = room_index + row
            input_i = row + 2
            input_j = room * 2 + 3
            amphipod_type = ord(lines[input_i][input_j]) - ord('A')
            state.positions[position_index] = amphipod_type
            amphipod_index = amphipod_type * depth
            while state.amphipods[amphipod_index] != -1:
                amphipod_index += 1
            state.amphipods[amphipod_index] = position_index
            if row > 0:
                adjacency.append([(position_index - 1, 1)])
                adjacency[position_index - 1].append((position_index, 1))
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
