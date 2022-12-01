def part1(data: str):
    digits = sum(map(lambda l: l.split(" | ")[1].split(), data.splitlines()), [])
    return len(list(filter(lambda d: len(d) in [2, 3, 4, 7], digits)))


def to_int(digit: str):
    digit = "".join(sorted(digit))
    return str(
        [
            "abcefg",
            "cf",
            "acdeg",
            "acdfg",
            "bcdf",
            "abdfg",
            "abdefg",
            "acf",
            "abcdefg",
            "abcdfg"
        ].index(digit)
    )


def make_mapping(mapping, digits) -> int:
    mapped = list(map(lambda d: "".join(map(mapping.get, d)), digits))
    # print(mapping)
    # print(mapped)
    return int("".join(map(to_int, mapped)))


def find_mapping(digits):
    one = next(filter(lambda d: len(d) == 2, digits))
    four = next(filter(lambda d: len(d) == 4, digits))
    seven = next(filter(lambda d: len(d) == 3, digits))
    eight = next(filter(lambda d: len(d) == 7, digits))
    segment_a = (set(seven) - set(one)).pop()
    mapping = {segment_a: "a"}
    six = next(filter(lambda d: len(d) == 6 and segment_a in d and ((one[0] in d) != (one[1] in d)), digits))
    mapping[one[0 if one[0] in six else 1]] = "f"
    mapping[one[1 if one[0] in six else 0]] = "c"
    four_on_seven = set(four + seven)
    nine = next(filter(lambda d: len(d) == 6 and len(set(d) & four_on_seven) == 5, digits))
    segment_g = (set(nine) - four_on_seven).pop()
    mapping[segment_g] = "g"
    seven_on_g = set(seven + segment_g)
    three = next(filter(lambda d: len(d) == 5 and len(set(d) & seven_on_g) == 4, digits))
    segment_d = (set(three) - seven_on_g).pop()
    mapping[segment_d] = "d"
    segment_b = (set(four) - set(three)).pop()
    mapping[segment_b] = "b"
    mapping[(set(eight) - set(nine)).pop()] = "e"
    return mapping


def part2(data: str):
    entries = list(map(lambda l: tuple(map(str.split, l.split(" | "))), data.splitlines()))
    mappings = list(map(lambda e: find_mapping(e[0]), entries))
    return sum(map(lambda pair: make_mapping(mappings[pair[0]], pair[1][1]), enumerate(entries)))


if __name__ == "__main__":
    test = True
    test = False
    test_input = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_8_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
