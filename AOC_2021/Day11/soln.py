DEBUG = True

import operator
import time


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    width = len(content[0].strip())
    height = len(content)

    octopus_grid = OctopusGrid(width, height)
    for y in range(height):

        for x in range(width):
            octopus = Octopus(x, y, int(content[y][x]))

            octopus_grid.set_octopus(octopus, x, y)

    return octopus_grid


class OctopusGrid:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.grid = {}

    def print(self):

        for row in range(self.height):
            for col in range(self.width):
                print(self.grid[col, row], end=" ")
            print()

    def set_octopus(self, octopus, x, y):
        self.grid[(x, y)] = octopus

    def init_octopi(self):
        for _, o in self.grid.items():
            o.configure_adjacent(self.grid)

    def time_step(self):
        for _, o in self.grid.items():
            o.time_step()

    def flash(self):
        for _, o in self.grid.items():
            o.flash(self.grid)

    def clean(self):
        for _, o in self.grid.items():
            o.cleanup()

    def total_flashes(self):
        return sum([o.flash_count for _, o in self.grid.items()])


class Octopus:
    energy_max = 9

    def __init__(self, x, y, energy):
        self.coords = (x, y)
        self.energy = energy
        self.has_flashed = False
        self.adjacent = []
        self.flash_count = 0

    def time_step(self):
        self.energy += 1

    def __repr__(self):
        return f"\u001b[38;5;{236+(2*self.energy)}m{str(self.energy)}\033[0m'"

    def configure_adjacent(self, octopus_dict):
        for index in self.adjacent_indices():
            nearby_oct = octopus_dict.get(index, None)
            if nearby_oct is None:
                continue
            else:
                self.adjacent.append(nearby_oct)

    def flash(self, octopus_dict):
        if self.has_flashed:
            return

        if self.energy > Octopus.energy_max:
            self.has_flashed = True

            for octopus in self.adjacent:
                octopus.hit_by_flash(octopus_dict)

    def cleanup(self):
        if self.has_flashed:
            self.has_flashed = False
            self.energy = 0
            self.flash_count += 1

    def hit_by_flash(self, octopus_dict):
        self.energy += 1
        self.flash(octopus_dict)

    def adjacent_indices(self):
        ul = tuple(map(operator.add, self.coords, (-1, -1)))
        uc = tuple(map(operator.add, self.coords, (0, -1)))
        ur = tuple(map(operator.add, self.coords, (1, -1)))
        cl = tuple(map(operator.add, self.coords, (-1, 0)))
        cr = tuple(map(operator.add, self.coords, (1, 0)))
        dl = tuple(map(operator.add, self.coords, (-1, 1)))
        dc = tuple(map(operator.add, self.coords, (0, 1)))
        dr = tuple(map(operator.add, self.coords, (1, 1)))

        return [ul, uc, ur, cl, cr, dl, dc, dr]


def task_1():
    grid = read_src()
    grid.init_octopi()
    grid.print()
    for gen in range(100):
        grid.time_step()
        grid.flash()
        grid.clean()
        print(f"\033[{grid.height+1}F")
        grid.print()

        time.sleep(0.02)
    print(f"task 1: {grid.total_flashes()}")


def task_2():
    content = read_src()

    grid = read_src()
    grid.init_octopi()
    grid.print()
    gen = 0
    while gen < 5000:
        gen += 1
        grid.time_step()
        grid.flash()
        if all(o.has_flashed for _, o in grid.grid.items()):
            break
        grid.clean()
        print(f"\033[{grid.height+1}F")
        grid.print()
        time.sleep(0.02)

    else:
        gen = "Never all flashed"

        # time.sleep(0.01)
    print(f"\033[{grid.height}E")
    print(f"task 2: {gen}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
