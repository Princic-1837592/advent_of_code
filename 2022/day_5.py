import re


def part1(data: str):
    crates, moves = data.split("\n\n")
    moves = list(map(lambda m: tuple(map(int, re.findall(r"\d+", m))), moves.split("\n")))
    lines = crates.split("\n")
    cols = len(lines[-1].strip().split())
    stacks = [[] for _ in range(cols)]
    for c, col in enumerate(range(1, len(lines[0]), 4)):
        for i in reversed(range(len(lines) - 1)):
            if lines[i][col] != " ":
                stacks[c].append(lines[i][col])
    for n, src, dest in moves:
        for _ in range(n):
            stacks[dest - 1].append(stacks[src - 1].pop())
    return "".join(map(lambda s: s[-1], stacks))


def part2(data: str):
    crates, moves = data.split("\n\n")
    moves = list(map(lambda m: tuple(map(int, re.findall(r"\d+", m))), moves.split("\n")))
    lines = crates.split("\n")
    cols = len(lines[-1].strip().split())
    stacks = [[] for _ in range(cols)]
    for c, col in enumerate(range(1, len(lines[0]), 4)):
        for i in reversed(range(len(lines) - 1)):
            if lines[i][col] != " ":
                stacks[c].append(lines[i][col])
    crane = []
    for n, src, dest in moves:
        for _ in range(n):
            crane.append(stacks[src - 1].pop())
        for _ in range(n):
            stacks[dest - 1].append(crane.pop())
    return "".join(map(lambda s: s[-1], stacks))


if __name__ == "__main__":
    test = True
    test = False
    test_input = """    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_5_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
