def part1(data: str):
    data = list(map(int, data.split()))
    c = 0
    for i in range(1, len(data)):
        if data[i] > data[i - 1]:
            c += 1
    return c


def part2(data: str):
    data = list(map(int, data.split()))
    c = 0
    for i in range(3, len(data)):
        ow = data[i - 3] + data[i - 2] + data[i - 1]
        w = data[i - 2] + data[i - 1] + data[i]
        if w > ow:
            c += 1
    return c


if __name__ == "__main__":
    test = True
    test = False
    test_input = """"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_1_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
