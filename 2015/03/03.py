DEBUG = True

import operator


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    instructions = content[0]
    return instructions


def shift(base, delta):
    return tuple(map(operator.add, base, delta))


def move(instr, coords):
    if instr == "^":
        coord = shift(coords, (0, 1))
    elif instr == ">":
        coord = shift(coords, (1, 0))
    elif instr == "<":
        coord = shift(coords, (-1, 0))
    elif instr == "v":
        coord = shift(coords, (0, -1))
    else:
        raise Exception("Unknown Instruction")
    return coord


def task_1():
    instructions = read_src()
    visited_houses = set([(0, 0)])
    curr_loc = (0, 0)
    for ins in instructions:
        curr_loc = move(ins, curr_loc)
        visited_houses.add(curr_loc)

    print(f"task 1: {len(visited_houses)} houses")


def task_2():
    instructions = read_src()
    visited_houses = set([(0, 0)])
    curr_loc = (0, 0)
    robo_loc = (0, 0)
    robo_active = False
    for ins in instructions:
        if robo_active:
            robo_loc = move(ins, robo_loc)
            visited_houses.add(robo_loc)
        else:
            curr_loc = move(ins, curr_loc)
            visited_houses.add(curr_loc)
        robo_active = not robo_active

    print(f"task 2: {len(visited_houses)} houses")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
