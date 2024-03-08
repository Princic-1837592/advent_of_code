def part1(data: str):
    forest = data.split()
    visible = [[False for _ in range(len(forest[0]))] for _ in range(len(forest))]
    c = len(forest) * 2 + (len(forest[0]) - 2) * 2
    for i in range(len(forest)):
        visible[i][0] = True
        visible[i][-1] = True
    for j in range(len(forest[0])):
        visible[0][j] = True
        visible[-1][j] = True
    for i in range(1, len(forest) - 1):
        max_left = forest[i][0]
        max_right = forest[i][-1]
        for j in range(1, len(forest[0]) - 1):
            if forest[i][j] > max_left:
                max_left = forest[i][j]
                if not visible[i][j]:
                    visible[i][j] = True
                    c += 1
            if forest[i][-j - 1] > max_right:
                max_right = forest[i][-j - 1]
                if not visible[i][-j - 1]:
                    visible[i][-j - 1] = True
                    c += 1
    for j in range(1, len(forest[0]) - 1):
        max_above = forest[0][j]
        max_below = forest[-1][j]
        for i in range(1, len(forest) - 1):
            if forest[i][j] > max_above:
                max_above = forest[i][j]
                if not visible[i][j]:
                    visible[i][j] = True
                    c += 1
            if forest[-i - 1][j] > max_below:
                max_below = forest[-i - 1][j]
                if not visible[-i - 1][j]:
                    visible[-i - 1][j] = True
                    c += 1
    return c


def compute_scenic_score(forest, i, j) -> int:
    above, below, left, right = i, len(forest) - i - 1, j, len(forest[0]) - j - 1
    for i1 in range(i - 1, -1, -1):
        if forest[i1][j] >= forest[i][j]:
            above = i - i1
            break
    for i1 in range(i + 1, len(forest)):
        if forest[i1][j] >= forest[i][j]:
            below = i1 - i
            break
    for j1 in range(j - 1, -1, -1):
        if forest[i][j1] >= forest[i][j]:
            left = j - j1
            break
    for j1 in range(j + 1, len(forest[0])):
        if forest[i][j1] >= forest[i][j]:
            right = j1 - j
            break
    return above * below * left * right


def part2(data: str):
    forest = data.split()
    max_scenic_score = 0
    for i in range(1, len(forest) - 1):
        for j in range(1, len(forest[0]) - 1):
            max_scenic_score = max(max_scenic_score, compute_scenic_score(forest, i, j))
    return max_scenic_score


if __name__ == "__main__":
    test = True
    test = False
    test_input = """30373
25512
65332
33549
35390"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2022/day_08_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
