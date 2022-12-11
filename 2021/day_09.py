from collections import deque
from typing import List, Tuple


def get_adjacent(floor: List[List[int]], i: int, j: int) -> List[int]:
    return [floor[i1][j1] for i1, j1 in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] if
            not (i1 == i and j1 == j) and 0 <= i1 < len(floor) and 0 <= j1 < len(floor[0])]


def get_low_points(floor: List[List[int]]) -> List[Tuple[int, int]]:
    return [(i, j) for i in range(len(floor)) for j in range(len(floor[i])) if
            all(floor[i][j] < x for x in get_adjacent(floor, i, j))]


def part1(data: str):
    floor = list(map(lambda x: list(map(int, list(x))), data.splitlines()))
    low_points = get_low_points(floor)
    return sum(map(lambda coord: floor[coord[0]][coord[1]], low_points), len(low_points))


def get_basin_size(floor: List[List[int]], i: int, j: int) -> int:
    visited = [[False for _ in range(len(floor[i]))] for _ in range(len(floor))]
    basin = deque()
    basin.append((i, j))
    c = 0
    while basin:
        i, j = basin.popleft()
        if visited[i][j]:
            continue
        c += 1
        visited[i][j] = True
        for i1, j1 in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if 0 <= i1 < len(floor) and 0 <= j1 < len(floor[i]) and floor[i][j] < floor[i1][j1] != 9:
                basin.append((i1, j1))
    return c


def part2(data: str):
    floor = list(map(lambda x: list(map(int, list(x))), data.splitlines()))
    low_points = get_low_points(floor)
    first_three = sorted(map(lambda l: get_basin_size(floor, *l), low_points))[-3:]
    return first_three[0] * first_three[1] * first_three[2]


if __name__ == "__main__":
    test = True
    test = False
    test_input = """2199943210
3987894921
9856789892
8767896789
9899965678"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_09_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
