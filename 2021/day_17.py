def part1(data: str):
    _, coords = data.split(" ", 1)
    x_str, y_str = coords.split(", ")
    x = tuple(map(int, x_str.split("=")[1].split("..")))
    y = tuple(map(int, y_str.split("=")[1].split("..")))
    print(x, y)
    x_mirrored = False
    if all(map(lambda x: x < 0, x)):
        x_mirrored = True
        x = tuple(map(lambda x: -x, x))
    print(x, y, x_mirrored)


def part2(data: str):
    pass


if __name__ == "__main__":
    test = True
    # test = False
    test_input = """target area: x=20..30, y=-10..-5"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_17_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
