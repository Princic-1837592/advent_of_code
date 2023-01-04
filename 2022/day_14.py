from typing import List


def get_paths(data: str):
    return list(
        map(
            lambda path: list(map(lambda pair: tuple(map(int, reversed(pair.split(",")))), path.split(" -> "))),
            data.splitlines()
        )
    )


def get_min_max(paths, min_max, coord: int) -> int:
    return min_max(map(lambda path: min_max(map(lambda point: point[coord], path)), paths))


def make_cave(paths, h, w_min, w_max):
    w = w_max - w_min + 1
    cave = [[0 for _ in range(w)] for _ in range(h)]
    for path in paths:
        last = path[0]
        for point in path:
            if last[0] == point[0]:
                for j in range(min(last[1], point[1]), max(last[1], point[1]) + 1):
                    cave[last[0]][j - w_min] = 1
            else:
                for i in range(min(last[0], point[0]), max(last[0], point[0]) + 1):
                    cave[i][last[1] - w_min] = 1
            last = point
    return cave


def get_dims(paths):
    h = get_min_max(paths, max, 0) + 1
    w_max = get_min_max(paths, max, 1)
    w_min = get_min_max(paths, min, 1)
    return h, w_min, w_max


def drop_one(cave: List[List[int]], j: int):
    i = 0
    moved = True
    while moved:
        moved = False
        if i + 1 < len(cave) and cave[i + 1][j] == 0:
            i += 1
            moved = True
        elif j - 1 >= 0 and i + 1 < len(cave) and cave[i + 1][j - 1] == 0:
            j -= 1
            i += 1
            moved = True
        elif j + 1 < len(cave[0]) and i + 1 < len(cave) and cave[i + 1][j + 1] == 0:
            j += 1
            i += 1
            moved = True
    return i, j


def to_str(cave: List[List[int]]) -> str:
    symbols = [" ", "#", "o"]
    cave_str = [[symbols[x] for x in row] for row in cave]
    return "\n".join(map(lambda row: "".join(row), cave_str))


def part1(data: str):
    paths = get_paths(data)
    h, w_min, w_max = get_dims(paths)
    cave = make_cave(paths, h, w_min, w_max)
    middle = 500 - w_min
    units = 0
    while cave[0][middle] == 0:
        i, j = drop_one(cave, middle)
        if i == len(cave) - 1:
            break
        cave[i][j] = 2
        units += 1
    return units


def part2(data: str):
    paths = get_paths(data)
    h, w_min, w_max = get_dims(paths)
    w_min = 0
    w_max *= 2
    h += 2
    paths.append([(h - 1, 0), (h - 1, w_max)])
    cave = make_cave(paths, h, w_min, w_max)
    middle = 500
    units = 0
    while cave[0][middle] == 0:
        i, j = drop_one(cave, middle)
        cave[i][j] = 2
        units += 1
    return units


if __name__ == "__main__":
    test = True
    test = False
    test_input = """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_14_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
