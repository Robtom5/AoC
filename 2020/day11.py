import enum
import numpy as np
from matplotlib import animation
import matplotlib.pyplot as plt

with open("./Day11Input.txt") as infile:
    contents = infile.readlines()

# L empty seat, # occupied seat, . floor
# Decisions taken from 8 surrounding seats
# If a seat is empty and there are no occupied seats adjaent, its becomes ocupied
# If a seat is occupied and 4 or more adjacent are occupied, it becomes empty

class SeatState(enum.IntEnum):
    FLOOR = 0
    EMPTY = 1
    OCCUPIED = 2

num_rows = len(contents)
num_cols = len(contents[0])
initial_room = np.zeros((num_cols, num_rows))

def parse_row(room, row, row_num):
    for i in range(num_cols):
        seat_state = SeatState.FLOOR
        if row[i] == 'L':
            seat_state = SeatState.EMPTY
        elif row[i] == '#':
            seat_state = SeatState.OCCUPIED

        room[i, row_num] = seat_state

def initialize_room(room, rawInput):
    for i in range(num_rows):
        parse_row(room, rawInput[i], i)
    # Pad with floor
    return np.pad(room, 1, mode='constant', constant_values=0)

loaded_room = initialize_room(initial_room, contents)

def generation(room, offset):
    stable = True
    next_generation = np.zeros((num_cols + 2, num_rows + 2))
    for i in range(offset, num_rows+offset):
        for j in range(offset, num_cols+offset):
            cell = room[j, i]
            surroundings = room[j-1:j+2, i-1:i+2]
            new_value = cell
            if cell == SeatState.FLOOR:
                new_value = SeatState.FLOOR
            elif cell == SeatState.EMPTY:
                if not SeatState.OCCUPIED in surroundings:
                    new_value = SeatState.OCCUPIED
            elif cell == SeatState.OCCUPIED:
                if np.count_nonzero(surroundings == SeatState.OCCUPIED) > 4:
                    new_value = SeatState.EMPTY
            stable = (new_value == cell) and stable
            next_generation[j, i] = new_value
    return stable, next_generation

stable, gen = generation(loaded_room, 1)
gen_count = 1
while not stable:
    stable, gen = generation(gen, 1)
    gen_count += 1
    if gen_count % 5 == 0:
        print(f"Generation: {gen_count}")
print(f"Occupied Seats: {np.count_nonzero(gen == SeatState.OCCUPIED)}")
#need to vectorize the function to calculating seats rather than just doing it in fors as slow

# Part 2
# walk in cardinals till find seat rather than just checking adjacent
# also. threshold for leaving now 5
