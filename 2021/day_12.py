from typing import Dict, List


def extract_paths(lines: List[str]) -> Dict[str, List[str]]:
    caves = {}
    for line in lines:
        start, end = line.split("-")
        if start not in caves:
            caves[start] = [end]
        else:
            caves[start].append(end)
        if end not in caves:
            caves[end] = [start]
        else:
            caves[end].append(start)
    return caves


def visit(
        caves: Dict[str, List[str]],
        cave: str,
        visited: List[str],
        used_extra_cave: bool
) -> int:
    if cave in visited:
        if used_extra_cave or cave == "start":
            return 0
        else:
            used_extra_cave = True
    if cave == "end":
        return 1
    paths = 0
    if cave.islower():
        visited.append(cave)
    for dest in caves.get(cave, []):
        paths += visit(caves, dest, visited, used_extra_cave)
    if cave.islower():
        visited.remove(cave)
    return paths


def part1(data: str):
    lines = data.splitlines()
    caves = extract_paths(lines)
    return visit(caves, "start", [], True)


def part2(data: str):
    lines = data.splitlines()
    caves = extract_paths(lines)
    return visit(caves, "start", [], False)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_12_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
