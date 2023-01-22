closed = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}


def part1(data: str):
    lines = data.splitlines()
    points = {
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137,
    }
    result = 0
    for line in lines:
        stack = []
        for char in line:
            if char in "([{<":
                stack.append(char)
            else:
                popped = stack.pop()
                if char != closed[popped]:
                    result += points[char]
                    break
    return result


def is_incomplete(line: str):
    stack = []
    for char in line:
        if char in "([{<":
            stack.append(closed[char])
        else:
            if not stack:
                return True
            expected = stack.pop()
            if char != expected:
                return False
    return "".join(reversed(stack))


def score(completion: str) -> int:
    points = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    }
    score = 0
    for char in completion:
        score = score * 5 + points[char]
    return score


def part2(data: str):
    lines = list(filter(None, map(is_incomplete, data.splitlines())))
    results = list(map(score, lines))
    return sorted(results)[len(results) // 2]


if __name__ == "__main__":
    test = True
    test = False
    test_input = """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_10_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
