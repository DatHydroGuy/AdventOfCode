import collections


def get_start_and_end_positions(grid):
    start = end = (-1, -1)

    for r in range(len(grid)):
        for c in range(len(grid[0])):
            if grid[r][c] == 'S':
                start = (r, c)
            elif grid[r][c] == 'E':
                end = (r, c)
    return start, end


def find_cells_by_manhattan_distance(path_cells, cheat_distance, minimum_saving):
    valid_shortcuts = 0
    for idx in range(len(path_cells)):
        pos1, dist1 = path_cells[idx]
        for idx2 in range(idx + 1, len(path_cells)):
            pos2, dist2 = path_cells[idx2]
            path_dist = abs(dist1 - dist2)
            # if path_dist < cheat_distance:
            #     continue    # don't waste time evaluating neighbouring cells on the path

            manhattan_distance = abs(pos1[0] - pos2[0]) + abs(pos1[1] - pos2[1])
            saving = path_dist - manhattan_distance
            if manhattan_distance <= cheat_distance:
                # viable shortcut.  Does it save more than our required minimum?
                if saving >= minimum_saving:
                    valid_shortcuts += 1

    return valid_shortcuts


def part1(grid):
    start, end = get_start_and_end_positions(grid)
    path_cells = bfs(grid, start, end)

    # find all pairs of cells with a Manhattan distance of 2 or less between them.  These are valid cheats for the track.
    number_valid_shortcuts = find_cells_by_manhattan_distance(path_cells, 2, 100)
    return number_valid_shortcuts


def bfs(grid, start, end):
    rows = len(grid)
    cols = len(grid[0])

    distances = {}
    parents = {}
    visited = set()

    queue = collections.deque([end])
    distances[end] = 0
    visited.add(end)

    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]

    while queue:
        current_cell = queue.popleft()

        for direction in directions:
            next_row = current_cell[0] + direction[0]
            next_col = current_cell[1] + direction[1]

            if next_row < 0 or next_row >= rows or next_col < 0 or next_col >= cols or (next_row, next_col) in visited or grid[next_row][next_col] == '#':
                continue

            visited.add((next_row, next_col))
            queue.append((next_row, next_col))
            distances[(next_row, next_col)] = distances[current_cell] + 1
            parents[(next_row, next_col)] = current_cell

    if start not in distances:
        return float('inf')

    # Reconstruct path
    path_cells = {}
    current_cell = start
    while current_cell != end:
        path_cells[current_cell] = distances[current_cell]
        current_cell = parents[current_cell]
    path_cells[end] = distances[end]

    return sorted(path_cells.items(), key=lambda x: x[1])

def part2(grid):
    start, end = get_start_and_end_positions(grid)
    path_cells = bfs(grid, start, end)

    # find all pairs of cells with a Manhattan distance of 2 or less between them.  These are valid cheats for the track.
    number_valid_shortcuts = find_cells_by_manhattan_distance(path_cells, 20, 100)
    return number_valid_shortcuts


def parse_data(raw_data):
    blocks = [[c for c in line.strip()] for line in raw_data]
    return blocks


def run_puzzles(data):
    import time
    start_time = time.time()
    parsed_data = parse_data(data)
    answer1 = part1(parsed_data)
    print(f"DAY 20, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
    start_time = time.time()
    answer2 = part2(parsed_data)
    print(f"DAY 20, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
