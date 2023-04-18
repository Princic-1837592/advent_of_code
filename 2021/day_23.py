# https://adventofcode.com/2021/day/23
# https://adventofcode.com/2021/day/23/input
from typing import Tuple, List
from collections import deque


class State:
    def __init__(self, n: int):
        self.matrix = [-1] * n


wall = 0
empty = 1
type_a = 2
type_b = 3
type_c = 4
type_d = 5

"""
steps = [
    [0, 1, 3, 5, 7, 9, 10, 3, 4, 5, 6, 7, 8, 9, 10],
    [1, 0, 2, 4, 6, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9],
    [3, 2, 0, 2, 4, 6, 7, 2, 3, 2, 3, 4, 5, 6, 7],
    [5, 4, 2, 0, 2, 4, 5, 4, 5, 2, 3, 2, 3, 4, 5],
    [7, 6, 4, 2, 0, 2, 3, 6, 7, 4, 5, 2, 3, 2, 3],
    [9, 8, 6, 4, 2, 0, 1, 8, 9, 6, 7, 4, 5, 2, 3],
    [10, 9, 7, 5, 3, 1, 0, 9, 10, 7, 8, 5, 6, 3, 4],
    [3, 2, 2, 4, 6, 8, 9, 0, 1, 4, 5, 6, 7, 8, 9],
    [4, 3, 3, 5, 7, 9, 10, 1, 0, 5, 6, 7, 8, 9, 10],
    [5, 4, 2, 2, 4, 6, 7, 4, 5, 0, 1, 4, 5, 6, 7],
    [6, 5, 3, 3, 5, 7, 8, 5, 6, 1, 0, 5, 6, 7, 8],
    [7, 6, 4, 2, 2, 4, 5, 6, 7, 4, 5, 0, 1, 4, 5],
    [8, 7, 5, 3, 3, 5, 6, 7, 8, 5, 6, 1, 0, 5, 6],
    [9, 8, 6, 4, 2, 2, 3, 8, 9, 6, 7, 4, 5, 0, 1],
    [10, 9, 7, 5, 3, 3, 4, 9, 10, 7, 8, 5, 6, 1, 0],
]
"""

GRAPH = [
    [(1, 1)],
    [(0, 1), (7, 2), (2, 2)],
    [(1, 2), (7, 2), (9, 2), (3, 2)],
    [(2, 2), (9, 2), (11, 2), (4, 2)],
    [(3, 2), (11, 2), (13, 2), (5, 2)],
    [(4, 2), (13, 2), (6, 1)],
    [(5, 1)],
    [(1, 2), (8, 1), (2, 2)],
    [(7, 1)],
    [(2, 2), (10, 1), (3, 2)],
    [(9, 1)],
    [(3, 2), (12, 1), (4, 2)],
    [(11, 1)],
    [(4, 2), (14, 1), (5, 2)],
    [(13, 1)],
]


def bfs(src: int) -> List[int]:
    dist = [-1] * len(GRAPH)
    dist[src] = 0
    queue = deque([src])
    while queue:
        node = queue.popleft()
        for neighbor, cost in GRAPH[node]:
            if dist[neighbor] == -1:
                dist[neighbor] = dist[node] + cost
                queue.append(neighbor)
    return dist


STEPS = [bfs(i) for i in range(len(GRAPH))]
CORRECT_ROOM = [[7, 8], [9, 10], [11, 12], [13, 14]]


def is_hallway(pos: int) -> bool:
    return pos <= 6


