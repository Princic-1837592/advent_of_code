from typing import List, Tuple


def split_line(line: str) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    start, end = line.split(" -> ")
    start = tuple(map(int, start.split(",")))
    end = tuple(map(int, end.split(",")))
    return start, end


def part1(data: str):
    lines: List[Tuple[Tuple[int, int], Tuple[int, int]]] = list(map(split_line, data.splitlines()))
    width = max(max(line[0][1], line[1][1]) for line in lines)
    height = max(max(line[0][0], line[1][0]) for line in lines)
    grid = [[0 for _ in range(width + 1)] for _ in range(height + 1)]
    for line in lines:
        x1, y1 = line[0]
        x2, y2 = line[1]
        if x1 == x2:
            for y in range(min(y1, y2), max(y1, y2) + 1):
                grid[x1][y] += 1
        elif y1 == y2:
            for x in range(min(x1, x2), max(x1, x2) + 1):
                grid[x][y1] += 1
    c = 0
    for row in grid:
        for col in row:
            if col > 1:
                c += 1
    return c


def part2(data: str):
    lines: List[Tuple[Tuple[int, int], Tuple[int, int]]] = list(map(split_line, data.splitlines()))
    width = max(max(line[0][1], line[1][1]) for line in lines)
    height = max(max(line[0][0], line[1][0]) for line in lines)
    grid = [[0 for _ in range(width + 1)] for _ in range(height + 1)]
    for line in lines:
        x1, y1 = line[0]
        x2, y2 = line[1]
        if x1 == x2:
            for y in range(min(y1, y2), max(y1, y2) + 1):
                grid[x1][y] += 1
        elif y1 == y2:
            for x in range(min(x1, x2), max(x1, x2) + 1):
                grid[x][y1] += 1
        else:
            diff = abs(x1 - x2)
            for i in range(diff + 1):
                x = x1 + i if x1 < x2 else x1 - i
                y = y1 + i if y1 < y2 else y1 - i
                grid[x][y] += 1
    c = 0
    for row in grid:
        for col in row:
            if col > 1:
                c += 1
    return c


if __name__ == "__main__":
    test = True
    test = False
    test_input = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2021/day_05_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
