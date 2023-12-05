from configparser import ConfigParser
from datetime import date

from requests import get

today = date.today()
if today.month != 12:
    print("It's not December!")
    exit(1)
if today.day > 25:
    print("This year's Advent of Code is over!")
    exit(1)

config = ConfigParser()
config.read("local_config.ini")
session = config["auto_download"]["session"]
resp = get(
    f"https://adventofcode.com/{today.year}/day/{today.day}/input",
    cookies={"session": session},
    headers={
        "User-Agent": "https://github.com/Princic-1837592/advent_of_code/blob/main/setup.py"
                      " by princic.1837592@studenti.uniroma1.it"
    },
)
if resp.status_code != 200:
    print("Something went wrong with the request!")
    print(F"Error code: {resp.status_code}")
    print(resp.text)
    exit(1)
with open(f"{today.year}/inputs/day_{today.day:0>2}_input.txt", "w", encoding="utf-8") as file:
    if file.write(resp.text) > 0:
        print("Input downloaded successfully!")
