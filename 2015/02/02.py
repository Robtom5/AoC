DEBUG = True

import re

present_regex = re.compile(r"^(?P<x>\d+)x(?P<y>\d+)x(?P<z>\d+)")


def parse_line(line):
    match = present_regex.match(line)
    if match is not None:
        x = int(match.group("x"))
        y = int(match.group("y"))
        z = int(match.group("z"))
        return (x, y, z)


def wrapping_paper_needed(dimension):
    xy, xz, yz = sides(dimension)
    min_sz = min([xy, xz, yz])
    return 2 * (xy + xz + yz) + min_sz


def sides(dimension):
    x, y, z = dimension
    xy = x * y
    xz = x * z
    yz = y * z
    return xy, xz, yz


def ribbon_needed(dimension):
    x, y, z = dimension
    dim_1, dim_2 = sorted([x, y, z])[0:2]
    return 2 * (dim_1 + dim_2) + (x * y * z)


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    dimensions = [parse_line(l) for l in content if parse_line(l)]

    return dimensions


def task_1():
    dimensions = read_src()
    total_wrapping_paper = sum([wrapping_paper_needed(d) for d in dimensions])
    print(f"task 1: {total_wrapping_paper}")


def task_2():
    dimensions = read_src()
    total_ribbon = sum([ribbon_needed(d) for d in dimensions])
    print(f"task 2: {total_ribbon}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
