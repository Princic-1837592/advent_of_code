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
        else:
            self.left = Number(values[0], self)
            self.right = Number(values[1], self)

    def reduce(self, level=0) -> None:
        while True:
            if type(self.left) != int and self.left._explode(level + 1):
                continue
            if type(self.right) != int and self.right._explode(level + 1):
                continue
            # todo
            break

    def _explode(self, level=0) -> bool:
        while self.left._explode(level + 1):
            pass
        while self.right._explode(level + 1):
            pass
        if type(self.left) == int and type(self.right) == int:
            return False

    def _sum_to_leftmost_child(self, value: int, from_child: "Number") -> None:
        pass

    def _sum_to_rightmost_child(self, value: int, from_child: "Number") -> None:
        pass

    def _split(self):
        if type(self.value) == int:
            return self.value
        return self.left._split() * self.right._split()

    def _to_string(self, level=0, space="|   "):
        if self.is_leaf:
            return f"{space * level}{self.value}"
        return f"{self.left._to_string(level + 1, space)}\n{space * level}<\n{self.right._to_string(level + 1, space)}"

    def __str__(self):
        return self._to_string(space="   ")


def parse(data: str) -> List[Number]:
    return list(map(Number, map(loads, data.splitlines())))


def part1(data: str):
    numbers = parse(data)
    print(numbers[0])


def part2(data: str):
    pass


if __name__ == "__main__":
    test = True
    # test = False
    test_input = """[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"""
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
