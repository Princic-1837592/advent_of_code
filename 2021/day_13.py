from typing import List, Tuple


def split_data(data: str) -> Tuple[List[List[int]], List[Tuple[str, int]]]:
    def split_instruction(line: str):
        split = line.split("=")
        return split[0][-1], int(split[1])

    dots, instructions = data.split("\n\n")
    coords = list(map(lambda x: tuple(map(int, reversed(x.split(",")))), dots.splitlines()))
    height, width = max(coords, key = lambda x: x[0])[0], max(coords, key = lambda x: x[1])[1]
    paper = [[0 for _ in range(width + 1)] for _ in range(height + 1)]
    for x, y in coords:
        paper[x][y] = 1
    return paper, list(map(split_instruction, instructions.splitlines()))


def fold(paper: List[List[int]], direction: str, index: int):
    if direction == "x":
        for offset in reversed(range(1, len(paper[0]) - index)):
            for i in range(len(paper)):
                if paper[i][index + offset] == 1:
                    paper[i][index - offset] = 1
                paper[i].pop()
        for i in range(len(paper)):
            paper[i].pop()
    elif direction == "y":
        for offset in reversed(range(1, len(paper) - index)):
            for j in range(len(paper[0])):
                if paper[index + offset][j] == 1:
                    paper[index - offset][j] = 1
            paper.pop()
        paper.pop()


def count_dots(paper: List[List[int]]):
    dots = 0
    for row in paper:
        for dot in row:
            if dot == 1:
                dots += 1
    return dots


def part1(data: str):
    paper, instructions = split_data(data)
    fold(paper, *instructions[0])
    return count_dots(paper)


def part2(data: str):
    paper, instructions = split_data(data)
    for direction, index in instructions:
        fold(paper, direction, index)
    return "\n".join(map(lambda l: "".join(map(lambda d: " " if d == 0 else "#", l)), paper))


if __name__ == "__main__":
    test = True
    test = False
    test_input = """6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_13_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
