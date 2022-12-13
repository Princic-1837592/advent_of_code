import json


def compare(left, right):
    if type(left) == type(right) == int:
        return left - right
    if type(left) == type(right):
        for i in range(min(len(left), len(right))):
            result = compare(left[i], right[i])
            if result != 0:
                return result
        return len(left) - len(right)
    if type(left) == int:
        return compare([left], right)
    return compare(left, [right])


def part1(data: str):
    pairs = list(tuple(map(json.loads, pair.splitlines())) for pair in data.split("\n\n"))
    count = 0
    for i, pair in zip(range(1, len(pairs) + 1), pairs):
        if compare(pair[0], pair[1]) < 0:
            count += i
    return count


def part2(data: str):
    packets = list(map(json.loads, data.split()))
    return ((len(list(filter(lambda p: compare(p, [[2]]) < 0, packets))) + 1) *
            (len(list(filter(lambda p: compare(p, [[6]]) < 0, packets))) + 2))


if __name__ == "__main__":
    test = True
    test = False
    test_input = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_13_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
