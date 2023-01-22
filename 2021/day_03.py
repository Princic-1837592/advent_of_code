def part1(data: str):
    data = data.split()
    gamma = ["0"] * len(data[0])
    epsilon = ["1"] * len(gamma)
    for j in range(len(gamma)):
        ones = 0
        for i in data:
            if i[j] == "1":
                ones += 1
        if ones > len(data) // 2:
            gamma[j] = "1"
            epsilon[j] = "0"
    return int("".join(gamma), 2) * int("".join(epsilon), 2)


def part2(data: str):
    data = data.split()
    filtered = data
    j = 0
    while j < len(data[0]) and len(filtered) > 1:
        ones = 0
        for i in filtered:
            if i[j] == "0":
                ones += 1
        filtered = list(filter(lambda x: x[j] == ("0" if ones > len(filtered) // 2 else "1"), filtered))
        j += 1
    oxygen = int("".join(filtered[0]), 2)
    filtered = data
    j = 0
    while j < len(data[0]) and len(filtered) > 1:
        zeroes = 0
        for i in filtered:
            if i[j] == "0":
                zeroes += 1
        filtered = list(filter(lambda x: x[j] == ("1" if zeroes > len(filtered) // 2 else "0"), filtered))
        j += 1
    return oxygen * int("".join(filtered[0]), 2)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_03_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
