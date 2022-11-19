def part1(data: str):
    pass


def part2(data: str):
    pass


if __name__ == '__main__':
    test = True
    test = False
    test_input = ''''''
    if test:
        puzzle_input = test_input
    else:
        with open('day_21_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
