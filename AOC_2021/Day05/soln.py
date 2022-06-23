import re


movement_regex = re.compile(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")


def read_src():
    with open("src.txt", "r") as fh:
        content = fh.readlines()
    if DEBUG:
        content = """0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2""".split(
            "\n"
        )

    fog = [FogLine(line) for line in content]
    return fog


class FogLine:
    def __init__(self, str_def):
        self.coords = movement_regex.match(str_def.strip())

    def covered_points_no_diag(self):
        x1, y1, x2, y2 = self.start_end()

        if x1 == x2:
            if y1 > y2:
                y1, y2 = y2, y1
            for y in range(y1, y2):
                yield (x1, y)
            yield (x1, y2)

        elif y1 == y2:
            if x1 > x2:
                x1, x2 = x2, x1
            for x in range(x1, x2):
                yield (x, y1)
            yield (x2, y1)
        else:
            pass

    def covered_points(self):
        x1, y1, x2, y2 = self.start_end()

        if x1 == x2:
            if y1 > y2:
                y1, y2 = y2, y1
            for y in range(y1, y2):
                yield (x1, y)
            yield (x1, y2)

        elif y1 == y2:
            if x1 > x2:
                x1, x2 = x2, x1
            for x in range(x1, x2):
                yield (x, y1)
            yield (x2, y1)
        else:
            x_dir = 1 if x1 < x2 else -1
            y_dir = 1 if y1 < y2 else -1

            for x, y in zip(range(x1, x2, x_dir), range(y1, y2, y_dir)):
                yield (x, y)
            yield (x2, y2)

    def start_end(self):
        x1 = int(self.coords.group("x1"))
        y1 = int(self.coords.group("y1"))
        x2 = int(self.coords.group("x2"))
        y2 = int(self.coords.group("y2"))

        return x1, y1, x2, y2


def task_1():
    fogs = read_src()
    covered = {}

    for fog in fogs:
        for pt in fog.covered_points_no_diag():
            covered[pt] = covered.get(pt, 0) + 1

    print(f"task 1: {len([x for x in covered if covered[x] >= 2])}")
    debug_print(covered)


def task_2():
    fogs = read_src()
    covered = {}

    for fog in fogs:
        for pt in fog.covered_points():
            covered[pt] = covered.get(pt, 0) + 1

    print(f"task 2: {len([x for x in covered if covered[x] >= 2])}")
    debug_print(covered)


def debug_print(covered):
    if DEBUG:
        print()
        for y in range(10):
            line = ""
            for x in range(10):
                line += str(covered.get((x, y), "."))
            print(line)
        print()


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
