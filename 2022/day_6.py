from collections import deque


def find_first(data, l):
    queue = deque()
    for i in range(l - 1):
        queue.append(data[i])
    for i in range(l - 1, len(data)):
        queue.append(data[i])
        if len(set(queue)) == l:
            return i + 1
        queue.popleft()


def part1(data: str):
    return find_first(data, 4)


def part2(data: str):
    return find_first(data, 14)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_6_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
