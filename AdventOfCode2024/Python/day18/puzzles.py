from heapq import heappush, heappop


def part1(blocks, size=71, num_bytes=1024):
    grid = create_grid(blocks, num_bytes, size)

    rows = size
    cols = size

    start = (0, 0)
    end = (rows - 1, cols - 1)

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

            if next_row < 0 or next_column < 0 or next_row >= rows or next_column >= cols:
                continue

            if grid[next_row][next_column] != '#' and (next_row, next_column, direction[0], direction[1]) not in visited:
                new_score = score + 1
                heappush(pq, (new_score, next_row, next_column, direction[0], direction[1]))

    return float('inf')


def create_grid(blocks, num_bytes, size):
    grid = [['.' for _ in range(size)] for _ in range(size)]
    for block in blocks[:num_bytes]:
        grid[block[0]][block[1]] = '#'
    return grid


def part2(blocks):
    start = 1023
    i = 0
    result = 0
    while result != float('inf'):
        i += 1
        result = part1(blocks, size=71, num_bytes=start + i)
    return tuple(reversed(blocks[start + i - 1]))

def parse_data(data):
    blocks = []
    for line in data:
        x, y = line.strip().split(',')
        blocks.append((int(y), int(x)))
    return blocks


def run_puzzles(data):
    import time
    start_time = time.time()
    parsed_data = parse_data(data)
    answer1 = part1(parsed_data)
    print(f"DAY 18, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
    start_time = time.time()
    parsed_data = parse_data(data)
    answer2 = part2(parsed_data)
    print(f"DAY 18, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
