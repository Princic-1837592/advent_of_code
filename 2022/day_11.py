from collections import deque
from typing import List, Callable


class Monkeys:
    def __init__(self, data: List[str], worry_level_coefficient: int):
        self.monkeys = [Monkey(d) for d in data]
        self.worry_level_coefficient = worry_level_coefficient
        mcm = 1
        for monkey in self.monkeys:
            mcm *= monkey.divisible_by
        self.mcm = mcm

    def do_rounds(self, n: int):
        for _ in range(n):
            self._do_round()

    def _do_round(self):
        for monkey in self.monkeys:
            monkey.do_turn(self.monkeys, self.worry_level_coefficient, self.mcm)

    def __str__(self):
        return "\n".join(map(str, self.monkeys))


class Monkey:
    def __init__(self, data: str):
        _number, items, operation, divisible_by, if_true, if_false = map(str.strip, data.splitlines())
        self.items = deque(map(int, items[16:].split(", ")))
        self.operation = self.parse_operation(operation[21:])
        self.divisible_by = int(divisible_by[19:])
        self.if_true = int(if_true[25:])
        self.if_false = int(if_false[26:])
        self.inspected = 0

    def do_turn(self, monkeys: List['Monkey'], worry_level_coefficient: int, mcm: int):
        while self.items:
            self.inspected += 1
            self.items[0] = self.operation(self.items[0])
            self.items[0] %= mcm
            self.items[0] //= worry_level_coefficient
            self.throw(monkeys[self.if_true if self.items[0] % self.divisible_by == 0 else self.if_false])

    def throw(self, monkey: 'Monkey'):
        monkey.catch(self.items.popleft())

    def catch(self, item: int):
        self.items.append(item)

    @staticmethod
    def parse_operation(operation: str) -> Callable[[int], int]:
        operand, value = operation.split(" ")
        if operand == "+":
            return (lambda x: x + int(value)) if value != "old" else (lambda x: x + x)
        if operand == "*":
            return (lambda x: x * int(value)) if value != "old" else (lambda x: x * x)
        return lambda x: x

    def __str__(self):
        return f"Monkey(items={self.items}, operation={self.operation}, divisible_by={self.divisible_by}, " \
               f"if_true={self.if_true}, if_false={self.if_false}), inspected={self.inspected}"


def do_rounds(data: str, n: int, worry_level_coefficient: int):
    monkeys = Monkeys(data.split("\n\n"), worry_level_coefficient)
    monkeys.do_rounds(n)
    monkeys.monkeys.sort(key=lambda m: m.inspected, reverse=True)
    return monkeys.monkeys[0].inspected * monkeys.monkeys[1].inspected


def part1(data: str):
    return do_rounds(data, 20, 3)


def part2(data: str):
    return do_rounds(data, 10_000, 1)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_11_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
