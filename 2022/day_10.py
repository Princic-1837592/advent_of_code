def part1(data: str):
    cycle = 1
    add = False
    instructions = data.splitlines()
    x = 1
    i = 0
    result = 0
    while i < len(instructions):
        if (cycle - 20) % 40 == 0:
            result += x * cycle
        instr = instructions[i]
        if add:
            add = False
            x += int(instr.split()[1])
            i += 1
        elif instr == "noop":
            i += 1
        elif instr.startswith("addx "):
            add = True
        cycle += 1
    return result


def part2(data: str, out_char: str = "#"):
    instructions = data.splitlines()
    crt = [[" " for _ in range(40)] for _ in range(6)]
    sprite = 0
    cycle = 0
    add = False
    i = 0
    while i < len(instructions):
        if sprite <= cycle % 40 <= sprite + 2:
            crt[cycle // 40][cycle % 40] = out_char
        instr = instructions[i]
        if add:
            add = False
            sprite += int(instr.split()[1])
            i += 1
        elif instr == "noop":
            i += 1
        elif instr.startswith("addx "):
            add = True
        cycle += 1
    return "\n".join("".join(row) for row in crt)


if __name__ == "__main__":
    test = True
    test = False
    test_input = """addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_10_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
