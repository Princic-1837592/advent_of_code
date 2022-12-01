import os

content = '''
def part1(data: str):
    pass


def part2(data: str):
    pass


if __name__ == "__main__":
    test = True
    # test = False
    test_input = """"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_{}_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
'''.strip()


def setup_calendar(year: str):
    if not os.path.exists(year):
        os.mkdir(year)
    for day in range(1, 25 + 1):
        path = os.path.join(year, f"day_{day}.py")
        if not os.path.exists(path):
            with open(path, "w") as f:
                f.write(content.format(day))
                f.write("\n")
        path = os.path.join(year, f"day_{day}_input.txt")
        if not os.path.exists(path):
            with open(path, "w") as _f:
                pass


if __name__ == "__main__":
    setup_calendar("2021")
