from typing import Set, Tuple


class HeadAndTail:
    moves = {
        'R': (1, 0),
        'L': (-1, 0),
        'U': (0, 1),
        'D': (0, -1),
    }

    def __init__(self):
        self.head = (0, 0)
        self.tail = self.head
        self.tail_visited: Set[Tuple[int, int]] = {self.tail}

    def move(self, direction: str, n: int):
        move = self.moves[direction]
        for _ in range(n):
            self.__move_one(move)

    def __move_one(self, move: Tuple[int, int]):
        old_head = self.head
        self.head = (self.head[0] + move[0], self.head[1] + move[1])
        if abs(self.head[0] - self.tail[0]) > 1 or abs(self.head[1] - self.tail[1]) > 1:
            self.tail = old_head
            self.tail_visited.add(self.tail)


def part1(data: str):
    ht = HeadAndTail()
    for move in data.splitlines():
        direction, n_str = move.split()
        n = int(n_str)
        ht.move(direction, n)
    return len(ht.tail_visited)


class GeneralizedHeadAndTail:
    moves = {
        'R': (1, 0),
        'L': (-1, 0),
        'U': (0, 1),
        'D': (0, -1),
    }

    def __init__(self, length: int):
        self.knots = [(0, 0)] * length
        self.tail_visited: Set[Tuple[int, int]] = {self.knots[-1]}

    def move(self, direction: str, n: int):
        move = self.moves[direction]
        for _ in range(n):
            self.__move_one(move)

    def __move_one(self, move: Tuple[int, int]):
        self.knots[0] = (self.knots[0][0] + move[0], self.knots[0][1] + move[1])
        for knot in range(1, len(self.knots)):
            if (abs(self.knots[knot][0] - self.knots[knot - 1][0]) > 1 or
                    abs(self.knots[knot][1] - self.knots[knot - 1][1]) > 1):
                if self.knots[knot][0] == self.knots[knot - 1][0] or self.knots[knot][1] == self.knots[knot - 1][1]:
                    self.knots[knot] = (self.knots[knot][0] + (self.knots[knot - 1][0] - self.knots[knot][0]) // 2,
                                        self.knots[knot][1] + (self.knots[knot - 1][1] - self.knots[knot][1]) // 2)
                else:
                    for diagonal in [(-1, -1), (-1, 1), (1, -1), (1, 1)]:
                        new_possible_knot = (self.knots[knot][0] + diagonal[0], self.knots[knot][1] + diagonal[1])
                        if (abs(new_possible_knot[0] - self.knots[knot - 1][0]) <= 1 and
                                abs(new_possible_knot[1] - self.knots[knot - 1][1]) <= 1):
                            self.knots[knot] = new_possible_knot
                            break
        self.tail_visited.add(self.knots[-1])


def part2(data: str):
    ght = GeneralizedHeadAndTail(10)
    for move in data.splitlines():
        direction, n_str = move.split()
        n = int(n_str)
        ght.move(direction, n)
    return len(ght.tail_visited)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""
    test_input = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_9_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
