from queue import Queue


def parse(data: str):
    points = list(map(lambda line: tuple(map(int, line.split(","))), data.splitlines()))
    max_x = max(points, key=lambda t: t[0])[0]
    max_y = max(points, key=lambda t: t[1])[1]
    max_z = max(points, key=lambda t: t[2])[2]
    return points, (max_x, max_y, max_z)


def get_matrix(max_x, max_y, max_z, points):
    matrix = [[[False for _ in range(max_z + 2)] for _ in range(max_y + 2)] for _ in range(max_x + 2)]
    for x, y, z in points:
        matrix[x][y][z] = True
    return matrix


def part1(data: str):
    points, (max_x, max_y, max_z) = parse(data)
    matrix = get_matrix(max_x, max_y, max_z, points)
    count = 0
    for x, y, z in points:
        if not matrix[x][y][z + 1]:
            count += 1
        if not matrix[x][y][z - 1]:
            count += 1
        if not matrix[x][y + 1][z]:
            count += 1
        if not matrix[x][y - 1][z]:
            count += 1
        if not matrix[x + 1][y][z]:
            count += 1
        if not matrix[x - 1][y][z]:
            count += 1
    return count


def bfs(matrix):
    queue = Queue()
    queue.put((0, 0, 0))
    visited = [[[False for _ in range(len(matrix[0][0]))] for _ in range(len(matrix[0]))] for _ in range(len(matrix))]
    while not queue.empty():
        x, y, z = queue.get()
        if visited[x][y][z]:
            continue
        visited[x][y][z] = True
        for dx, dy, dz in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]:
            new_x, new_y, new_z = x + dx, y + dy, z + dz
            if 0 <= new_x < len(matrix) and 0 <= new_y < len(matrix[0]) and 0 <= new_z < len(matrix[0][0]):
                if not matrix[new_x][new_y][new_z]:
                    queue.put((new_x, new_y, new_z))
    return visited


def part2(data: str):
    points, (max_x, max_y, max_z) = parse(data)
    matrix = get_matrix(max_x, max_y, max_z, points)
    visited = bfs(matrix)
    count = 0
    for x, y, z in points:
        if visited[x][y][z + 1]:
            count += 1
        if visited[x][y][z - 1]:
            count += 1
        if visited[x][y + 1][z]:
            count += 1
        if visited[x][y - 1][z]:
            count += 1
        if visited[x + 1][y][z]:
            count += 1
        if visited[x - 1][y][z]:
            count += 1
    return count


if __name__ == "__main__":
    test = True
    test = False
    test_input = """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_18_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
