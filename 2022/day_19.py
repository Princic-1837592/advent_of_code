import re
from typing import List


class Cost:
    def __init__(self, ore: int = 0, clay: int = 0, obsidian: int = 0):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian

    def __str__(self):
        return f"ore: {self.ore}, clay: {self.clay}, obsidian: {self.obsidian}"

    def __hash__(self):
        return hash((self.ore, self.clay, self.obsidian))

    def __eq__(self, other):
        return self.ore == other.ore and self.clay == other.clay and self.obsidian == other.obsidian


class Resources:
    def __init__(
            self, ore: int = 0, clay: int = 0, obsidian: int = 0, geode: int = 0, ore_robots: int = 1,
            clay_robots: int = 0, obsidian_robots: int = 0, geode_robots: int = 0
    ):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode
        self.ore_robots = ore_robots
        self.clay_robots = clay_robots
        self.obsidian_robots = obsidian_robots
        self.geode_robots = geode_robots

    def produce(self):
        return Resources(
            self.ore + self.ore_robots, self.clay + self.clay_robots, self.obsidian + self.obsidian_robots,
            self.geode + self.geode_robots, self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots
        )

    def __str__(self):
        return f"ore: {self.ore}, clay: {self.clay}, obsidian: {self.obsidian}, geode: {self.geode}, " \
               f"ore_robots: {self.ore_robots}, clay_robots: {self.clay_robots}, " \
               f"obsidian_robots: {self.obsidian_robots}, geode_robots: {self.geode_robots}"

    def __hash__(self):
        return hash(
            (self.ore, self.clay, self.obsidian, self.geode, self.ore_robots, self.clay_robots,
             self.obsidian_robots, self.geode_robots)
        )

    def __eq__(self, other):
        return (
                self.ore == other.ore and self.clay == other.clay and self.obsidian == other.obsidian and
                self.geode == other.geode and self.ore_robots == other.ore_robots and
                self.clay_robots == other.clay_robots and self.obsidian_robots == other.obsidian_robots and
                self.geode_robots == other.geode_robots
        )

    def __ge__(self, cost: Cost):
        return self.ore >= cost.ore and self.clay >= cost.clay and self.obsidian >= cost.obsidian

    def __iter__(self):
        yield self.ore
        yield self.clay
        yield self.obsidian
        yield self.geode

    def __isub__(self, other: "Cost"):
        self.ore -= other.ore
        self.clay -= other.clay
        self.obsidian -= other.obsidian
        return self


class Blueprint:
    def __init__(
            self, ore: Cost = Cost(0, 0, 0), clay: Cost = Cost(0, 0, 0), obsidian: Cost = Cost(0, 0, 0),
            geode: Cost = Cost(0, 0, 0)
    ):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode
        self.max_ore = 0
        self.max_clay = 0
        self.max_obsidian = 0
        self.finalize()

    def finalize(self):
        self.max_ore = max(map(lambda p: p.ore, self))
        self.max_clay = max(map(lambda p: p.clay, self))
        self.max_obsidian = max(map(lambda p: p.obsidian, self))

    def __str__(self):
        return f"ore: ({self.ore}), clay: ({self.clay}), obsidian: ({self.obsidian}), geode: ({self.geode})"

    def __hash__(self):
        return hash((self.ore, self.clay, self.obsidian, self.geode))

    def __eq__(self, other):
        return (
                self.ore == other.ore and self.clay == other.clay and
                self.obsidian == other.obsidian and self.geode == other.geode
        )

    def __iter__(self):
        yield self.ore
        yield self.clay
        yield self.obsidian
        yield self.geode


def parse(data: str) -> List[Blueprint]:
    lines = data.splitlines()
    blueprints = [Blueprint() for _ in range(len(lines))]
    regex = re.compile(r"\d+")
    for i, line in enumerate(lines):
        ore, clay, obsidian, geode = line.split(". ")
        ore_robot = regex.findall(ore)
        blueprints[i].ore = Cost(ore=int(ore_robot[1]))
        clay_robot = regex.findall(clay)
        blueprints[i].clay = Cost(ore=int(clay_robot[0]))
        obsidian_robot = regex.findall(obsidian)
        blueprints[i].obsidian = Cost(ore=int(obsidian_robot[0]), clay=int(obsidian_robot[1]))
        geode_robot = regex.findall(geode)
        blueprints[i].geode = Cost(ore=int(geode_robot[0]), obsidian=int(geode_robot[1]))
        blueprints[i].finalize()
    return blueprints


def compute_geodes(blueprint: Blueprint, tot_minutes: int) -> int:
    def explore(
            minutes: int,
            global_max: int,
            resources: Resources
    ):
        if (minutes, resources) in seen:
            return seen[(minutes, resources)]
        if minutes == 0:
            return resources.geode
        if resources.geode + resources.geode_robots * minutes + (minutes * (minutes + 1)) // 2 <= global_max:
            return global_max
        # build geode robot, no other choices
        if resources >= blueprint.geode:
            new_resources = resources.produce()
            new_resources -= blueprint.geode
            new_resources.geode_robots += 1
            return explore(minutes - 1, max(global_max, resources.geode), new_resources)
        # build obsidian robot
        if resources >= blueprint.obsidian and resources.geode_robots < blueprint.max_obsidian:
            new_resources = resources.produce()
            new_resources -= blueprint.obsidian
            new_resources.obsidian_robots += 1
            global_max = max(global_max, explore(minutes - 1, global_max, new_resources))
            seen[(minutes, new_resources)] = global_max
        # build clay robot
        if resources >= blueprint.clay and resources.clay_robots < blueprint.max_clay:
            new_resources = resources.produce()
            new_resources -= blueprint.clay
            new_resources.clay_robots += 1
            global_max = max(global_max, explore(minutes - 1, global_max, new_resources))
            seen[(minutes, new_resources)] = global_max
        # build ore robot
        if resources >= blueprint.ore and resources.ore_robots < blueprint.max_ore:
            new_resources = resources.produce()
            new_resources -= blueprint.ore
            new_resources.ore_robots += 1
            global_max = max(global_max, explore(minutes - 1, global_max, new_resources))
            seen[(minutes, new_resources)] = global_max
        # produce
        new_resources = resources.produce()
        global_max = max(global_max, explore(minutes - 1, global_max, new_resources))
        seen[(minutes, new_resources)] = global_max
        return global_max

    seen = {}
    return explore(tot_minutes, 0, Resources())


def part1(data: str):
    blueprints = parse(data)
    return sum(map(lambda i_print: compute_geodes(i_print[1], 24) * (i_print[0] + 1), enumerate(blueprints)))


def part2(data: str):
    blueprints = parse(data)[:3]
    result = 1
    for blueprint in blueprints:
        result *= compute_geodes(blueprint, 32)
    return result


if __name__ == "__main__":
    # test = True
    test = False
    test_input = """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 
    ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. 
Each geode robot costs 3 ore and 12 obsidian."""
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_19_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
