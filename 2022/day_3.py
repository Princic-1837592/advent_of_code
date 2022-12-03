priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


def part1(data: str):
    rucksacks = list(map(lambda r: (r[:len(r) // 2], r[len(r) // 2:]), data.split()))
    c = 0
    for left, right in rucksacks:
        c += priorities.index(set(left).intersection(set(right)).pop()) + 1
    return c


def part2(data: str):
    rucksacks = data.split()
    c = 0
    for i in range(0, len(rucksacks), 3):
        c += priorities.index(
            set(rucksacks[i])
            .intersection(set(rucksacks[i + 1]))
            .intersection(set(rucksacks[i + 2]))
            .pop()
        ) + 1
    return c


if __name__ == "__main__":
    test = True
    test = False
    test_input = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_3_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
