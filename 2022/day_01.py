def part1(data: str):
    elves = data.split("\n\n")
    elves = list(map(sum, map(lambda x: map(int, x.split("\n")), elves)))
    return max(elves)


def part2(data: str):
    elves = data.split("\n\n")
    elves = list(map(sum, map(lambda x: map(int, x.split("\n")), elves)))
    return sum(sorted(elves, reverse=True)[:3])


if __name__ == "__main__":
    test = True
    test = False
    test_input = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_01_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
