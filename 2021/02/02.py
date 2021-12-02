def read_src():
    with open("02_a_src.txt", "r") as fh:
        content = fh.readlines()

    return content


def task_1():
    instructions = read_src()
    depth = 0
    horiz = 0

    for line in instructions:
        instr, mag, *_ = line.split(" ")
        mag = int(mag)
        if instr.startswith("forward"):
            horiz += mag
        elif instr.startswith("down"):
            depth += mag
        elif instr.startswith("up"):
            depth -= mag

    print(f"part a: depth: {depth} horiz: {horiz} product: {depth*horiz}")


def task_2():
    instructions = read_src()
    depth = 0
    horiz = 0
    aim = 0

    for line in instructions:
        instr, mag, *_ = line.split(" ")
        mag = int(mag)
        if instr.startswith("forward"):
            horiz += mag
            depth += mag * aim
        elif instr.startswith("down"):
            aim += mag
        elif instr.startswith("up"):
            aim -= mag

    print(f"part b: depth: {depth} horiz: {horiz} product: {depth*horiz}")


if __name__ == "__main__":
    task_1()
    task_2()
