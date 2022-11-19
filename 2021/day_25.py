from typing import List


def move(bottom_of_the_sea: List[List[str]], cucumber: str, i_coeff: int, j_coeff: int) -> bool:
    can_move = [[False for _ in range(len(bottom_of_the_sea[0]))] for _ in range(len(bottom_of_the_sea))]
    moved = False
    for i in range(len(bottom_of_the_sea)):
        for j in range(len(bottom_of_the_sea[i])):
            i1, j1 = (i + i_coeff) % len(bottom_of_the_sea), (j + j_coeff) % len(bottom_of_the_sea[i])
            if bottom_of_the_sea[i][j] == cucumber and bottom_of_the_sea[i1][j1] == '.':
                can_move[i][j] = True
    for i in range(len(bottom_of_the_sea)):
        for j in range(len(bottom_of_the_sea[i])):
            if can_move[i][j]:
                i1, j1 = (i + i_coeff) % len(bottom_of_the_sea), (j + j_coeff) % len(bottom_of_the_sea[i])
                bottom_of_the_sea[i1][j1] = cucumber
                bottom_of_the_sea[i][j] = '.'
                moved = True
    return moved


def part1(data: str):
    bottom_of_the_sea = [[c for c in line] for line in data.split()]
    i = 0
    while True:
        moved = False
        moved = move(bottom_of_the_sea, '>', 0, 1) or moved
        moved = move(bottom_of_the_sea, 'v', 1, 0) or moved
        i += 1
        if not moved:
            return i


def part2(data: str):
    pass


if __name__ == '__main__':
    test = True
    test = False
    test_input = '''v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>'''
    if test:
        puzzle_input = test_input
    else:
        with open('day_25_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
'''v...>.>vv>
.vv>.>vv..
>.>>v...>v
>>v>.>.>v.
v>v.vv.v..
.>>..>v...
.vv...>>v.
v.v..>>v.v
>...v..v..'''
