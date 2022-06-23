DEBUG = True

import re

line_regex = re.compile(
    r"(?P<min>\d+)-(?P<max>\d+) (?P<character>\D): (?P<password>\D+)"
)


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()

    valid = 0
    for line in content:
        result = line_regex.match(line)
        minOccurences = int(result.group("min"))
        maxOccurences = int(result.group("max"))
        character = result.group("character")
        password = result.group("password")
        occurences = password.count(character)
        if minOccurences <= occurences <= maxOccurences:
            valid += 1

    print(f"task 1: {valid}")


def task_2():
    content = read_src()

    valid = 0
    for line in content:
        result = line_regex.match(line)
        firstIndex = int(result.group("min")) - 1
        secondIndex = int(result.group("max")) - 1
        character = result.group("character")
        password = result.group("password")
        presentAtFirst = password[firstIndex] == character
        presentAtSecond = password[secondIndex] == character
        if presentAtFirst ^ presentAtSecond:
            valid += 1

    print(f"task 2: {valid}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
