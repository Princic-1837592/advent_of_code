from typing import List


def simulate(days, limit: int) -> List[int]:
    fishes = [0] * 10
    for i in days:
        fishes[i + 1] += 1
    for _ in range(limit):
        for day in range(1, len(fishes)):
            fishes[day - 1] = fishes[day]
        fishes[7] += fishes[0]
        fishes[9] = fishes[0]
        fishes[0] = 0
    return fishes


def part1(data: str):
    days = list(map(int, data.split(",")))
    fishes = simulate(days, 80)
    return sum(fishes)


def part2(data: str):
    days = list(map(int, data.split(",")))
    fishes = simulate(days, 256)
    return sum(fishes)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """3,4,3,1,2"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_6_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
