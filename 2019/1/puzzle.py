import math

def FuelToLaunch(mass):
    return (math.floor(mass/3))-2

print(FuelToLaunch(12))
print(FuelToLaunch(14))
print(FuelToLaunch(1969))
print(FuelToLaunch(100756))

## PART 1
with open('source.txt') as f:
    puzzleData=f.read()

modules = puzzleData.splitlines()

print(sum(FuelToLaunch(int(module)) for module in modules))

## PART 2

def AdvancedFuelToLaunch(mass):
    # this is recursive

    # fuel cost for the module, this is constant
    fuel_mass = max(FuelToLaunch(mass),0)

    fuel_fuel = 0
    if (fuel_mass > 0):
        fuel_fuel = AdvancedFuelToLaunch(fuel_mass)

    return fuel_fuel + fuel_mass;

print(sum(AdvancedFuelToLaunch(int(module)) for module in modules))