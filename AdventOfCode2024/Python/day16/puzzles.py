from heapq import heappush, heappop


def part1(grid):
    rows = len(grid)
    cols = len(grid[0])

    start = (rows - 2, 1)
    end = (1, cols - 2)

    grid[start[0]][start[1]] = '.'
    grid[end[0]][end[1]] = '.'

    pq = [(0, start[0], start[1], 0, 1)]  # score, row, column, y_direction, x_direction
    visited = set()
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

    while pq:
        score, row, column, y_direction, x_direction = heappop(pq)

        if (row, column, y_direction, x_direction) in visited:
            continue

        visited.add((row, column, y_direction, x_direction))

        if row == end[0] and column == end[1]:
            return score

        for direction in directions:
            next_row = row + direction[0]
            next_column = column + direction[1]

            if grid[next_row][next_column] != '#' and (next_row, next_column, direction[0], direction[1]) not in visited:
                if direction[0] == y_direction and direction[1] == x_direction:
                    new_score = score + 1
                elif direction[0] == -y_direction and direction[1] == -x_direction:
                    continue
                else:
                    new_score = score + 1001
                heappush(pq, (new_score, next_row, next_column, direction[0], direction[1]))

    return float('inf')


def part2(grid, lowest_score):
    rows = len(grid)
    cols = len(grid[0])

    start = (rows - 2, 1)
    end = (1, cols - 2)

    grid[start[0]][start[1]] = '.'
    grid[end[0]][end[1]] = '.'

    pq = [(0, start[0], start[1], 0, 1, {start})]  # score, row, column, y_direction, x_direction, path_set
    visited = {}
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    lowest_paths = set()

    while pq:
        score, row, column, y_direction, x_direction, path_set = heappop(pq)

        state_key = (row, column, y_direction, x_direction)
        if state_key in visited and visited[state_key] < score:
            continue

        visited[state_key] = score

        if row == end[0] and column == end[1]:
            if score <= lowest_score:
                lowest_paths.update(path_set)
            continue

        for direction in directions:
            next_row = row + direction[0]
            next_column = column + direction[1]

            if grid[next_row][next_column] != '#':
                new_path_set = set(path_set)
                new_path_set.add((next_row, next_column))

                if direction[0] == y_direction and direction[1] == x_direction:
                    new_score = score + 1
                elif direction[0] == -y_direction and direction[1] == -x_direction:
                    continue
                else:
                    new_score = score + 1001

                next_state_key = (next_row, next_column, direction[0], direction[1])
                if next_state_key not in visited or visited[next_state_key] > new_score:
                    heappush(pq, (new_score, next_row, next_column, direction[0], direction[1], new_path_set))

    return len(lowest_paths)


def parse_data(data):
    return [[x for x in y.strip()] for y in data]


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 16, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data(data)
    start_time = time.time()
    answer2 = part2(parsed_data2, answer1)
    print(f"DAY 16, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
