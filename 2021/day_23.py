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
    graph = [
        [
            (0, []),
            (1, [1, ]),
            (3, [1, 2]),
            (5, [1, 2, 3]),
            (7, [1, 2, 3, 4]),
            (9, [1, 2, 3, 4, 5]),
            (10, [1, 2, 3, 4, 5, 6])
        ],
        [
            (1, [0, ]),
            (0, []),
            (2, [2, ]),
            (4, [2, 3]),
            (6, [2, 3, 4]),
            (8, [2, 3, 4, 5]),
            (9, [2, 3, 4, 5, 6])
        ],
        [
            (3, [1, 0]),
            (2, [1, ]),
            (0, []),
            (2, [3, ]),
            (4, [3, 4]),
            (6, [3, 4, 5]),
            (7, [3, 4, 5, 6])
        ],
        [
            (5, [2, 1, 0]),
            (4, [2, 1]),
            (2, [2, ]),
            (0, []),
            (2, [4, ]),
            (4, [4, 5]),
            (5, [4, 5, 6])
        ],
        [
            (7, [3, 2, 1, 0]),
            (6, [3, 2, 1]),
            (4, [3, 2]),
            (2, [3, ]),
            (0, []),
            (2, [5, ]),
            (3, [5, 6])
        ],
        [
            (9, [4, 3, 2, 1, 0]),
            (8, [4, 3, 2, 1]),
            (6, [4, 3, 2]),
            (4, [4, 3]),
            (2, [4, ]),
            (0, []),
            (1, [6, ])
        ],
        [
            (10, [5, 4, 3, 2, 1, 0]),
            (9, [5, 4, 3, 2, 1]),
            (7, [5, 4, 3, 2]),
            (5, [5, 4, 3]),
            (3, [5, 4]),
            (1, [5, ]),
            (0, [])
        ]
    ]
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
