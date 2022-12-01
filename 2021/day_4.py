from typing import List


def check_win(board: List[List[int]], picked: List[bool]) -> bool:
    for coord in range(len(board)):
        if all(picked[n] for n in board[coord]):
            return True
        if all(picked[board[j][coord]] for j in range(len(board))):
            return True
    return False


def compute_score(board: List[List[int]], picked: List[bool], last_picked: int) -> int:
    sum = 0
    for i in range(len(board)):
        for number in board[i]:
            if not picked[number]:
                sum += number
    return sum * last_picked


def part1(data: str):
    sequence, *boards = data.split("\n\n")
    picked = [False] * len(sequence)
    boards = [[list(map(int, line.split())) for line in board.splitlines()] for board in boards]
    for number in map(int, sequence.split(",")):
        picked[number] = True
        for board in boards:
            if check_win(board, picked):
                return compute_score(board, picked, number)


def part2(data: str):
    sequence, *boards = data.split("\n\n")
    picked = [False] * len(sequence)
    boards = [[list(map(int, line.split())) for line in board.splitlines()] for board in boards]
    has_won = [False] * len(boards)
    won_last = -1
    last_picked = -1
    last_score= 0
    for number in map(int, sequence.split(",")):
        picked[number] = True
        for i, board in enumerate(boards):
            if not has_won[i] and check_win(board, picked):
                has_won[i] = True
                won_last = i
                last_picked = number
                last_score = compute_score(board, picked, number)
    return last_score


if __name__ == "__main__":
    test = True
    test = False
    test_input = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_4_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
