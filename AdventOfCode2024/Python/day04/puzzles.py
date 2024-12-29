UP_LEFT = [(0, 0), (-1, -1), (-2, -2), (-3, -3)]
UP = [(0, 0), (-1, 0), (-2, 0), (-3, 0)]
UP_RIGHT = [(0, 0), (-1, 1), (-2, 2), (-3, 3)]
RIGHT = [(0, 0), (0, 1), (0, 2), (0, 3)]
DOWN_RIGHT = [(0, 0), (1, 1), (2, 2), (3, 3)]
DOWN = [(0, 0), (1, 0), (2, 0), (3, 0)]
DOWN_LEFT = [(0, 0), (1, -1), (2, -2), (3, -3)]
LEFT = [(0, 0), (0, -1), (0, -2), (0, -3)]
DIRECTIONS = [UP_LEFT, UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT]

def count_xmasses_at_cell(grid, y, x):
    total = 0
    for direction in DIRECTIONS:
        word = ''
        for cell in direction:
            word += grid[y + cell[0]][x + cell[1]]
        if word == 'XMAS' or word == 'SAMX':
            total += 1
    return total

def count_x_masses_at_cell(grid, y, x):
    total = 0
    if ((grid[y - 1][x - 1] == 'M' and grid[y + 1][x + 1] == 'S')
            or (grid[y - 1][x - 1] == 'S' and grid[y + 1][x + 1] == 'M'))\
            and ((grid[y - 1][x + 1] == 'M' and grid[y + 1][x - 1] == 'S')
            or (grid[y - 1][x + 1] == 'S' and grid[y + 1][x - 1] == 'M')):
        total += 1
    return total


def part1(data):
    width = len(data[0]) - 6
    height = len(data) - 6
    total = 0
    for y in range(3, 3 + height):
        for x in range(3, 3 + width):
            total += count_xmasses_at_cell(data, y, x)
    return total // 2    # occurrences are counted twice!


def part2(data):
    width = len(data[0]) - 2
    height = len(data) - 2
    total = 0
    for y in range(1, 1 + height):
        for x in range(1, 1 + width):
            if data[y][x] == 'A':
                total += count_x_masses_at_cell(data, y, x)
    return total

def parse_data(data):
    # spilt data into character lists and pad around edges with 3 layers
    data = [d.strip() for d in data]
    width = len(data[0])
    grid = [['.', '.', '.'] + [c for c in d] + ['.', '.', '.'] for d in data]
    grid.insert(0, ['.' for _ in range(width + 6)])
    grid.insert(0, ['.' for _ in range(width + 6)])
    grid.insert(0, ['.' for _ in range(width + 6)])
    grid.append(['.' for _ in range(width + 6)])
    grid.append(['.' for _ in range(width + 6)])
    grid.append(['.' for _ in range(width + 6)])
    return grid


def parse_data2(data):
    # spilt data into character lists and pad around edges with 1 layer
    data = [d.strip() for d in data]
    width = len(data[0])
    grid = [['.'] + [c for c in d] + ['.'] for d in data]
    grid.insert(0, ['.' for _ in range(width + 2)])
    grid.append(['.' for _ in range(width + 2)])
    return grid


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 04, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data2(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 04, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
