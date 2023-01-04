from collections import deque
from typing import List, Tuple, Deque, Optional


class Cell:
    def __init__(self, current):
        self.current: Deque[str] = deque(current)
        self.future: Deque[str] = deque()

    def __str__(self):
        return "".join(self.current) if len(self.current) == 1 else str(len(self.current))

    def __repr__(self):
        return str(self)

    def __eq__(self, other):
        if type(other) == str:
            return self.current[0] == other

    def __ne__(self, other):
        if type(other) == str:
            return self.current[0] != other


Position = Tuple[int, int]
Valley = List[List[Cell]]


def move(coord: int, direction: int, size: int) -> int:
    next_coord = coord + direction
    if next_coord == 0:
        next_coord = size - 2
    elif next_coord == size - 1:
        next_coord = 1
    return next_coord


def move_blizzards(valley: Valley, direction=1):
    h, w = len(valley), len(valley[0])
    for i in range(1, h - 1):
        for j in range(1, w - 1):
            if valley[i][j].current[0] != ".":
                while valley[i][j].current:
                    blizzard = valley[i][j].current.popleft()
                    if blizzard == ">":
                        valley[i][move(j, direction, w)].future.append(">")
                    elif blizzard == "<":
                        valley[i][move(j, -direction, w)].future.append("<")
                    elif blizzard == "^":
                        valley[move(i, -direction, h)][j].future.append("^")
                    elif blizzard == "v":
                        valley[move(i, direction, h)][j].future.append("v")
    for i in range(1, h - 1):
        for j in range(1, w - 1):
            if valley[i][j].future:
                valley[i][j].current = valley[i][j].future
                valley[i][j].future = deque()
            else:
                valley[i][j].current.clear()
                valley[i][j].current.append(".")


def bfs(current: Deque[Optional[Position]], valley: Valley, target: Position):
    h, w = len(valley), len(valley[0])
    minutes = 0
    while True:
        position = current.popleft()
        if position is None:
            minutes += 1
            move_blizzards(valley)
            current = deque(set(current))
            current.append(None)
            continue
        if position == target:
            return minutes
        if valley[position[0]][position[1]] != ".":
            continue
        for direction in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)]:
            next_position = ((position[0] + direction[0]) % h, (position[1] + direction[1]) % w)
            current.append(next_position)


def part1(data: str):
    valley = list(map(lambda l: list(map(lambda c: Cell(c), l)), data.splitlines()))
    return bfs(deque([None, (0, 1)]), valley, (len(valley) - 1, len(valley[0]) - 2))


def part2(data: str):
    valley = list(map(lambda l: list(map(lambda c: Cell(c), l)), data.splitlines()))
    steps = bfs(deque([None, (0, 1)]), valley, (len(valley) - 1, len(valley[0]) - 2))
    steps += bfs(deque([None, (len(valley) - 1, len(valley[0]) - 2)]), valley, (0, 1))
    steps += bfs(deque([None, (0, 1)]), valley, (len(valley) - 1, len(valley[0]) - 2))
    return steps


if __name__ == "__main__":
    test = True
    test = False
    test_input = """#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_24_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
