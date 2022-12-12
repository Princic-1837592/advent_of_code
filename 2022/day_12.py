from typing import List, Tuple


def find_start_end(data: str) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    start = (0, 0)
    end = (0, 0)
    for i, row in enumerate(data.splitlines()):
        for j, cell in enumerate(row):
            if cell == "S":
                start = (i, j)
            elif cell == "E":
                end = (i, j)
    return start, end


def bfs(grid: List[List[int]], start: Tuple[int, int], end: Tuple[int, int]) -> int:
    front = {start}
    visited = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
    steps = 0
    max_steps = len(grid) * len(grid[0])
    while steps < max_steps and end not in front:
        steps += 1
        next_front = set()
        for node in front:
            i, j = node
            for di, dj in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                if i + di < 0 or j + dj < 0 or i + di >= len(grid) or j + dj >= len(grid[0]):
                    continue
                if visited[i + di][j + dj] != 0:
                    continue
                if grid[i + di][j + dj] <= grid[i][j] + 1:
                    next_front.add((i + di, j + dj))
                    visited[i + di][j + dj] = steps
        front = next_front
    return steps


def part1(data: str):
    grid_str = data.splitlines()
    grid = [[ord(x) - ord("a") for x in row] for row in grid_str]
    start, end = find_start_end(data)
    grid[start[0]][start[1]] = ord("a") - ord("a")
    grid[end[0]][end[1]] = ord("z") - ord("a")
    return bfs(grid, start, end)


def bfs_reversed(grid: List[List[int]], start: Tuple[int, int]) -> int:
    front = {start}
    visited = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
    steps = 0
    max_steps = len(grid) * len(grid[0])
    while steps < max_steps:
        steps += 1
        next_front = set()
        for node in front:
            i, j = node
            for di, dj in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                if i + di < 0 or j + dj < 0 or i + di >= len(grid) or j + dj >= len(grid[0]):
                    continue
                if visited[i + di][j + dj] != 0:
                    continue
                if grid[i + di][j + dj] >= grid[i][j] - 1:
                    if grid[i + di][j + dj] == 0:
                        return steps
                    next_front.add((i + di, j + dj))
                    visited[i + di][j + dj] = steps
        front = next_front
    return steps


def part2(data: str):
    grid_str = data.splitlines()
    grid = [[ord(x) - ord("a") for x in row] for row in grid_str]
    start, end = find_start_end(data)
    grid[start[0]][start[1]] = ord("a") - ord("a")
    grid[end[0]][end[1]] = ord("z") - ord("a")
    return bfs_reversed(grid, end)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_12_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
