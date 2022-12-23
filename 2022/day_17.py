from collections import deque
from typing import List, Deque

Line = int
Shape = List[Line]
Chamber = Deque[Line]


# right is bottom
def horizontal():
    return [
        0b0_0011110
    ]


def cross():
    return [
        0b0_0001000,
        0b0_0011100,
        0b0_0001000
    ]


def left_l():
    return [
        0b0_0000100,
        0b0_0000100,
        0b0_0011100
    ]


def vertical():
    return [
        0b0_0010000,
        0b0_0010000,
        0b0_0010000,
        0b0_0010000
    ]


def square():
    return [
        0b0_0011000,
        0b0_0011000
    ]


def iter_shapes():
    while True:
        yield horizontal()
        yield cross()
        yield left_l()
        yield vertical()
        yield square()


def iter_moves(data: str):
    i = 0
    while True:
        yield data[i]
        i = (i + 1) % len(data)


def make_horizontal_move(shape: Shape, top_index: int, move: str, chamber: Chamber) -> bool:
    if move == ">":
        if any(map(lambda x: x & 0b0_0000001, shape)):
            return False
        after_move = list(map(lambda x: x >> 1, shape))
    else:
        if any(map(lambda x: x & 0b0_1000000, shape)):
            return False
        after_move = list(map(lambda x: x << 1, shape))
    for i, line in enumerate(after_move):
        if line & chamber[top_index + i]:
            return False
    for i in range(len(after_move)):
        shape[i] = after_move[i]
    return True


def can_move_down(shape: Shape, top_index: int, chamber: Chamber) -> bool:
    if top_index + len(shape) >= len(chamber):
        return False
    for i, line in enumerate(shape):
        if line & chamber[top_index + i + 1]:
            return False
    return True


def write(shape: Shape, top_index: int, chamber: Chamber):
    for i, line in enumerate(shape):
        chamber[top_index + i] |= line


def part1(data: str):
    chamber = deque(
        [0b0_0000000,
         0b0_0000000,
         0b0_0000000,
         0b0_0000000]
    )
    moves = iter_moves(data)
    highest_rock = len(chamber)
    fallen_rocks = 0
    for shape in iter_shapes():
        if fallen_rocks >= 2022:
            break
        top_index = highest_rock - 3 - len(shape)
        while True:
            make_horizontal_move(shape, top_index, next(moves), chamber)
            if can_move_down(shape, top_index, chamber):
                top_index += 1
            else:
                write(shape, top_index, chamber)
                fallen_rocks += 1
                highest_rock = min(highest_rock, top_index)
                while highest_rock < 7:
                    chamber.appendleft(0b0_0000000)
                    highest_rock += 1
                break
    return len(chamber) - highest_rock


def part2(data: str):
    chamber = deque(
        [0b0_0000000,
         0b0_0000000,
         0b0_0000000,
         0b0_0000000]
    )
    cache = {}
    moves = iter_moves(data)
    highest_rock = len(chamber)
    fallen_rocks = 0
    forgotten_lines = 0
    for shape in iter_shapes():
        if fallen_rocks >= 1000000000000:
            break
        top_index = highest_rock - 3 - len(shape)
        while True:
            make_horizontal_move(shape, top_index, next(moves), chamber)
            if can_move_down(shape, top_index, chamber):
                top_index += 1
            else:
                write(shape, top_index, chamber)
                fallen_rocks += 1
                highest_rock = min(highest_rock, top_index)
                while highest_rock < 7:
                    chamber.appendleft(0b0_0000000)
                    highest_rock += 1
                break
        while len(chamber) > 100:
            chamber.pop()
            forgotten_lines += 1
        hashable = tuple(chamber)
        cached = cache.get(hashable)
        if cached is not None:
            cached_fallen_rocks, cached_forgotten_lines = cached
            fallen_in_cycle = fallen_rocks - cached_fallen_rocks
            forgotten_in_cycle = forgotten_lines - cached_forgotten_lines
            left_to_go = 1000000000000 - fallen_rocks
            cycles_left = left_to_go // fallen_in_cycle
            fallen_rocks += cycles_left * fallen_in_cycle
            forgotten_lines += cycles_left * forgotten_in_cycle
        else:
            cache[hashable] = (fallen_rocks, forgotten_lines)
    return len(chamber) - highest_rock + forgotten_lines


if __name__ == "__main__":
    test = True
    test = False
    test_input = """>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_17_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
