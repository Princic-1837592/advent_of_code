from queue import Queue
from re import match
from typing import Dict, List, Tuple


class Valve:
    def __init__(self, name: str, value: str | int, tunnels: str):
        self.name = name
        self.id = 0
        self.value = int(value)
        self.edges: Dict[int, int] = {target: 1 for target in tunnels.split(", ")} if tunnels else {}

    def __str__(self):
        return f"{self.name} {self.value} {self.edges}"

    def __repr__(self):
        return self.__str__()


Graph = Dict[int, Valve]


def bfs(graph: Dict[str, Valve], start: str) -> Dict[str, int]:
    queue = Queue()
    queue.put(start)
    visited = {start: 0}
    while not queue.empty():
        current = queue.get()
        for neighbour in graph[current].edges:
            if neighbour not in visited:
                visited[neighbour] = visited[current] + graph[current].edges[neighbour]
                queue.put(neighbour)
    visited.pop(start)
    return visited


def full_bfs(graph: Dict[str, Valve]) -> Dict[str, Dict[str, int]]:
    return {node: bfs(graph, node) for node in graph}


def make_graph(data: str):
    return {
        m.group(1): Valve(*m.groups()) for m in map(
            lambda l: match(r"Valve ([A-Z]{2}) .+?=(\d+); .+?valves? ((?:[A-Z]{2}(?:, )?)*)", l),
            data.splitlines()
        )
    }


def remove_0(graph, distances):
    to_remove = set()
    for name, node in graph.items():
        if node.value == 0 and name != "AA":
            to_remove.add(name)
    for name, edges in distances.items():
        for edge in edges:
            graph[name].edges[edge] = distances[name][edge]
    for removing in to_remove:
        graph.pop(removing)
        for name, node in graph.items():
            if removing in node.edges:
                node.edges.pop(removing)


def make_non_0_graph(data: str) -> Tuple[Graph, int]:
    graph = make_graph(data)
    distances = full_bfs(graph)
    remove_0(graph, distances)
    to_numbers = {key: i for i, key in enumerate(graph)}
    aa = 0
    for key, node in graph.copy().items():
        if key == "AA":
            aa = to_numbers[key]
        graph[to_numbers[key]] = graph.pop(key)
        node.id = to_numbers[key]
        for edge in node.edges.copy():
            node.edges[to_numbers[edge]] = node.edges.pop(edge)
    return graph, aa


def explore_dfs(global_states, graph, local_best, minutes_left, pressure, state_open, valve):
    for neighbour, minutes_to_go in graph[valve].edges.items():
        local_best = max(
            local_best,
            dfs(
                graph,
                neighbour,
                minutes_left - minutes_to_go,
                pressure,
                state_open,
                global_states
            )
        )
    return local_best


def dfs(
        graph: Graph,
        valve: int,
        minutes_left: int,
        pressure: int,
        state_open: List[bool],
        global_states: Dict[int, List[int]]
) -> int:
    if minutes_left < 0:
        return -1
    if global_states[valve][minutes_left] >= pressure:
        return -1
    global_states[valve][minutes_left] = pressure
    local_best = pressure
    # don't open
    local_best = explore_dfs(global_states, graph, local_best, minutes_left, pressure, state_open, valve)
    # open
    if not state_open[valve]:
        state_open[valve] = True
        minutes_left -= 1
        pressure += graph[valve].value * minutes_left
        local_best = explore_dfs(global_states, graph, local_best, minutes_left, pressure, state_open, valve)
        state_open[valve] = False
    return local_best


def part1(data: str):
    minutes = 30
    graph, aa = make_non_0_graph(data)
    state_open = [False for _ in range(len(graph))]
    global_states = {valve: [-1 for _ in range(minutes + 1)] for valve in graph}
    return dfs(graph, aa, minutes, 0, state_open, global_states)


def explore_elephant_dfs(
        e_minutes,
        e_valve,
        global_states,
        graph,
        h_minutes,
        h_valve,
        local_best,
        pressure,
        state_open
):
    for h_neighbour, h_minutes_to_go in graph[h_valve].edges.items():
        for e_neighbour, e_minutes_to_go in graph[e_valve].edges.items():
            local_best = max(
                local_best,
                elephant_dfs(
                    graph,
                    (h_neighbour, e_neighbour),
                    (h_minutes - h_minutes_to_go, e_minutes - e_minutes_to_go),
                    pressure,
                    state_open,
                    global_states
                )
            )
    return local_best


def elephant_dfs(
        graph: Graph,
        valves: Tuple[int, int],
        minutes_left: Tuple[int, int],
        pressure: int,
        state_open: List[bool],
        global_states: Dict[Tuple[int, int], Dict[Tuple[int, int], int]]
) -> int:
    h_minutes, e_minutes = minutes_left
    if h_minutes < 0 or e_minutes < 0:
        return -1
    h_valve, e_valve = valves
    if global_states[valves][minutes_left] >= pressure:
        return -1
    global_states[valves][minutes_left] = pressure
    local_best = pressure
    # don't open
    local_best = explore_elephant_dfs(
        e_minutes, e_valve, global_states, graph, h_minutes, h_valve, local_best, pressure, state_open
    )
    # open human
    if not state_open[h_valve]:
        state_open[h_valve] = True
        h_minutes -= 1
        pressure += graph[h_valve].value * h_minutes
        local_best = explore_elephant_dfs(
            e_minutes, e_valve, global_states, graph, h_minutes, h_valve, local_best, pressure, state_open
        )
        pressure -= graph[h_valve].value * h_minutes
        h_minutes += 1
        state_open[h_valve] = False
    if h_valve != e_valve:
        # open elephant
        if not state_open[e_valve]:
            state_open[e_valve] = True
            e_minutes -= 1
            pressure += graph[e_valve].value * e_minutes
            local_best = explore_elephant_dfs(
                e_minutes, e_valve, global_states, graph, h_minutes, h_valve, local_best, pressure, state_open
            )
            pressure -= graph[e_valve].value * e_minutes
            e_minutes += 1
            state_open[e_valve] = False
        # open both
        if not state_open[h_valve] and not state_open[e_valve]:
            state_open[h_valve] = True
            state_open[e_valve] = True
            h_minutes -= 1
            e_minutes -= 1
            pressure += graph[h_valve].value * h_minutes
            pressure += graph[e_valve].value * e_minutes
            local_best = explore_elephant_dfs(
                e_minutes, e_valve, global_states, graph, h_minutes, h_valve, local_best, pressure, state_open
            )
            state_open[e_valve] = False
            state_open[h_valve] = False
    return local_best


def part2(data: str):
    minutes = 26
    graph, aa = make_non_0_graph(data)
    state_open = [False for _ in range(len(graph))]
    global_states = {
        (hv, ev): {(hm, em): -1
                   for hm in range(minutes + 1)
                   for em in range(minutes + 1)}
        for hv in graph
        for ev in graph
    }
    return elephant_dfs(graph, (aa, aa), (minutes, minutes), 0, state_open, global_states)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II""".strip()
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_16_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
