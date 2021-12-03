with open("./Day09Input.txt") as infile:
    contents = infile.readlines()

PREAMBLE_LENGTH = 25
XMAS_raw = [int(n) for n in contents]
PREAMBLE = XMAS_raw[:PREAMBLE_LENGTH]
XMAS_input = XMAS_raw[PREAMBLE_LENGTH:]

def find_pairs(input_value, preamble):
    # loop over preamble and try and find pairs
    preamble_pairs = []
    local_preamble = preamble.copy()
    for val in preamble:
        val_pair = input_value - val
        if val_pair in local_preamble:
            local_preamble.remove(val)
            preamble_pairs.append((val, val_pair))
    return preamble_pairs

def recursive_pair(preamble, remainingInput, starting_depth):
    depth = starting_depth
    # while len(remainingInput) > 0:
    next_value = remainingInput.pop(0)
    depth += 1
    if len(preamble) > 1:
        pairs = find_pairs(next_value, preamble)
    else:
        pairs = False

    if pairs:
        deepest = (0,0)
        preamble_copy = preamble.copy()
        preamble_copy.pop(0)
        preamble_copy.append(next_value)
        remaining = remainingInput.copy()
        pair_depth = recursive_pair(preamble_copy, remaining, depth)
        if (pair_depth[0] > deepest[0]):
            deepest = pair_depth
        return deepest
    else:
        return (depth, next_value)


deepest = recursive_pair(PREAMBLE, XMAS_input, PREAMBLE_LENGTH)
print(f"Deepest: {deepest[1]} at {deepest[0]}")

# Part 2

target = deepest[1]
XMAS_before_target = XMAS_raw[:deepest[0]]

def find_longest_chain(target_number, deepest_index):
    for index in range(deepest_index):
        running_sum = 0
        length = 0
        while running_sum < target_number:
            running_sum += XMAS_before_target[index+length]
            length += 1
        if running_sum == target_number:
            summed_values = XMAS_before_target[index:index+length]
            first = max(summed_values)
            last = min(summed_values)
            return (first, last, first + last)

longest = find_longest_chain(target, deepest[0])
print(f"Greatest: {longest[0]} Lowest: {longest[1]} Weakness: {longest[2]}")

