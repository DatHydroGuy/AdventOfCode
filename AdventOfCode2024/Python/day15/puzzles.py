def move_element(grid, curr_y, curr_x, next_y, next_x):
    grid[curr_y][curr_x], grid[next_y][next_x] = grid[next_y][next_x], grid[curr_y][curr_x]
    return next_y, next_x


def peek_ahead(grid, curr_y, curr_x, next_y, next_x, direction):
    if grid[next_y][next_x] == '#':
        return False

    if grid[next_y][next_x] == '.':
        move_element(grid, curr_y, curr_x, next_y, next_x)
        return True

    if peek_ahead(grid, curr_y + direction[0], curr_x + direction[1], next_y + direction[0], next_x + direction[1], direction):
        return move_element(grid, curr_y, curr_x, next_y, next_x)

    return False


def part1(grid, moves):
    init_pos = get_robot_initial_position(grid)
    curr_pos = init_pos
    directions = {'^': (-1, 0), 'v': (1, 0), '<': (0, -1), '>': (0, 1)}
    for move in moves:
        direction = directions[move]
        next_y = curr_pos[0] + direction[0]
        next_x = curr_pos[1] + direction[1]
        if peek_ahead(grid, curr_pos[0], curr_pos[1], next_y, next_x, direction):
            curr_pos = [next_y, next_x]
    return get_gps_score(grid)


def get_gps_score(grid):
    score = 0
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == 'O' or grid[y][x] == '[':
                score += y * 100 + x
    return score


def get_robot_initial_position(grid):
    init_pos = [-1, -1]
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == '@':
                init_pos = [y, x]
                break
    return init_pos


def transform_grid(grid):
    new_grid = []
    for y in grid:
        new_row = []
        for x in y:
            if x == '#':
                new_row.extend(['#', '#'])
            elif x == '.':
                new_row.extend(['.', '.'])
            elif x == '@':
                new_row.extend(['@', '.'])
            elif x == 'O':
                new_row.extend(['[', ']'])
        new_grid.append(new_row)
    return new_grid


def part2(grid, moves):
    grid = transform_grid(grid)
    init_pos = get_robot_initial_position(grid)
    curr_pos = init_pos
    directions = {'^': (-1, 0), 'v': (1, 0), '<': (0, -1), '>': (0, 1)}
    for move in moves:
        direction = directions[move]
        next_y = curr_pos[0] + direction[0]
        next_x = curr_pos[1] + direction[1]
        can_move = peek_ahead2(grid, curr_pos[0], curr_pos[1], next_y, next_x, direction)
        if can_move is not None:
            move_stuff(grid, can_move, direction)
            curr_pos = [next_y, next_x]
    return get_gps_score(grid)


def move_stuff(grid, cells_to_move, direction):
    cells_to_move = list(set(cells_to_move))
    cell_length = len(cells_to_move)

    if direction == (0, -1):
        cells_to_move = sorted(cells_to_move, key=lambda cell: cell[1])
    elif direction == (0, 1):
        cells_to_move = sorted(cells_to_move, key=lambda cell: -cell[1])
    elif direction == (-1, 0):
        cells_to_move = sorted(cells_to_move, key=lambda cell: cell[0])
    elif direction == (1, 0):
        cells_to_move = sorted(cells_to_move, key=lambda cell: -cell[0])

    for index in range(cell_length):
        curr_y, curr_x = cells_to_move[index]
        next_y, next_x = curr_y - direction[0], curr_x - direction[1]
        move_element2(grid, curr_y, curr_x, next_y, next_x)


def move_element2(grid, curr_y, curr_x, next_y, next_x):
    grid[curr_y][curr_x], grid[next_y][next_x] = grid[next_y][next_x], grid[curr_y][curr_x]
    return next_y, next_x


def peek_ahead2(grid, curr_y, curr_x, next_y, next_x, direction, front_face=None):
    if grid[next_y][next_x] == '#':
        return None

    if grid[next_y][next_x] == '.':
        if front_face is None:
            front_face = [(next_y, next_x)]
        else:
            front_face.append((next_y, next_x))

        return front_face

    if grid[next_y][next_x] == '[':
        if direction[0] == 0:
            front_face = peek_ahead2(grid, curr_y + direction[0], curr_x + direction[1], next_y + direction[0], next_x + direction[1], direction, front_face)
        else:
            front_face = peek_ahead2(grid, curr_y + direction[0], curr_x + direction[1], next_y + direction[0], next_x + direction[1], direction, front_face)
            if front_face is not None:
                next_front_face = peek_ahead2(grid, curr_y + direction[0], curr_x + 1 + direction[1], next_y + direction[0], next_x + 1 + direction[1], direction)
                if next_front_face is not None:
                    front_face.extend(next_front_face)
                else:
                    return None
            else:
                return None

    if grid[next_y][next_x] == ']':
        if direction[0] == 0:
            front_face = peek_ahead2(grid, curr_y + direction[0], curr_x + direction[1], next_y + direction[0], next_x + direction[1], direction, front_face)
        else:
            front_face = peek_ahead2(grid, curr_y + direction[0], curr_x + direction[1], next_y + direction[0], next_x + direction[1], direction, front_face)
            if front_face is not None:
                next_front_face = peek_ahead2(grid, curr_y + direction[0], curr_x - 1 + direction[1], next_y + direction[0], next_x - 1 + direction[1], direction)
                if next_front_face is not None:
                    front_face.extend(next_front_face)
                else:
                    return None
            else:
                return None

    if front_face is not None:
        front_face.append((next_y, next_x))

    return front_face


def parse_data(raw_data):
    grid = []
    moves = []
    fill_grid = True
    for data_line in raw_data:
        if len(data_line.strip()) == 0:
            fill_grid = False
            continue
        if fill_grid:
            grid.append([x for x in data_line.strip()])
        else:
            moves.extend([x for x in data_line.strip()])
    return grid, moves


def run_puzzles(data):
    grid, moves = parse_data(data)
    answer1 = part1(grid, moves)
    print("DAY 15, PART 1 RESULT: ", answer1)
    grid, moves = parse_data(data)
    answer2 = part2(grid, moves)
    print("DAY 15, PART 2 RESULT: ", answer2)
