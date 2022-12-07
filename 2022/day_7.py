from typing import List, Tuple


def parse_directory(commands: List[str], i: int) -> Tuple[int, dict]:
    cur_dir = {}
    while i < len(commands):
        command = commands[i]
        if command == "$ ls":
            i += 1
        elif command == "$ cd ..":
            return i + 1, cur_dir
        elif command.startswith("$ cd "):
            i, sub_dir = parse_directory(commands, i + 1)
            cur_dir[command[5:]] = sub_dir
        elif command.startswith("dir "):
            i += 1
        else:
            dim, name = command.split()
            cur_dir[name] = int(dim)
            i += 1
    return i, cur_dir


def count_weight(dirs: dict, at_most: int) -> Tuple[int, int]:
    local_total = 0
    local_count = 0
    for subdir in dirs:
        if isinstance(dirs[subdir], dict):
            total, count = count_weight(dirs[subdir], at_most)
            local_total += total
            local_count += count
        else:
            local_total += dirs[subdir]
    if local_total <= at_most:
        local_count += local_total
    return local_total, local_count


def part1(data: str):
    _, dirs = parse_directory(data.splitlines(), 0)
    return count_weight(dirs, 100000)[1]


def find_smallest(dirs: dict, at_least: int) -> Tuple[int, int]:
    local_total = 0
    local_smallest = 70000000
    for subdir in dirs:
        if isinstance(dirs[subdir], dict):
            total, smallest = find_smallest(dirs[subdir], at_least)
            local_total += total
            if smallest < local_smallest:
                local_smallest = smallest
        else:
            local_total += dirs[subdir]
    if local_smallest > local_total >= at_least:
        local_smallest = local_total
    return local_total, local_smallest


def part2(data: str):
    dirs = parse_directory(data.splitlines(), 0)[1]
    total_weight = count_weight(dirs, 100000)[0]
    free_space = 70000000 - total_weight
    return find_smallest(dirs, 30000000 - free_space)[1]


if __name__ == "__main__":
    test = True
    test = False
    test_input = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_7_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
