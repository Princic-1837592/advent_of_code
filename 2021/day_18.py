from typing import List, Optional
from json import loads


class Number:
    def __init__(self, values: list | int, father=None):
        self.value: Optional[int] = None
        self.left: Optional[Number] = None
        self.right: Optional[Number] = None
        self.is_leaf = False
        self.father: Optional[Number] = father
        if type(values) == int:
            self.value = values
            self.is_leaf = True
            self.nodes = 1
        else:
            if type(values[0]) == Number:
                self.left = values[0]
                self.left.father = self
                self.right = values[1]
                self.right.father = self
            else:
                self.left = Number(values[0], self)
                self.right = Number(values[1], self)
            self.nodes = self.left.nodes + self.right.nodes

    def reduce(self):
        while True:
            if self._explode(self, 0, 0, 0):
                continue
            if self._split():
                continue
            break

    def _add_to_nth_number(self, n: int, value: int, on_left: int):
        if self.is_leaf:
            if on_left == n - 1:
                self.value += value
            return
        if on_left + self.left.nodes >= n:
            self.left._add_to_nth_number(n, value, on_left)
        else:
            self.right._add_to_nth_number(n, value, on_left + self.left.nodes)

    def _explode(self, root: "Number", on_left: int, on_right: int, level: int) -> bool:
        if self.is_leaf:
            return False
        if level >= 4:
            assert self.left.is_leaf and self.right.is_leaf
            if on_left > 0:
                root._add_to_nth_number(on_left, self.left.value, 0)
            if on_right > 0:
                root._add_to_nth_number(on_left + 3, self.right.value, 0)
            self.left = None
            self.right = None
            self.value = 0
            self.is_leaf = True
            father = self
            while father is not None:
                father.nodes -= 1
                father = father.father
            return True
        return (
                self.left._explode(root, on_left, on_right + self.left.nodes, level + 1)
                or
                self.right._explode(root, on_left + self.left.nodes, on_right, level + 1)
        )

    def _split(self) -> bool:
        if self.is_leaf:
            if self.value >= 10:
                self.left = Number(self.value // 2, self)
                self.right = Number(self.value - self.left.value, self)
                self.value = None
                self.is_leaf = False
                father = self
                while father is not None:
                    father.nodes += 1
                    father = father.father
                return True
            return False
        return self.left._split() or self.right._split()

    def _to_lines(self, centered: bool) -> List[str]:
        if self.value is not None:
            return ["|", str(self.value)]
        left, right = self.left._to_lines(centered), self.right._to_lines(centered)
        width = len(left[0]) + len(right[0]) + 3
        left_spaces = left[0].find("|") + 1
        right_spaces = len(right[0]) - right[0].rfind("|")
        if centered:
            left_underscores = width // 2 - left_spaces
        else:
            left_underscores = len(left[0]) + 1 - left_spaces
        if centered:
            right_underscores = width // 2 - right_spaces
        else:
            right_underscores = len(right[0]) + 1 - right_spaces
        lines = [" " * left_spaces + "_" * left_underscores + "|" + "_" * right_underscores + " " * right_spaces]
        for i in range(max(len(left), len(right))):
            lines.append(
                (left[i] if i < len(left) else " " * len(left[0]))
                + "   "
                + (right[i] if i < len(right) else " " * len(right[0]))
            )
        return lines

    def magnitude(self) -> int:
        if self.is_leaf:
            return self.value
        return self.left.magnitude() * 3 + self.right.magnitude() * 2

    def copy(self) -> "Number":
        def inner(node: Number) -> Number:
            if node.is_leaf:
                return Number(node.value)
            return Number([inner(node.left), inner(node.right)])

        def give_father(node: Number):
            if node.is_leaf:
                return
            node.left.father = node
            node.right.father = node
            give_father(node.left)
            give_father(node.right)

        result = inner(self)
        give_father(result)
        return result

    def to_string(self, centered=True):
        return "\n".join(self._to_lines(centered))

    def __str__(self):
        return self.to_string()

    def __add__(self, other: "Number") -> "Number":
        return Number([self, other], None)


def parse(data: str) -> List[Number]:
    return list(map(Number, map(loads, data.splitlines())))


def part1(data: str):
    numbers = parse(data)
    result = numbers[0]
    result.reduce()
    for i in range(1, len(numbers)):
        result = result + numbers[i]
        result.reduce()
    return result.magnitude()


def part2(data: str):
    numbers = parse(data)
    max_magnitude = 0
    for i in range(len(numbers)):
        for j in range(i + 1, len(numbers)):
            result = numbers[i] + numbers[j]
            result = result.copy()
            result.reduce()
            max_magnitude = max(max_magnitude, result.magnitude())
            result = numbers[j] + numbers[i]
            result = result.copy()
            result.reduce()
            max_magnitude = max(max_magnitude, result.magnitude())
    return max_magnitude


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_18_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
