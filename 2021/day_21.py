# https://adventofcode.com/2021/day/21
# https://adventofcode.com/2021/day/21/input
from typing import Tuple


def parse(data: str) -> Tuple[int, int]:
    player1, player2 = data.split("\n")
    return int(player1.split(": ")[1]), int(player2.split(": ")[1])


def part1(data: str):
    player1, player2 = parse(data)
    positions = [player1 - 1, player2 - 1]
    points = [0, 0]
    dice = 0
    while True:
        for i in range(2):
            for _ in range(3):
                dice += 1
                positions[i] += dice % 100
            positions[i] %= 10
            points[i] += positions[i] + 1
            if points[i] >= 1000:
                return points[1 - i] * dice


def part2(data: str):
    def turn(position1: int, position2: int, points1: int, points2: int) -> Tuple[int, int]:
        if (position1, position2, points1, points2) in seen:
            return seen[(position1, position2, points1, points2)]
        wins1, wins2 = 0, 0
        for total1, prob1 in dice.items():
            new_position1 = position1 + total1
            if new_position1 > 10:
                new_position1 %= 10
            new_points1 = points1 + new_position1
            if new_points1 >= 21:
                wins1 += prob1
                continue
            new_wins2, new_wins1 = turn(position2, new_position1, points2, new_points1)
            wins1 += prob1 * new_wins1
            wins2 += prob1 * new_wins2
        seen[(position1, position2, points1, points2)] = (wins1, wins2)
        return wins1, wins2

    dice = {}
    for d1 in range(1, 3 + 1):
        for d2 in range(1, 3 + 1):
            for d3 in range(1, 3 + 1):
                dice[d1 + d2 + d3] = dice.get(d1 + d2 + d3, 0) + 1
    player1, player2 = parse(data)
    seen = {}
    return max(turn(player1, player2, 0, 0))


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """Player 1 starting position: 4
Player 2 starting position: 8"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2021/day_21_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
