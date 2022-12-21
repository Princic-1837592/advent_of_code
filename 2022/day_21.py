from re import match
from typing import Tuple


def compute(monkeys, monkey):
    if type(monkeys[monkey]) == int:
        return monkeys[monkey]
    operation, left, right = monkeys[monkey]
    left, right = compute(monkeys, left), compute(monkeys, right)
    if operation == "+":
        return left + right
    elif operation == "*":
        return left * right
    elif operation == "-":
        return left - right
    elif operation == "/":
        return left / right


def part1(data: str):
    monkeys = {}
    for line in data.splitlines():
        if monkey := match(r"([a-z]+): (\d+)", line):
            monkeys[monkey.group(1)] = int(monkey.group(2))
        else:
            monkey = match(r"([a-z]+): ([a-z]+) (.) ([a-z]+)", line)
            monkeys[monkey.group(1)] = (monkey.group(3), monkey.group(2), monkey.group(4))
    return compute(monkeys, "root")


def compute_and_find_human(monkeys, monkey, parents, results) -> int:
    if type(monkeys[monkey]) == int:
        results[monkey] = monkeys[monkey]
        return monkeys[monkey]
    operation, left_monkey, right_monkey = monkeys[monkey]
    left = compute_and_find_human(monkeys, left_monkey, parents, results)
    right = compute_and_find_human(monkeys, right_monkey, parents, results)
    if left_monkey in parents or right_monkey in parents:
        parents.add(monkey)
    result = None
    if operation == "+":
        result = left + right
    elif operation == "*":
        result = left * right
    elif operation == "-":
        result = left - right
    elif operation == "/":
        result = left / right
    results[monkey] = result
    return result


inverse_left = {
    "-": lambda solution, right: solution + right,
    "+": lambda solution, right: solution - right,
    "/": lambda solution, right: solution * right,
    "*": lambda solution, right: solution / right,
}

inverse_right = {
    "-": lambda solution, left: left - solution,
    "+": lambda solution, left: solution - left,
    "/": lambda solution, left: left / solution,
    "*": lambda solution, left: solution / left,
}


def find_x(monkeys, monkey, parents, solution, results) -> float:
    if monkey == "humn":
        return solution
    operation, left_monkey, right_monkey = monkeys[monkey]
    if left_monkey in parents:
        return find_x(monkeys, left_monkey, parents, inverse_left[operation](solution, results[right_monkey]), results)
    return find_x(monkeys, right_monkey, parents, inverse_right[operation](solution, results[left_monkey]), results)


def part2(data: str):
    monkeys = {}
    for line in data.splitlines():
        if monkey := match(r"(.+): (\d+)$", line):
            monkeys[monkey.group(1)] = int(monkey.group(2))
        else:
            monkey = match(r"(.+): (.+) (.) (.+)", line)
            monkeys[monkey.group(1)] = (monkey.group(3), monkey.group(2), monkey.group(4))
    parents = {"root", "humn"}
    results = {}
    _, left_monkey, right_monkey = monkeys["root"]
    left = compute_and_find_human(monkeys, left_monkey, parents, results)
    right = compute_and_find_human(monkeys, right_monkey, parents, results)
    human_on_left = left_monkey in parents
    if human_on_left:
        objective = right
        variable = left_monkey
    else:
        objective = left
        variable = right_monkey
    return find_x(monkeys, variable, parents, objective, results)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_21_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
