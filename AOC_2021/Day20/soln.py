DEBUG = True

import numpy as np


def parse_raw_grid_cell(cell):
    if cell == "#":
        return 1
    elif cell == ".":
        return 0
    else:
        raise Exception(f"Invalid Cell: {cell}")


class Algorithm:
    def __init__(self, raw):
        self.raw = raw
        self.alg = [int(x == "#") for x in self.raw]

    def parse_binary(self, binary):
        target_index = self.binary_to_int(binary)
        return self.alg[target_index]

    @classmethod
    def binary_to_int(cls, binary):
        return int(binary, 2)


class Grid:
    def __init__(self):
        # Can use set instead of dict
        self.cells = set()

    def copy(self):
        cop = Grid()
        for coord in self.cells:
            cop.add_value(coord)
        return cop

    def add_raw(self, coords, raw_value):
        if parse_raw_grid_cell(raw_value):
            self.add_value(coords)

    def add_value(self, coords):
        self.cells.add(coords)

    def print(self):
        minx, maxx, miny, maxy = self.bounds()
        for y in range(miny, maxy + 1):
            for x in range(minx, maxx + 1):
                print(self.value_at((x, y)), end="")
            print("")

    def bounds(self):
        minx, maxx, miny, maxy = 0, 0, 0, 0
        for x, y in self.cells:
            minx = min(x, minx)
            maxx = max(x, maxx)
            miny = min(y, miny)
            maxy = max(y, maxy)
        return minx, maxx, miny, maxy

    def value_at(self, coords):
        return "#" if coords in self.cells else "."

    def binary_at(self, coords):
        return "1" if coords in self.cells else "0"

    def window_for(self, coords):
        x, y = coords
        binary_string = ""
        for y_ in range(y - 1, y + 2):
            for x_ in range(x - 1, x + 2):
                binary_string += self.binary_at((x_, y_))
        return binary_string

    def apply_algorithm(self, algorithm):
        minx, maxx, miny, maxy = self.bounds()
        next_grid = Grid()
        safety = 10
        limit = 1

        for y in range(miny - safety, maxy + safety):
            for x in range(minx - safety, maxx + safety):
                binary = self.window_for((x, y))
                lit = algorithm.parse_binary(binary)
                if lit:
                    if (
                        x > minx - safety + limit
                        and x < maxx + safety - limit
                        and y > miny - safety + limit
                        and y < maxy + safety - limit
                    ):
                        next_grid.add_value((x, y))

        return next_grid

    def strip(self, depth):
        minx, maxx, miny, maxy = self.bounds()
        x_range = [i for i in range(minx, minx + depth)] + [
            j for j in range(maxx - depth, maxx + 1)
        ]
        y_range = [i for i in range(miny, miny + depth)] + [
            j for j in range(maxy - depth, maxy + 1)
        ]
        for y in range(miny, maxy + 1):
            for x in x_range:
                self.cells.discard((x, y))

        for y in y_range:
            for x in range(minx, maxx + 1):
                self.cells.discard((x, y))


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    algorithm_raw = content[0]
    algorithm = Algorithm(algorithm_raw)

    grid_raw = content[2:]

    grid_width = len(grid_raw[0])
    grid_height = len(grid_raw)

    grid = Grid()
    for y in range(grid_height):
        for x in range(grid_width):
            grid.add_raw((x, y), grid_raw[y][x])

    return algorithm, grid


def task_1():
    alg, grid = read_src()
    grid2 = grid.apply_algorithm(alg)
    grid3 = grid2.apply_algorithm(alg)

    # Infinite grid has issue with all unlit becoming lit
    # Then they toggle off again
    # Issue
    # Brute force way
    grid3.strip(10)
    # grid3.print()
    print(f"task 1:{len(grid3.cells)}")


def task_2():
    # To remove the brute force. Need to check if the algorithm will
    # cause empty cells to toggle. then if so, need to make sure that
    # every other application of algorithm understands that all values
    # outside of the image bounds will be lit

    alg, grid = read_src()
    for x in range(50):
        grid = grid.apply_algorithm(alg)

        if x % 2 == 1:
            # Brute force
            grid.strip(10)

    print(f"task 2:{len(grid.cells)}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
