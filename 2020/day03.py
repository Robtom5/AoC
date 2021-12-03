import numpy as np

with open("./Day03Input.txt") as infile:
    lines = infile.readlines()

EXPECTED_WIDTH = 32
TREE = '#'

my_map = np.zeros((len(lines), EXPECTED_WIDTH - 1))

def load_grid(lines):
    x = 0
    y = 0
    for line in lines:
        assert len(line) == EXPECTED_WIDTH
        for feature in line :
            if feature is not '\n':
                my_map[y, x] = feature == TREE
                x += 1
        # do something
        x = 0
        y += 1

load_grid(lines)

def hit_trees(startingCoordinates, vel_x, vel_y):
    
    x, y = startingCoordinates
    slope_height, rep_width = my_map.shape
    hits = 0
    assert vel_y > 0
    while(y < slope_height):
        hits +=  my_map[y, x]
        y += vel_y
        x = (x + vel_x) %rep_width

    return hits

print(f'Trees hit: {hit_trees((0,0), 3, 1)}')

# Part 2

slope_1_hits = hit_trees((0,0), 1, 1)
slope_2_hits = hit_trees((0,0), 3, 1)
slope_3_hits = hit_trees((0,0), 5, 1)
slope_4_hits = hit_trees((0,0), 7, 1)
slope_5_hits = hit_trees((0,0), 1, 2)
product = slope_1_hits * slope_2_hits * slope_3_hits * slope_4_hits * slope_5_hits

print(f'Product of hits: {product}')


