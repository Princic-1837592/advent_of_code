wins = {
    "A": "C",
    "B": "A",
    "C": "B"
}
loses = {
    "A": "B",
    "B": "C",
    "C": "A"
}
points = {
    "A": 1,
    "B": 2,
    "C": 3
}
to_abc = {
    "X": "A",
    "Y": "B",
    "Z": "C",
}


def part1(data: str):
    rounds = list(map(lambda l: (l.split()[0], to_abc[(l.split()[1])]), data.splitlines()))
    score = 0
    for other, me in rounds:
        score += points[me]
        if wins[me] == other:
            score += + 6
        if me == other:
            score += + 3
    return score


def part2(data: str):
    rounds = list(map(str.split, data.splitlines()))
    score = 0
    for other, result in rounds:
        if result == "X":
            score += 0
            score += points[wins[other]]
        if result == "Y":
            score += 3
            score += points[other]
        if result == "Z":
            score += 6
            score += points[loses[other]]
    return score


if __name__ == "__main__":
    test = True
    test = False
    test_input = """A Y
B X
C Z"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_2_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
