import numpy as np
import sys

STEPS=int(sys.argv[1])

with open("input.txt") as f:
    lines = f.read()
lines = lines.splitlines()
line_count = len(lines)
line_length = len(lines[0])
# Edge cases go away
matrix = np.empty((line_count, line_length), bool)

def get_value_safe(y, x):
    if y >= 0 and y < line_count and x >= 0 and x < line_length:
        return matrix[y, x]
    else:
        return False

start_y = -1
start_x = -1
for y, line in enumerate(lines):
    for x, char in enumerate(line):
        matrix[y, x] = False if char == '#' else True
        if char == 'S':
            start_y = y
            start_x = x
current_positions: set[tuple[int, int]] = set()
current_positions.add((start_y, start_x))
for step in range(STEPS):
    next_positions: set[tuple[int, int]] = set()
    for (y, x) in current_positions:
        # Down
        if get_value_safe(y + 1, x):
            next_positions.add((y+1, x))
        # Up
        if get_value_safe(y - 1, x):
            next_positions.add((y-1, x))
        # Right
        if get_value_safe(y, x + 1):
            next_positions.add((y, x+1))
        # Left
        if get_value_safe(y, x - 1):
            next_positions.add((y, x-1))
    current_positions = next_positions
print(len(current_positions))
