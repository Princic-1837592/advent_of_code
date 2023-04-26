# https://adventofcode.com/2021/day/23
# https://adventofcode.com/2021/day/23/input
from typing import Tuple, List
from collections import deque


class State:
    def __init__(self, positions: int, amphipods: int):
        self.positions = [-1] * positions
        self.amphipods = [-1] * amphipods


NEIGHBORS = ((0, 1), (0, -1), (1, 0), (-1, 0))


def parse(data: str) -> State:
    spaces = []
    matrix = [list(line) for line in data.splitlines()]
    for i, row in enumerate(matrix):
        for j, c in enumerate(row):
            if c in "ABCD" or c == "." and i + 1 < len(matrix) and matrix[i + 1][j] not in "ABCD":
                spaces.append((i, j))
    print(spaces)
    distances = [[0] * len(spaces) for _ in range(len(spaces))]
    print(distances)
    for src in range(len(spaces)):
        queue = deque([(spaces[src], 0)])
        visited = {src}
        while queue:
            pos, dist = queue.popleft()
            if matrix[pos[0]][pos[1]] not in ".ABCD":
                continue
            if pos in visited:
                continue
            if pos in spaces:
                distances[src][spaces.index(pos)] = dist
            visited.add(pos)
            i, j = pos
            for di, dj in NEIGHBORS:
                queue.append(((i + di, j + dj), dist + 1))
    from pprint import pprint
    pprint(distances)
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
