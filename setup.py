import os

content = """
def part1(data: str):
    pass


def part2(data: str):
    pass


if __name__ == '__main__':
    test = True
    # test = False
    test_input = ''''''
    if test:
        puzzle_input = test_input
    else:
        with open('day_{}_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
""".strip()


def setup_calendar(year: str):
    if not os.path.exists(year):
        os.mkdir(year)
    for day in range(1, 25 + 1):
        path = os.path.join(year, f'day_{day}.py')
        if not os.path.exists(path):
            with open(path, 'w') as f:
                f.write(content.format(day))
                f.write('\n')
        path = os.path.join(year, f'day_{day}_input.txt')
        if not os.path.exists(path):
            with open(path, 'w') as _f:
                pass
    # asyncio.run(download_inputs(year))


# async def download_inputs(year: str):
#     async with aiohttp.ClientSession() as session:
#         await asyncio.gather(
#             *[download_input(session, year, str(day)) for day in range(1, 25 + 1)],
#             return_exceptions = True
#         )
#
#
# async def download_input(session, year: str, day: str):
#     url = f'https://adventofcode.com/{year}/day/{day}/input'
#     async with session.get(url) as resp:
#         if resp.status == 200:
#             path = os.path.join(year, f'day_{day}_input.txt')
#             async with aiofile.async_open(path, 'w') as aiof:
#                 await aiof.write(await resp.text())
#         else:
#             print(f'Error downloading input for day {day}: {resp.status}')


if __name__ == '__main__':
    setup_calendar('2021')
