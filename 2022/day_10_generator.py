ASCII = {
    "A": [
        " ## ",
        "#  #",
        "#  #",
        "####",
        "#  #",
        "#  #"
    ],
    "B": [
        "### ",
        "#  #",
        "### ",
        "### ",
        "#  #",
        "### "
    ],
    "C": [
        " ## ",
        "#  #",
        "#   ",
        "#   ",
        "#  #",
        " ## "
    ],
    "D": [
        "### ",
        "#  #",
        "#  #",
        "#  #",
        "#  #",
        "### "
    ],
    "E": [
        "####",
        "#   ",
        "### ",
        "#   ",
        "#   ",
        "####"
    ],
    "F": [
        "####",
        "#   ",
        "### ",
        "#   ",
        "#   ",
        "#   "
    ],
    "G": [
        " ## ",
        "#  #",
        "#   ",
        "# ##",
        "#  #",
        " ## "
    ],
    "H": [
        "#  #",
        "#  #",
        "####",
        "#  #",
        "#  #",
        "#  #"
    ],
    "I": [
        " #  ",
        " #  ",
        " #  ",
        " #  ",
        " #  ",
        " #  "
    ],
    "J": [
        "   #",
        "   #",
        "   #",
        "   #",
        "#  #",
        " ## "
    ],
    "K": [
        "#  #",
        "# # ",
        "##  ",
        "# # ",
        "#  #",
        "#  #"
    ],
    "L": [
        "#   ",
        "#   ",
        "#   ",
        "#   ",
        "#   ",
        "####"
    ],
    "N": [
        "#  #",
        "## #",
        "# ##",
        "#  #",
        "#  #",
        "#  #"
    ],
    "O": [
        " ## ",
        "#  #",
        "#  #",
        "#  #",
        "#  #",
        " ## "
    ],
    "P": [
        "### ",
        "#  #",
        "### ",
        "#   ",
        "#   ",
        "#   "
    ],
    "Q": [
        " ## ",
        "#  #",
        "#  #",
        "#  #",
        "# ##",
        " # #"
    ],
    "R": [
        "### ",
        "#  #",
        "### ",
        "# # ",
        "#  #",
        "#  #"
    ],
    "S": [
        " ## ",
        "#  #",
        " #  ",
        "  # ",
        "#  #",
        " ## "
    ],
    "T": [
        "####",
        " #  ",
        " #  ",
        " #  ",
        " #  ",
        " #  "
    ],
    "U": [
        "#  #",
        "#  #",
        "#  #",
        "#  #",
        "#  #",
        " ## "
    ],
    "Y": [
        "#  #",
        "#  #",
        " #  ",
        " #  ",
        " #  ",
        " #  "
    ],
    "Z": [
        "####",
        "   #",
        "  # ",
        " #  ",
        "#   ",
        "####"
    ],
    " ": [
        "    ",
        "    ",
        "    ",
        "    ",
        "    ",
        "    "
    ],
}


def check_validity(letters: str) -> int:
    if len(letters) == 40 * 6:
        return 3 if all(letter in "# " for letter in letters) else 0
    if not all(letter in ASCII for letter in letters) or len(letters) != 8:
        return 0
    if ASCII[letters[0]][0][:2] != "##":
        return 1
    return 2


def crt_encode(text: str, out_char: str = "#") -> str:
    encoded = list(map(lambda l: f"{' '.join(l)} ", zip(*[ASCII[letter] for letter in text])))
    return "\n".join(
        map(
            lambda line: "".join(map(lambda x: out_char if x != " " else x, line)),
            encoded
        )
    )


def generate(text: str, strict: bool = True) -> str:
    if len(text) != 8:
        text = "".join(map(lambda x: "#" if x != " " else x, text))
    valid = check_validity(text)
    if valid == 0 or (strict and valid == 1):
        raise ValueError("Invalid input: can't produce such result")
    # if valid == 1:
    #     print("Warning: the first letter won't be shown correctly")
    if valid == 3:
        lines = "".join(text[i:i + 40] for i in range(0, len(text), 40))
    else:
        lines = "".join(crt_encode(text).splitlines())
    instructions = []
    sprite = 0
    for cycle in range(2, 240 + 1, 2):
        next_pair = lines[cycle:cycle + 2]
        column = cycle % 40
        if next_pair == "##":
            instructions.append(f"addx {column - sprite}")
            sprite = column
        elif next_pair == "# ":
            instructions.append(f"addx {column - sprite - 2}")
            sprite = cycle % 40 - 2
        elif next_pair == " #":
            instructions.append(f"addx {column - sprite + 1}")
            sprite = column + 1
        else:  # "  "
            instructions.append(f"addx {column - sprite + 2}")
            sprite = column + 2
        if sprite >= 40:
            sprite -= int(instructions[-1].split()[-1])
            instructions[-1] = "addx -3"
            sprite -= 3
        if instructions[-1] == "addx 0":
            instructions.pop()
            instructions.append("noop")
            instructions.append("noop")
    return "\n".join(instructions)
