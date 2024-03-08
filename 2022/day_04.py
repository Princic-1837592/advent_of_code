def part1(data: str):
    pairs = list(map(lambda x: tuple(map(lambda y: tuple(map(int, y.split("-"))), x.split(","))), data.split()))
    c = 0
    for x, y in pairs:
        if x[0] <= y[0] and x[1] >= y[1] or y[0] <= x[0] and y[1] >= x[1]:
            c += 1
    return c


def part2(data: str):
    pairs = list(map(lambda x: tuple(map(lambda y: tuple(map(int, y.split("-"))), x.split(","))), data.split()))
    c = 0
    for x, y in pairs:
        if x[0] <= y[0] <= x[1] or y[0] <= x[0] <= y[1]:
            c += 1
    return c


if __name__ == "__main__":
    test = True
    test = False
    test_input = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2022/day_04_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
