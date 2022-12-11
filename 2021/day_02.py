def part1(data: str):
    data = data.splitlines()
    h, d = 0, 0
    for line in data:
        instruction, value = line.split()
        value = int(value)
        if instruction[0] == "f":
            h += value
        elif instruction[0] == "d":
            d += value
        else:
            d -= value
    return h * d


def part2(data: str):
    data = data.splitlines()
    h, d, a = 0, 0, 0
    for line in data:
        instruction, value = line.split()
        value = int(value)
        if instruction[0] == "f":
            h += value
            d += value * a
        elif instruction[0] == "d":
            a += value
        else:
            a -= value
    return h * d


if __name__ == "__main__":
    test = True
    test = False
    test_input = """"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_02_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