def is_correct_room(amphipod: int, pos: int) -> bool:
    return pos in CORRECT_ROOM[amphipod // 2]


def move_cost(amphipod: int, src: int, dst: int) -> int:
    return STEPS[src][dst] * (10 ** (amphipod // 2))


def parse(data: str) -> List[int]:
    result = [-1, -1, -1, -1, -1, -1, -1, -1]
    for i, g in [(31, 7), (33, 9), (35, 11), (37, 13), (45, 8), (47, 10,), (49, 12), (51, 14)]:
        if data[i] == "A":
            if result[0] == -1:
                result[0] = g
            else:
                result[1] = g
        elif data[i] == "B":
            if result[2] == -1:
                result[2] = g
            else:
                result[3] = g
        elif data[i] == "C":
            if result[4] == -1:
                result[4] = g
            else:
                result[5] = g
        elif data[i] == "D":
            if result[6] == -1:
                result[6] = g
            else:
                result[7] = g
    return result


def solve(state: List[int]) -> int:
    def to_number() -> int:
        result = 0
        for i in state:
            result *= 100
            result += i
        return result

    def is_reachable(src: int, dst: int) -> bool:
        occupied = [False] * len(GRAPH)
        for d in state:
            if d == dst:
                return False
            occupied[d] = True
        queue = deque([src])
        visited = [False] * len(GRAPH)
        while queue:
            node = queue.popleft()
            if node == dst:
                return True
            visited[node] = True
            for neighbor, cost in GRAPH[node]:
                if not occupied[neighbor] and not visited[neighbor]:
                    queue.append(neighbor)

    def dfs(cost: int, global_min: int) -> int:
        # print(cost)
        if cost >= global_min:
            # print("pruned cost")
            return cost + 1
        number = to_number()
        if number in seen:
            # print("seen", number, cost, seen[number])
            if cost >= seen[number]:
                # print("pruned seen")
                return cost + 1
        if all(map(lambda i: state[i] in CORRECT_ROOM[i // 2], range(len(state)))):
            return cost
        min_cost = global_min
        reachable = [
            [dst for dst in range(len(GRAPH)) if is_reachable(src, dst)]
            for src in state
        ]
        for amphipod, src in enumerate(state):
            if is_hallway(src):
                correct_rooms = CORRECT_ROOM[amphipod // 2]
                if reachable[correct_rooms[1]]:
                    dst = correct_rooms[1]
                    state[amphipod] = dst
                    min_cost = min(min_cost, dfs(cost + move_cost(amphipod, src, dst), min_cost))
                    state[amphipod] = src
                elif reachable[correct_rooms[0]] and correct_rooms[1] not in state:
                    pass
            else:
                pass
            # for dst in reachable[src]:
            #     if is_hallway(src) and is_hallway(dst):
            #         continue
            #     if not is_hallway(dst) and is_correct_room(amphipod, dst) and dst % 2 == 1 and (
            #             amphipod, src, dst + 1) in moves:
            #         continue
            #     # can't move from hallway to hallway
            #     # can't move from hallway to room if room is not the correct one or
            #     # if there are amphipods of different kind in room
            #     if is_hallway(src) and not is_hallway(dst) and (
            #             not is_correct_room(amphipod, dst) or (dst + ((dst % 2) * 2 - 1)) in state):
            #         continue
            #     # can't move from room to same room
            #     if not is_hallway(src) and not is_hallway(dst) and src + 1 == dst and src % 2 == 1:
            #         continue
            #     # in the real input no amphipod starts in the correct room, meaning that if an amphipod is in the correct room,
            #     # it must have been moved there, and we don't want to move it back
            #     if is_correct_room(amphipod, src) and not is_correct_room(amphipod, dst):
            #         continue
            #     # print(f"moving {amphipod} from {src} to {dst}")
            #     state[amphipod] = dst
            #     min_cost = min(min_cost, dfs(cost + STEPS[src][dst] * (10 ** (amphipod // 2)), min_cost))
            #     state[amphipod] = src
        seen[number] = min_cost
        return min_cost

    seen = {}
    return dfs(0, 12872)


def part1(data: str):
    # 12348 too low
    # 12872 too high
    state = parse(data)
    print(state)
    return solve(state)


def part2(data: str):
    pass


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_23_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
