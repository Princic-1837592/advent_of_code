from typing import List

SNAFU_TO_TEN = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2
}
TEN_TO_SNAFU = {
    -2: "=",
    -1: "-",
    0: "0",
    1: "1",
    2: "2"
}


def snafu_to_ten_digits(snafu: str) -> List[int]:
    result = list(map(SNAFU_TO_TEN.get, snafu))
    result.reverse()
    return result


def ten_digits_to_snafu(snafu: List[int]) -> str:
    result = list(map(TEN_TO_SNAFU.get, snafu))
    result.reverse()
    return "".join(result)


def snafu_to_ten(snafu: List[int]):
    total = 0
    c = 1
    for digit in snafu:
        total += digit * c
        c *= 5
    return total


def add_1(snafu: List[int]):
    for i, digit in enumerate(snafu):
        if digit == 2:
            snafu[i] = -1
        else:
            snafu[i] += 1
            break
    return snafu


def ten_to_snafu(ten: int):
    c = 0
    x = [1]
    while True:
        snafu_x = snafu_to_ten(x)
        if snafu_x >= ten:
            break
        x[c] += 1
        snafu_x = snafu_to_ten(x)
        if snafu_x >= ten:
            break
        c += 1
        x.append(1)
    c -= 1
    while snafu_to_ten(x) != ten:
        while snafu_to_ten(x) > ten:
            if x[c] == -2:
                c -= 1
            x[c] -= 1
        while snafu_to_ten(x) < ten:
            if x[c] == 2:
                c += 1
            x[c] += 1
        c -= 1
    return x


def part1(data: str):
    snafus = list(map(snafu_to_ten_digits, data.splitlines()))
    tens = list(map(snafu_to_ten, snafus))
    ten = sum(tens)
    snafu = ten_to_snafu(ten)
    return ten_digits_to_snafu(snafu)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2022/day_25_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
