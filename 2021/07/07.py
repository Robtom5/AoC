DEBUG = False


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read()

    # content = "3,4,3,1,2"
    content = [int(c) for c in content.split(",")]

    return content


def fuel_to_pos_1(pos, crab):
    return abs(pos - crab)


def fuel_to_pos_2(pos, crab):
    displ = abs(pos - crab)
    return int((displ * displ + displ) * 0.5)


def task_1():
    crabs = read_src()

    def median(crabs):
        sorted_crabs = sorted(crabs)
        return sorted_crabs[int(len(sorted_crabs) / 2)]

    median_crab = median(crabs)
    total_fuel = sum([fuel_to_pos_1(median_crab, crab) for crab in crabs])
    print(f"task 1: {total_fuel}")


def task_2():
    crabs = read_src()
    max_crab, min_crab = max(crabs), min(crabs)

    def calc_total_fuel(pos):
        return sum([fuel_to_pos_2(pos, crab) for crab in crabs])

    cheapest_pos = min_crab
    for x in range(min_crab, max_crab):
        fuel = calc_total_fuel(x)
        cheapest_pos = x if fuel < calc_total_fuel(cheapest_pos) else cheapest_pos
    print(f"task 2: {calc_total_fuel(cheapest_pos)} fuel at pos: {cheapest_pos}")


if __name__ == "__main__":
    task_1()
    task_2()
