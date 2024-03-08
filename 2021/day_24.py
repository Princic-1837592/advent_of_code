# https://adventofcode.com/2021/day/24
# https://adventofcode.com/2021/day/24/input
from typing import List, Tuple


def parse(data: str) -> List[Tuple[str, str, str]]:
    return list(map(lambda line: tuple(line.split()), data.splitlines()))


def get_equations(instructions: List[Tuple[str, str, str]]) -> List[Tuple[int, int, int]]:
    stack = []
    i = 0
    digit = 0
    equations = []
    while i < len(instructions):
        check = int(instructions[i + 5][2])
        offset = int(instructions[i + 15][2])
        if check > 0:
            stack.append((digit, offset))
        else:
            pop_digit, pop_offset = stack.pop()
            diff = pop_offset + check
            if diff >= 0:
                equations.append((pop_digit, digit, -diff))
            else:
                equations.append((digit, pop_digit, diff))
        digit += 1
        i += 18
    return equations


def part1(data: str) -> int:
    instructions = parse(data)
    equations = get_equations(instructions)
    number = [0] * 14
    for a, b, c in equations:
        number[b] = 9
        number[a] = 9 + c
    return int("".join(map(str, number)))


def part2(data: str) -> int:
    instructions = parse(data)
    equations = get_equations(instructions)
    number = [0] * 14
    for a, b, c in equations:
        number[a] = 1
        number[b] = 1 - c
    return int("".join(map(str, number)))


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2021/day_24_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
