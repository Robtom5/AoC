DEBUG = True

import re
import operator

direction_regex = re.compile(r"(?P<dir>[UDLR])(?P<dist>\d+)")


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    wires = []
    for line in content:
        if line is not "":
            instrs = direction_regex.findall(line)
            last_pt = (0, 0)
            covered_pts = set()
            dists = {}
            length = 0
            for instr in instrs:
                last_pt, length = pts_from_dir(
                    last_pt, instr, covered_pts, dists, length
                )
            covered_pts.remove((0, 0))
            wires.append((covered_pts, dists))
    return wires


def pts_between_ends(start, end, pts, dists, length):
    x1, y1 = start
    x2, y2 = end

    if x1 != x2:
        if x1 > x2:
            direction = -1
        else:
            direction = 1
        for x in range(x1, x2, direction):
            dists[(x, y1)] = length
            pts.add((x, y1))
            length += 1
    else:
        if y1 > y2:
            direction = -1
        else:
            direction = 1

        for y in range(y1, y2, direction):
            dists[(x1, y)] = length
            pts.add((x1, y))
            length += 1

    if direction == -1:
        pts.add((x2, y2))
        dists[(x2, y2)] = length
        length += 1


def pts_from_dir(start, instr, pts, dists, length):
    direction, distance = instr
    distance = int(distance)

    if direction == "U":
        delta = (0, distance)
    elif direction == "R":
        delta = (distance, 0)
    elif direction == "D":
        delta = (0, -distance)
    elif direction == "L":
        delta = (-distance, 0)

    end = tuple(map(operator.add, start, delta))
    pts_between_ends(start, end, pts, dists, length)
    return end, length + distance


def manhatten_dist(pt):
    return abs(pt[0]) + abs(pt[1])


def print_map(wire1, wire2):
    all_pts = wire1.union(wire2)
    intersections = wire1.intersection(wire2)
    x_max = max(all_pts, key=lambda x: x[0])[0]
    x_min = min(all_pts, key=lambda x: x[0])[0]
    y_max = max(all_pts, key=lambda x: x[1])[1]
    y_min = min(all_pts, key=lambda x: x[1])[1]

    for y in range(y_max, y_min - 1, -1):
        for x in range(x_min, x_max + 1):
            if (x, y) in intersections:
                print("o", end="")
            elif (x, y) in all_pts:
                print("+", end="")
            else:
                print(".", end="")
        print("")


def task_1():
    wires = read_src()
    wire1, wire2, *_ = wires
    wire1, _ = wire1
    wire2, _ = wire2

    # print_map(wire1, wire2)

    intersections = wire1.intersection(wire2)
    min_dist = min(map(manhatten_dist, intersections))
    print(f"task 1: {min_dist}")


def task_2():
    wires = read_src()
    wire1, wire2, *_ = wires
    wire1, wire1_dist = wire1
    wire2, wire2_dist = wire2

    intersections = wire1.intersection(wire2)

    def dist_down_wire(pt):
        return wire1_dist[pt] + wire2_dist[pt]

    min_dist = min(map(dist_down_wire, intersections))
    print(f"task 2: {min_dist}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
