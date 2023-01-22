from typing import List


def dynamic_programming(data: List[List[int]]):
    grid = [[0 for _ in range(len(data[0]))] for _ in range(len(data))]
    for k in range(1, len(data[0])):
        grid[0][k] = data[0][k] + grid[0][k - 1]
        grid[k][0] = data[k][0] + grid[k - 1][0]
    for i in range(1, len(data)):
        for j in range(1, len(data[0])):
            grid[i][j] = data[i][j] + min(grid[i - 1][j], grid[i][j - 1])
    # print("\n".join(map(lambda l: "".join(map(lambda x: f"{x:>5}", l)), grid)))
    return grid[-1][-1]


def dijkstra(cavern: List[List[int]]) -> int:
    # print("\n".join(map(lambda l: "".join(map(str, l)), cavern)))
    dynamic_result = dynamic_programming(cavern)
    print("dynamic result:", dynamic_result)
    weights = [[{} for _ in range(len(cavern[0]))] for _ in range(len(cavern))]
    for i in range(len(cavern)):
        for j in range(len(cavern[0])):
            if i < len(cavern) - 1:
                weights[i][j][(i + 1, j)] = cavern[i + 1][j]
            if j < len(cavern[0]) - 1:
                weights[i][j][(i, j + 1)] = cavern[i][j + 1]
            if i > 0:
                weights[i][j][(i - 1, j)] = cavern[i - 1][j]
            if j > 0:
                weights[i][j][(i, j - 1)] = cavern[i][j - 1]
    visited = [[False for _ in range(len(cavern[0]))] for _ in range(len(cavern))]
    distances = [[dynamic_result for _ in range(len(cavern[0]))] for _ in range(len(cavern))]
    distances[0][0] = 0
    reachable = {(0, 0)}
    round = 0
    while True:
        if round % 10000 == 0:
            print(round, len(reachable))
        mi, mj = None, None
        for node in reachable:
            i, j = node
            if not visited[i][j]:
                if mi is None:
                    mi, mj = node
                elif distances[i][j] < distances[mi][mj]:
                    mi, mj = node
        for edge, weight in weights[mi][mj].items():
            i, j = edge
            distances[i][j] = min(distances[i][j], distances[mi][mj] + weight)
        visited[mi][mj] = True
        reachable.remove((mi, mj))
        if mi < len(cavern) - 1 and not visited[mi + 1][mj]:
            reachable.add((mi + 1, mj))
        if mj < len(cavern[0]) - 1 and not visited[mi][mj + 1]:
            reachable.add((mi, mj + 1))
        if mi > 0 and not visited[mi - 1][mj]:
            reachable.add((mi - 1, mj))
        if mj > 0 and not visited[mi][mj - 1]:
            reachable.add((mi, mj - 1))
        if (mi, mj) == (len(cavern) - 1, len(cavern[0]) - 1):
            break
        round += 1
    return distances[len(cavern) - 1][len(cavern[0]) - 1]


def part1(data: str):
    cavern = list(map(lambda l: list(map(int, l)), data.splitlines()))
    return dijkstra(cavern)


def multiply(cavern):
    bigger_cavern = [[0 for _ in range(len(cavern[0]) * 5)] for _ in range(len(cavern) * 5)]
    for bi in range(5):
        coeff = bi
        for bj in range(5):
            for k in range(len(cavern)):
                for l in range(len(cavern[0])):
                    bigger_cavern[bi * len(cavern) + k][bj * len(cavern[0]) + l] = (cavern[k][l] + coeff - 1) % 9 + 1
            coeff += 1
    return bigger_cavern


def part2(data: str):
    cavern = list(map(lambda l: list(map(int, l)), data.splitlines()))
    return dijkstra(multiply(cavern))


if __name__ == "__main__":
    import time

    test = True
    test = False
    test_input = """1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_15_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    start = time.time()
    print(part1(puzzle_input))
    print(time.time() - start)
    start = time.time()
    print(part2(puzzle_input))
    print(time.time() - start)
