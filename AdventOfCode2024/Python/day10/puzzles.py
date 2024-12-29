from collections import defaultdict
from heapq import heappush, heappop


def find_trail_heads(grid):
    trail_heads = defaultdict(set)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == 0:
                trail_heads[(row, col)] = set()     # finds uniques paths
    return trail_heads


def find_unique_trail_heads(grid):
    trail_heads = defaultdict(list)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == 0:
                trail_heads[(row, col)] = []        # finds all paths
    return trail_heads


def find_trail_ends(grid):
    trail_ends = []
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == 9:
                trail_ends.append((row, col))
    return trail_ends


def run(grid):
    trail_heads = find_trail_heads(grid)
    unique_trail_heads = find_unique_trail_heads(grid)
    trail_ends = find_trail_ends(grid)
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]     # right, down, left, up
    for trail_end in trail_ends:
        pq = [(9, trail_end[0], trail_end[1], 0, 0, trail_end)]

        while pq:
            height, curr_y, curr_x, delta_y, delta_x, end_cell = heappop(pq)

            if height == 0:
                trail_heads[(curr_y, curr_x)].add(end_cell)
                unique_trail_heads[(curr_y, curr_x)].append(end_cell)

            for direction in directions:
                next_y = curr_y + direction[0]
                next_x = curr_x + direction[1]

                if next_y < 0 or next_y >= len(grid):
                    continue

                if next_x < 0 or next_x >= len(grid[0]):
                    continue

                if grid[next_y][next_x] != grid[curr_y][curr_x] - 1:
                    continue

                heappush(pq, (grid[next_y][next_x], next_y, next_x, direction[0], direction[1], end_cell))

    total = sum([len(trail_heads[x]) for x in trail_heads])
    unique_total = sum([len(unique_trail_heads[x]) for x in unique_trail_heads])
    return total, unique_total


def parse_data(data):
    return [[int(char) for char in row.strip()] for row in data]


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1, answer2 = run(parsed_data)
    print(f"DAY 10, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    print(f"DAY 10, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
