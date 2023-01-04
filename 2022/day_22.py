from re import findall
from typing import Optional

FACING = {
    (0, 1): 0,
    (1, 0): 1,
    (0, -1): 2,
    (-1, 0): 3
}


def step(i, j, di, dj, board):
    i = (i + di) % len(board)
    j = (j + dj) % len(board[i])
    return i, j


def part1(data: str):
    board, moves = data.split("\n\n")
    board = board.splitlines()
    max_width = max(map(len, board))
    board = list(map(lambda l: l.ljust(max_width), board))
    numbers, letters = findall(r"\d+", moves), findall(r"[RL]", moves)
    moves = [Optional[str | int]] * (len(numbers) + len(letters))
    moves[::2], moves[1::2] = map(int, numbers), letters
    i, j = 0, 0
    while board[i][j] == " ":
        j += 1
    di, dj = 0, 1
    for move in moves:
        if move == "L":
            di, dj = -dj, di
        elif move == "R":
            di, dj = dj, -di
        else:
            for _ in range(move):
                ni, nj = step(i, j, di, dj, board)
                if board[ni][nj] == "#":
                    break
                while board[ni][nj] == " ":
                    ni, nj = step(ni, nj, di, dj, board)
                if board[ni][nj] == "#":
                    break
                i, j = ni, nj
    return 1000 * (i + 1) + 4 * (j + 1) + FACING[di, dj]


"""
 AB
 C
DE
F
"""
FACES = dict(
    zip(
        ["A", "B",
         "C",
         "D", "E",
         "F"],
        [
            (0, 1), (0, 2),
            (1, 1),
            (2, 0), (2, 1),
            (3, 0)
        ]
    )
)


def step_cube(i, j, di, dj, board):
    size = len(board[0]) // 3
    ni = i + di
    nj = j + dj
    # if coordinates are on a face
    for li, lj in FACES.values():
        if li * size <= ni < li * size + size and lj * size <= nj < lj * size + size:
            return ni, nj, di, dj
    if ni == -1 and (di, dj) == (-1, 0):
        # from A to F
        if FACES["A"][1] * size <= nj < FACES["A"][1] * size + size:
            return 3 * size + nj % size, 0, 0, 1
        # from B to F
        if FACES["B"][1] * size <= nj < FACES["B"][1] * size + size:
            return 4 * size - 1, nj % size, -1, 0
    if ni == 4 * size and (di, dj) == (1, 0):
        # from F to B
        return 0, nj + 2 * size, 1, 0
    if nj == -1 and (di, dj) == (0, -1):
        # from D to A
        if FACES["D"][0] * size <= ni < FACES["D"][0] * size + size:
            return size - ni % size - 1, size, 0, 1
        # from F to A
        if FACES["F"][0] <= ni < FACES["F"][0] * size + size:
            return 0, size + ni % size, 1, 0
    if nj == 3 * size and (di, dj) == (0, 1):
        # from B to E
        return 3 * size - ni % size - 1, 2 * size - 1, 0, -1
    if ni == 2 * size - 1 and (di, dj) == (-1, 0):
        # from D to C
        return size + nj % size, size, 0, 1
    if ni == 3 * size and (di, dj) == (1, 0):
        # from E to F
        return 3 * size + nj % size, size - 1, 0, -1
    if nj == 2 * size and (di, dj) == (0, 1):
        # from C to B
        if FACES["C"][0] * size <= ni < FACES["C"][0] * size + size:
            return size - 1, 2 * size + ni % size, -1, 0
        # from E to B
        if FACES["E"][0] * size <= ni < FACES["E"][0] * size + size:
            return size - ni % size - 1, 3 * size - 1, 0, -1
    if ni == size and (di, dj) == (1, 0):
        # from B to C
        return size + nj % size, 2 * size - 1, 0, -1
    if nj == size - 1 and (di, dj) == (0, -1):
        # from A to D
        if FACES["A"][0] <= ni < FACES["A"][0] * size + size:
            return 3 * size - ni % size - 1, 0, 0, 1
        # from C to D
        if FACES["C"][0] <= ni < FACES["C"][0] * size + size:
            return 2 * size, ni % size, 1, 0
    if nj == size and (di, dj) == (0, 1):
        # from F to E
        return 3 * size - 1, size + ni % size, -1, 0
    print(i, j, di, dj, ni, nj)
    raise ValueError(f"Can't step from {i, j} to {ni, nj}")


DIRECTIONS = {
    (0, 1): ">",
    (0, -1): "<",
    (1, 0): "v",
    (-1, 0): "^"
}


def part2(data: str):
    board, moves = data.split("\n\n")
    board = board.splitlines()
    max_width = max(map(len, board))
    board = list(map(lambda l: l.ljust(max_width), board))
    numbers, letters = findall(r"\d+", moves), findall(r"[RL]", moves)
    moves = [Optional[str | int]] * (len(numbers) + len(letters))
    moves[::2], moves[1::2] = map(int, numbers), letters
    i, j = 0, 0
    while board[i][j] == " ":
        j += 1
    di, dj = 0, 1
    to_print = [list(row) for row in board]
    for m, move in enumerate(moves):
        if move == "L":
            di, dj = -dj, di
        elif move == "R":
            di, dj = dj, -di
        else:
            for _ in range(move):
                ni, nj, ndi, ndj = step_cube(i, j, di, dj, board)
                if board[ni][nj] == "#":
                    break
                i, j, di, dj = ni, nj, ndi, ndj
                to_print[i][j] = DIRECTIONS[di, dj]
    return 1000 * (i + 1) + 4 * (j + 1) + FACING[di, dj]


if __name__ == "__main__":
    test = True
    # test = False
    test_input = """        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#. 

10R5L5R10L4R5L"""
    test_input = """    ........
    ........
    ........
    ........
    ....
    ....
    ....
    ....
........
........
........
........
....
....
....
....

13L12"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_22_input.txt", "r") as input_file:
            puzzle_input = input_file.read()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
