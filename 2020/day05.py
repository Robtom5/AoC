with open("./Day05Input.txt") as infile:
    contents = infile.readlines()

NUMROWS = 128
NUMCOLS = 8

def find_row(instructions):
    return recur(range(NUMROWS), instructions, 'B', 'F')

def find_col(instructions):
    return recur(range(NUMCOLS), instructions, 'R', 'L')

def recur(possible_values, instructions, upper_char, lower_char):
    len_possibles = len(possible_values)
    if len_possibles == 1:
        return possible_values[0]
    half = int(len_possibles/2)
    if instructions[0] == upper_char:
        return recur(possible_values[half:], instructions[1:], upper_char, lower_char)
    else:
        return recur(possible_values[:half], instructions[1:], upper_char, lower_char)

def find_seat(instructions):
    row = find_row(instructions[:7])
    col = find_col(instructions[7:])
    return (row, col)

def calculate_id(row, col):
    return (row * 8) + col

parsed = [find_seat(line) for line in contents]
ids = [calculate_id(r,c) for r,c in parsed]
print(f"Max Seat ID {max(ids)}")

# Part 2

allIds = []
for c in range(NUMCOLS):
    for r in range(NUMROWS):
        allIds.append(calculate_id(r, c));

possibleSeats = []
for seatID in allIds:
    if seatID in ids:
        continue
    if seatID + 1 not in ids:
        continue
    if seatID - 1 not in ids:
        continue
    possibleSeats.append(seatID)

print(possibleSeats)

# Ugly brute fore
brute= []
ids.sort()
index = 1
while index < len(ids):
    if ids[index] == ids[index - 1] + 1:
        pass
    else:
        brute.append(ids[index]-1)
    index += 1
print(brute)
