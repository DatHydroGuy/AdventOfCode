from collections import Counter


def get_unique_values(grid):
    counts = Counter()
    for row in grid:
        for cell in row:
            counts[cell] += 1
    return counts


def get_connected_region(grid, value):
    # Use depth-first search to find connected regions
    visited = [[False] * len(grid[y]) for y in range(len(grid))]
    regions = {}
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] != value:
                continue

            region_cells = []
            boundary_cells = []
            boundary_pairs = []
            stack = [(y, x)]

            while stack:
                curr_y, curr_x = stack.pop()

                if visited[curr_y][curr_x]:
                    continue

                visited[curr_y][curr_x] = True
                region_cells.append((curr_y, curr_x))

                # Check neighbours
                for direction in directions:
                    next_y, next_x = curr_y + direction[0], curr_x + direction[1]

                    if not(0 <= next_y < len(grid) and 0 <= next_x < len(grid[0])):
                        boundary_cells.append((curr_y, curr_x))
                        boundary_pairs.append((curr_y, curr_x, next_y, next_x))
                        continue

                    # If neighbor is the same value and not visited, add to stack
                    if grid[next_y][next_x] == value and not visited[next_y][next_x]:
                        stack.append((next_y, next_x))

                    # Check if neighbor is different value or out of grid (boundary)
                    if grid[next_y][next_x] != value:
                        boundary_cells.append((curr_y, curr_x))
                        boundary_pairs.append((curr_y, curr_x, next_y, next_x))

            if value not in regions:
                regions[value] = []

            if len(region_cells) > 0:
                regions[value].append({
                    'area': len(region_cells),
                    'perimeter': len(boundary_cells),
                    'cells': region_cells,
                    'perimeter_pairs': boundary_pairs,
                })

    return regions


def count_region_sides(region):
    total = 0
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    for direction in directions:
        total += count_edges(region, direction)
    return total


def count_edges(region, direction):
    total = 0
    # get all edges, sorted by y-values for horizontal edges and x-values for vertical edges
    edges = sorted([x for x in region if x[2] == x[0] + direction[0] and x[3] == x[1] + direction[1]], key=lambda x: (x[0], x[1]))
    if direction[0] == 0:
        # left and right edges
        uniques = set(y[1] for y in edges)
    else:
        # top and bottom edges
        uniques = set(y[0] for y in edges)

    # for each unique value, determine runs of consecutive cells
    for unique in uniques:
        if direction[0] == 0:
            # left and right edges
            aligned_cells = [t[0] for t in edges if t[1] == unique]
        else:
            # top and bottom edges
            aligned_cells = [t[1] for t in edges if t[0] == unique]

        if len(aligned_cells) == 1:
            total += 1
            continue

        aligned_cells.sort()
        aligned_count = 0
        old_position = aligned_cells[0]
        for position in aligned_cells[1:]:
            if position != old_position + 1:
                aligned_count += 1

            old_position = position
        aligned_count += 1
        total += aligned_count
    return total


def part1(grid):
    total = 0
    unique_values = get_unique_values(grid)
    for unique_value in unique_values:
        region = get_connected_region(grid, unique_value)
        for sub_region in region[unique_value]:
            total += sub_region['perimeter'] * sub_region['area']

    return total


def part2(grid):
    total = 0
    unique_values = get_unique_values(grid)
    for unique_value in unique_values:
        region = get_connected_region(grid, unique_value)
        for sub_region in region[unique_value]:
            sides = count_region_sides(sub_region['perimeter_pairs'])
            total += sides * sub_region['area']

    return total

def parse_data(data):
    return [[x for x in y.strip()] for y in data]


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 12, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 12, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
