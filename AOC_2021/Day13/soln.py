DEBUG = True

import re

fold_regex = re.compile(r"^fold along (?P<axis>[xy])=(?P<val>\d+)$")
coord_regex = re.compile(r"^(?P<x>\d+),(?P<y>\d+)$")


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    load_coords = True
    coords = set()
    folds = []
    for line in content:
        line = line.strip()
        if line == "":
            load_coords = False
            continue

        if load_coords:
            coord_match = coord_regex.match(line)
            coords.add((int(coord_match.group("x")), int(coord_match.group("y"))))
        else:
            fold_match = fold_regex.match(line)
            folds.append(
                (fold_match.group("axis") == "y", int(fold_match.group("val")))
            )
    return coords, folds


def apply_fold(pt, fold):
    horiz, val = fold
    x, y = pt
    if horiz and y > val:
        y = 2 * val - y
    elif not horiz and x > val:
        x = 2 * val - x
    return (x, y)


def print_grid(coords):
    width = 0
    height = 0
    for x, y in coords:
        width = max(width, x)
        height = max(height, y)

    for y in range(height + 1):
        for x in range(width + 1):
            char = "."
            if (x, y) in coords:
                char = "#"
            print(char, end="")
        print()


def task_1():
    coords, folds = read_src()

    coords = {apply_fold(pt, folds[0]) for pt in coords}

    print(f"task 1: {len(coords)}")


def task_2():
    coords, folds = read_src()

    for fold in folds:
        coords = {apply_fold(pt, fold) for pt in coords}
    print(f"task 2: ")
    print_grid(coords)


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
