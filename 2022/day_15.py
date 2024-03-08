import re

Y = 2000000


def part1(data: str):
    sensors = list(
        map(
            lambda t: ((t[0], t[1]), (t[2], t[3]), abs(t[0] - t[2]) + abs(t[1] - t[3])),
            map(lambda line: tuple(map(int, re.findall(r"-?\d+", line))), data.splitlines())
        )
    )
    interval = None
    for (x, y), _, d in sensors:
        if d >= abs(Y - y):
            x_min, x_max = x - (d - abs(Y - y)), x + (d - abs(Y - y))
            if interval is None:
                interval = x_min, x_max
            else:
                interval = min(interval[0], x_min), max(interval[1], x_max)
    size = interval[1] - interval[0] + 1
    for x, y in set(map(lambda t: t[1], sensors)):
        if y == Y and interval[0] <= x <= interval[1]:
            size -= 1
    return size


def part2(data: str):
    sensors = list(
        map(
            lambda t: ((t[0], t[1]), (t[2], t[3]), abs(t[0] - t[2]) + abs(t[1] - t[3])),
            map(lambda line: tuple(map(int, re.findall(r"-?\d+", line))), data.splitlines())
        )
    )
    for line in range(Y * 2 + 1):
        intervals = []
        for (x, y), _, d in sensors:
            if d >= abs(line - y):
                x_min, x_max = x - (d - abs(line - y)), x + (d - abs(line - y))
                for i_min, i_max in intervals.copy():
                    if (i_min - 1 <= x_min <= i_max + 1 or i_min - 1 <= x_max <= i_max + 1 or
                            x_min - 1 <= i_min <= x_max + 1 or x_min - 1 <= i_max <= x_max + 1):
                        x_min, x_max = min(i_min, x_min), max(i_max, x_max)
                        intervals.remove((i_min, i_max))
                intervals.append((x_min, x_max))
        if len(intervals) > 1:
            return 4000000 * (intervals[0][1] + 1) + line


if __name__ == "__main__":
    test = True
    test = False
    test_input = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""
    if test:
        puzzle_input = test_input
    else:
        with open("../inputs/2022/day_15_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
