DEBUG = True

import re

target_area_regex = re.compile(
    r"^target area: x=(?P<minx>-?\d+)..(?P<maxx>-?\d+), y=(?P<miny>-?\d+)..(?P<maxy>-?\d+)"
)

# r"^target area: x=(?P<minx>-?\d+)..(?P<maxx>-?\d+), y=(?P<miny>-?\d+)..(?P<maxy>-?\d+)"
class Probe:
    def __init__(self, coords, velocities):
        self.x, self.y = coords
        self.dx, self.dy = velocities
        self.zenith = 0

    def step(self):
        self.x += self.dx
        self.y += self.dy
        if self.dx != 0:
            self.dx -= self.dx / abs(self.dx)
        self.dy -= 1
        if self.y > self.zenith:
            self.zenith = self.y

    def hit(self, x_bounds, y_bounds):
        minx, maxx = x_bounds
        miny, maxy = y_bounds

        return self.x <= maxx and self.x >= minx and self.y <= maxy and self.y >= miny

    def missed(self, x_bounds, y_bounds):
        if self.y < y_bounds[0]:
            return True
        else:
            return False


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()
    target_match = target_area_regex.match(content[0].strip())

    minx = int(target_match.group("minx"))
    maxx = int(target_match.group("maxx"))
    miny = int(target_match.group("miny"))
    maxy = int(target_match.group("maxy"))

    return (minx, maxx), (miny, maxy)


def task_1():
    x_bounds, y_bounds = read_src()

    best_y = 0

    starting_dy = -y_bounds[0]

    while best_y == 0:
        probe = Probe((0, 0), (0, starting_dy))
        while not probe.missed((-1, 1), y_bounds):
            probe.step()
            if probe.hit((-1, 1), y_bounds):
                best_y = probe.zenith
                break
        starting_dy -= 1

    print(f"task 1: {best_y}")


def task_2():
    x_bounds, y_bounds = read_src()

    valid_launch = []
    for starting_dy in range(y_bounds[0], -y_bounds[0]):
        for starting_dx in range(x_bounds[1] + 1):
            probe = Probe((0, 0), (starting_dx, starting_dy))
            while not probe.missed(x_bounds, y_bounds):
                probe.step()
                if probe.hit(x_bounds, y_bounds):
                    valid_launch.append((starting_dx, starting_dy))
                    break

    print(f"task 2: {len(valid_launch)} ")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
