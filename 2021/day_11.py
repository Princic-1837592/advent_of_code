from collections import deque


def step(grid):
    flashed = [[False for _ in range(10)] for _ in range(10)]
    flashes = 0
    for i in range(10):
        for j in range(10):
            grid[i][j] += 1

    queue = deque()
    for i in range(10):
        for j in range(10):
            if grid[i][j] > 9:
                queue.append((i, j))
    while queue:
        i, j = queue.popleft()
        if flashed[i][j]:
            continue
        flashed[i][j] = True
        flashes += 1
        grid[i][j] = 0
        for di in range(-1, 2):
            for dj in range(-1, 2):
                i1, j1 = i + di, j + dj
                if di == 0 and dj == 0 or not (0 <= i1 < 10) or not (0 <= j1 < 10):
                    continue
                grid[i1][j1] += 1
                if grid[i1][j1] > 9:
                    queue.append((i1, j1))

    for i in range(10):
        for j in range(10):
            if flashed[i][j]:
                grid[i][j] = 0
    return flashes


def part1(data: str):
    grid = list(map(lambda l: list(map(int, l)), data.splitlines()))
    flashes = 0
    for _ in range(100):
        flashes += step(grid)
    return flashes


def part2(data: str):
    grid = list(map(lambda l: list(map(int, l)), data.splitlines()))
    steps = 0
    while True:
        steps += 1
        if step(grid) == 100:
            break
    return steps


if __name__ == "__main__":
    test = True
    test = False
    test_input = """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_11_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
