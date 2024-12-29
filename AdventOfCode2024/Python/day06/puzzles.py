def get_start_pos(grid):
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == '^':
                return row, col


def part1(grid):
    curr_pos = get_start_pos(grid)
    curr_dir = [-1, 0]
    visited = set()
    while 0 <= curr_pos[0] < len(grid) and 0 <= curr_pos[1] < len(grid[0]):
        if curr_pos not in visited:
            visited.add(curr_pos)

        next_y = curr_pos[0] + curr_dir[0]
        next_x = curr_pos[1] + curr_dir[1]
        if not (0 <= next_y < len(grid) and 0 <= next_x < len(grid[0])):
            break

        if grid[next_y][next_x] == '#':
            if curr_dir[0] != 0:
                curr_dir[0], curr_dir[1] = 0, -curr_dir[0]
            else:
                curr_dir[0], curr_dir[1] = curr_dir[1], curr_dir[0]
            continue

        curr_pos = (next_y, next_x)

    return len(visited)


def part2(grid):
    total = 0
    start_y, start_x = get_start_pos(grid)
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if x == start_x and y == start_y:
                continue
            old = grid[y][x]
            grid[y][x] = '#'
            if find_loop_in_grid(grid):
                total += 1
            grid[y][x] = old
    return total


def find_loop_in_grid(grid):
    grid_state = tuple(list(get_start_pos(grid)) + [-1, 0])
    visited = set()
    while 0 <= grid_state[0] < len(grid) and 0 <= grid_state[1] < len(grid[0]):
        if grid_state not in visited:
            visited.add(grid_state)
        else:
            return True

        next_y = grid_state[0] + grid_state[2]
        next_x = grid_state[1] + grid_state[3]
        if not (0 <= next_y < len(grid) and 0 <= next_x < len(grid[0])):
            return False

        if grid[next_y][next_x] == '#':
            if grid_state[2] != 0:
                grid_state = (grid_state[0], grid_state[1], 0, -grid_state[2])
            else:
                grid_state = (grid_state[0], grid_state[1], grid_state[3], grid_state[2])
            continue

        grid_state = (next_y, next_x, grid_state[2], grid_state[3])

    return False


def parse_data(raw_data):
    return [[cell for cell in data_line.strip()] for data_line in raw_data]


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 06, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = part2(parsed_data)
    print(f"DAY 06, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
