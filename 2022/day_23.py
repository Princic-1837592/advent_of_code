from typing import Tuple, Dict, List

Point = Tuple[int, int]

ADJACENT = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
DIRECTIONS = [
    [
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)]
    ],
    [
        [(1, 0), (1, 1), (1, -1)],
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
        [(-1, 0), (-1, 1), (-1, -1)],
    ],
    [
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
    ],
    [
        [(0, 1), (-1, 1), (1, 1)],
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
        [(0, -1), (-1, -1), (1, -1)],
    ],
]


def slide_directions(directions):
    directions.append(directions.pop(0))


def make_proposals(elves, directions) -> Dict[Point, List[Point]]:
    proposals = {}
    for elf in elves:
        if not any(point in elves for point in map(lambda d: (elf[0] + d[0], elf[1] + d[1]), ADJACENT)):
            continue
        for cardinal_point in directions:
            if not any(point in elves for point in map(lambda d: (elf[0] + d[0], elf[1] + d[1]), cardinal_point)):
                next_position = (elf[0] + cardinal_point[0][0], elf[1] + cardinal_point[0][1])
                if next_position not in proposals:
                    proposals[next_position] = []
                proposals[next_position].append(elf)
                break
    return proposals


def make_moves(elves, proposals) -> bool:
    moved = False
    for proposal, targets in proposals.items():
        if len(targets) == 1:
            moved = True
            elves.remove(targets[0])
            elves.add(proposal)
    return moved


def part1(data: str):
    elves = {(i, j) for i, row in enumerate(data.splitlines()) for j, ground in enumerate(row) if ground == '#'}
    for r in range(10):
        proposals = make_proposals(elves, DIRECTIONS[r % 4])
        moved = make_moves(elves, proposals)
        if not moved:
            break
    min_x = min(elves, key=lambda e: e[0])[0]
    max_x = max(elves, key=lambda e: e[0])[0]
    min_y = min(elves, key=lambda e: e[1])[1]
    max_y = max(elves, key=lambda e: e[1])[1]
    w, h = max_x - min_x + 1, max_y - min_y + 1
    return w * h - len(elves)


def part2(data: str):
    elves = {(i, j) for i, row in enumerate(data.splitlines()) for j, ground in enumerate(row) if ground == '#'}
    r = 0
    while True:
        proposals = make_proposals(elves, DIRECTIONS[r % 4])
        moved = make_moves(elves, proposals)
        r += 1
        if not moved:
            break
    return r


if __name__ == "__main__":
    test = True
    test = False
    test_input = """....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."""
    if test:
        puzzle_input = test_input
    else:
        with open("day_23_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
