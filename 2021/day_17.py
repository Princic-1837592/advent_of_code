def parse(data: str):
    _, coords = data.split(" ", 1)
    x_str, y_str = coords.split(", ")
    x = tuple(map(int, x_str.split("=")[1].split("..")))
    y = tuple(map(int, y_str.split("=")[1].split("..")))
    return x, y


def part1(data: str):
    _x, y = parse(data)
    return sum([i for i in range(max(map(abs, y)))])


def part2(data: str):
    (left, right), (bottom, top) = parse(data)
    left, right = sorted((left, right))
    bottom, top = sorted((bottom, top))
    candidates = set()
    for n in range(right + 1):
        s = 0
        m = n
        while m >= 0:
            if left <= s <= right:
                candidates.add(n)
            s += m
            m -= 1
    depth = bottom
    count = 0
    for candidate in candidates:
        for y in range(-abs(depth), abs(depth)):
            coord = (0, 0)
            v_speed = y
            h_speed = candidate
            while coord[1] >= depth:
                coord = (coord[0] + h_speed, coord[1] + v_speed)
                v_speed -= 1
                if h_speed > 0:
                    h_speed -= 1
                if left <= coord[0] <= right and bottom <= coord[1] <= top:
                    count += 1
                    break
    return count


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """target area: x=20..30, y=-10..-5"""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_17_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
