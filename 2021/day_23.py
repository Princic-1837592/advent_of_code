# https://adventofcode.com/2021/day/23
# https://adventofcode.com/2021/day/23/input
from collections import deque
from heapq import heappush, heappop
from typing import List, Tuple, Set


class State:
    def __init__(self, positions: int, amphipods: int, depth: int, graph: List[List[Tuple[int, int]]]):
        self.positions = [-1] * positions
        self.amphipods = [-1] * amphipods
        self.graph: List[List[Tuple[int, int]]] = graph
        self.distances: List[List[int]] = []
        self.depth = depth
        self.correct_rooms = [[room * depth + 7 + amphipod for amphipod in reversed(range(depth))] for room in range(4)]
        self.times_moved = [0] * amphipods
        self.cost = 0
        self.estimated = 0
        self.history = []
        self.max_steps = [0, 0, 0, 0]
        self.steps_per_room = [0, 0, 0, 0]

    def set_graph(self, graph: List[List[Tuple[int, int]]]):
        self.graph = graph

    def __compute_distances(self):
        def bfs(src: int) -> List[int]:
            visited = [False] * len(self.positions)
            distances = [-1] * len(self.positions)
            queue = deque([(src, 0)])
            while queue:
                node, distance = queue.popleft()
                if visited[node]:
                    continue
                distances[node] = distance
                visited[node] = True
                for neighbor, cost in self.graph[node]:
                    queue.append((neighbor, distance + cost))
            return distances

        self.distances = [bfs(i) for i in range(len(self.positions))]

    def finalize(self):
        self.__compute_distances()
        for amphipod, pos in enumerate(self.amphipods):
            self.estimated += self.move_cost(amphipod, pos, self.get_correct_rooms(amphipod)[-1])
        self.history.append(self.to_image())
        for room in range(4):
            for a in range(self.depth):
                amphipod = room * self.depth + a
                self.max_steps[room] += self.distances[self.amphipods[amphipod]][self.get_correct_rooms(amphipod)[-1]]
            self.max_steps[room] += (self.depth * (self.depth - 1)) // 2
            # controllare se ce n'è uno già nella stanza che prima deve uscire

    @staticmethod
    def is_hallway(pos: int) -> bool:
        return pos <= 6

    def is_correct_room(self, amphipod: int, pos: int) -> bool:
        return pos in self.correct_rooms[amphipod // self.depth]

    def unitary_move_cost(self, amphipod: int) -> int:
        return 10 ** (amphipod // self.depth)

    def move_cost(self, amphipod: int, src: int, dst: int) -> int:
        return self.distances[src][dst] * self.unitary_move_cost(amphipod)

    def is_solved(self) -> bool:
        return self.estimated == (1 + 10 + 100 + 1000) * ((self.depth - 1) * self.depth) // 2 and all(
            map(lambda a: self.is_correct_room(a, self.amphipods[a]), range(len(self.amphipods)))
        )

    def get_correct_rooms(self, amphipod: int) -> List[int]:
        return self.correct_rooms[amphipod // self.depth]

    def own_room_is_correctly_occupied(self, amphipod: int) -> bool:
        pos = self.amphipods[amphipod]
        return self.room_is_correctly_occupied(pos)

    def room_is_correctly_occupied(self, pos: int) -> bool:
        room = (pos - 7) // self.depth
        for d in range(self.depth):
            p = 7 + room * self.depth + d
            if self.positions[p] != -1 and not self.is_correct_room(self.positions[p], p):
                return False
        return True

    def possible_moves(self) -> List[Set[int]]:
        def bfs(src: int) -> Set[int]:
            visited = [False] * len(self.positions)
            reachable = set()
            queue = deque([src])
            while queue:
                node = queue.popleft()
                if visited[node]:
                    continue
                if node != src and self.positions[node] != -1:
                    continue
                reachable.add(node)
                visited[node] = True
                for neighbor, _ in self.graph[node]:
                    queue.append(neighbor)
            reachable.remove(src)
            return reachable

        return [bfs(i) for i in range(len(self.positions))]

    def copy(self):
        copy = State(len(self.positions), len(self.amphipods), self.depth, self.graph)
        copy.positions = self.positions.copy()
        copy.amphipods = self.amphipods.copy()
        copy.distances = self.distances
        copy.correct_rooms = self.correct_rooms
        copy.times_moved = self.times_moved.copy()
        copy.cost = self.cost
        return copy

    def move(self, amphipod: int, src: int, dst: int) -> "State":
        next_state = self.copy()
        next_state.positions[src] = -1
        next_state.positions[dst] = amphipod
        next_state.amphipods[amphipod] = dst
        next_state.steps_per_room = self.steps_per_room.copy()
        next_state.times_moved[amphipod] += 1
        next_state.steps_per_room[amphipod // self.depth] += 1
        next_state.cost += self.move_cost(amphipod, src, dst)
        next_state.estimated = self.estimated
        next_state.estimated -= self.move_cost(amphipod, src, self.get_correct_rooms(amphipod)[-1])
        next_state.estimated += self.move_cost(amphipod, dst, self.get_correct_rooms(amphipod)[-1])
        next_state.history = self.history.copy()
        next_state.history.append(next_state.to_image())
        next_state.max_steps = self.max_steps
        return next_state

    def exceeded_max_steps(self) -> bool:
        for room in range(4):
            if room < 2:
                continue
            if self.steps_per_room[room] > self.max_steps[room] + 6:
                return True
        return False

    def to_number(self) -> int:
        result = 0
        for i in self.amphipods:
            result *= 100
            result += i
        return result

    def __lt__(self, other):
        return self.cost + self.estimated < other.cost + other.estimated

    def __le__(self, other):
        return self.cost + self.estimated <= other.cost + other.estimated

    def __str__(self):
        return f"State({self.positions}, {self.amphipods})"

    def to_image(self):
        image = [list("#############"), ["#"]]
        positions = [0, 1, -1, 2, -1, 3, -1, 4, -1, 5, 6]
        for i in range(11):
            if positions[i] != -1 and (amphipod := self.positions[positions[i]]) != -1:
                image[-1].append(chr(amphipod // self.depth + ord("A")))
            else:
                image[-1].append(".")
        image[-1].append("#")
        for d in range(self.depth):
            image.append(list("##"))
            for room in range(4):
                image[-1].append("#")
                if (amphipod := self.positions[room * self.depth + d + 7]) != -1:
                    image[-1].append(chr(amphipod // self.depth + ord("A")))
                else:
                    image[-1].append(".")
            image[-1].extend(list("###"))
        image.append(list("#############"))
        return "\n".join(map(lambda l: "".join(map(str, l)), image))


NEIGHBORS = ((0, 1), (0, -1), (1, 0), (-1, 0))


def parse(data: str) -> State:
    lines = data.splitlines()
    depth = len(lines) - 3
    positions = 7 + depth * 4
    state = State(
        positions,
        depth * 4,
        depth,
        [
            [(1, 1)],
            [(0, 1), (2, 2)],
            [(1, 2), (3, 2)],
            [(2, 2), (4, 2)],
            [(3, 2), (5, 2)],
            [(4, 2), (6, 1)],
            [(5, 1)]
        ]
    )
    for room in range(4):
        room_index = room * depth + 7
        state.graph.append([(room + 1, 2), (room + 2, 2)])
        state.graph[room + 1].append((room_index, 2))
        state.graph[room + 2].append((room_index, 2))
        for row in range(depth):
            position_index = room_index + row
            input_i = row + 2
            input_j = room * 2 + 3
            amphipod_type = ord(lines[input_i][input_j]) - ord('A')
            amphipod_index = amphipod_type * depth
            while state.amphipods[amphipod_index] != -1:
                amphipod_index += 1
            state.positions[position_index] = amphipod_index
            state.amphipods[amphipod_index] = position_index
            if row > 0:
                state.graph.append([(position_index - 1, 1)])
                state.graph[position_index - 1].append((position_index, 1))
    state.finalize()
    return state


def solve_bfs(initial_state: State):
    queue: List[State] = []
    heappush(queue, initial_state)
    seen = {}
    min_solution = initial_state.estimated * 2
    while queue:
        state = heappop(queue)
        number = state.to_number()
        if state.cost >= seen.get(number, min_solution):
            continue
        if state.cost >= min_solution:
            continue
        # if state.exceeded_max_steps():
        #     continue
        if state.is_solved():
            for old_state in state.history:
                print(old_state)
                print()
            return state.cost
        seen[number] = state.cost
        reachable = state.possible_moves()
        for amphipod, src in enumerate(state.amphipods):
            if state.times_moved[amphipod] >= 2:
                continue
            if state.is_hallway(src):
                for room in state.get_correct_rooms(amphipod):
                    if room in reachable[src] and state.room_is_correctly_occupied(room):
                        next_state = state.move(amphipod, src, room)
                        if next_state.cost < min_solution and next_state.cost < seen.get(
                                next_state.to_number(),
                                min_solution
                        ):
                            heappush(queue, next_state)
            else:
                if not state.is_correct_room(amphipod, src):
                    for dst in state.possible_moves()[src]:
                        if state.is_hallway(dst):
                            next_state = state.move(amphipod, src, dst)
                            if next_state.cost < min_solution and next_state.cost < seen.get(
                                    next_state.to_number(),
                                    min_solution
                            ):
                                heappush(queue, next_state)
                else:
                    if not state.own_room_is_correctly_occupied(amphipod):
                        for dst in state.possible_moves()[src]:
                            if state.is_hallway(dst):
                                next_state = state.move(amphipod, src, dst)
                                if next_state.cost < min_solution and next_state.cost < seen.get(
                                        next_state.to_number(),
                                        min_solution
                                ):
                                    heappush(queue, next_state)


def part1(data: str):
    state = parse(data)
    return solve_bfs(state)


def part2(data: str):
    lines = data.splitlines()
    lines.insert(3, "  #D#C#B#A#")
    lines.insert(4, "  #D#B#A#C#")
    data = "\n".join(lines)
    return part1(data)


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
