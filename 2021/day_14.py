from collections import Counter
from typing import Dict, List


def step(polymer: List[str], rules: Dict[str, str]) -> List[str]:
    result = [''] * (len(polymer) * 2 - 1)
    for i in range(len(polymer) - 1):
        result[i * 2] = polymer[i]
        result[i * 2 + 1] = rules[polymer[i] + polymer[i + 1]]
    result[-1] = polymer[-1]
    return result


def part1(data: str):
    start, rules = data.split('\n\n')
    rules = dict([rule.split(' -> ') for rule in rules.splitlines()])
    for _ in range(10):
        start = step(start, rules)
    repetitions = Counter(start)
    return repetitions[max(repetitions, key = repetitions.get)] - repetitions[min(repetitions, key = repetitions.get)]


def step_pairs(pairs: Dict[str, int], rules: Dict[str, str]):
    pass
    new_pairs = pairs.copy()
    for pair, occurrences in pairs.items():
        if occurrences > 0:
            new_pairs[pair[0] + rules[pair]] += occurrences
            new_pairs[rules[pair] + pair[1]] += occurrences
            new_pairs[pair] -= occurrences
    return new_pairs


def part2(data: str):
    start, rules = data.split('\n\n')
    rules = dict([rule.split(' -> ') for rule in rules.splitlines()])
    pairs = {k: 0 for k in rules.keys()}
    for i in range(len(start) - 1):
        pairs[start[i] + start[i + 1]] += 1
    for _ in range(40):
        pairs = step_pairs(pairs, rules)
    counts = {}
    for pair, occurrences in pairs.items():
        counts[pair[0]] = counts.get(pair[0], 0) + occurrences
    counts[start[-1]] = counts.get(start[-1], 0) + 1
    return counts[max(counts, key = counts.get)] - counts[min(counts, key = counts.get)]


if __name__ == '__main__':
    test = True
    test = False
    test_input = '''NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C'''
    if test:
        puzzle_input = test_input
    else:
        with open('day_14_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
