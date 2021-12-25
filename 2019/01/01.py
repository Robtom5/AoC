DEBUG = True

import math


def FuelToLaunch(mass):
    return (math.floor(mass / 3)) - 2


def AdvancedFuelToLaunch(mass):
    # this is recursive

    # fuel cost for the module, this is constant
    fuel_mass = max(FuelToLaunch(mass), 0)

    fuel_fuel = 0
    if fuel_mass > 0:
        fuel_fuel = AdvancedFuelToLaunch(fuel_mass)

    return fuel_fuel + fuel_mass


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    modules = [int(l) for l in content]
    return modules


def task_1():
    modules = read_src()
    total_fuel = sum([FuelToLaunch(m) for m in modules])
    print(f"task 1: {total_fuel}")


def task_2():
    modules = read_src()
    total_fuel = sum([AdvancedFuelToLaunch(m) for m in modules])
    print(f"task 2: {total_fuel}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
