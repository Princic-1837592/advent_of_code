def part1(data: str):
    positions = list(map(int, data.split(',')))
    m, M = min(positions), max(positions)
    min_sum = M * len(positions)
    for n in range(m, M + 1):
        s = sum(map(lambda x: abs(x - n), positions))
        if s < min_sum:
            min_sum = s
    return min_sum


def part2(data: str):
    positions = list(map(int, data.split(',')))
    m, M = min(positions), max(positions)
    min_sum = M * len(positions) * (M * len(positions) + 1) // 2
    for n in range(m, M + 1):
        s = sum(map(lambda x: (abs(x - n) * (abs(x - n) + 1)) // 2, positions))
        if s < min_sum:
            min_sum = s
    return min_sum


if __name__ == '__main__':
    test = True
    test = False
    test_input = '''16,1,2,0,4,2,7,1,2,14'''
    if test:
        puzzle_input = test_input
    else:
        with open('day_7_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
