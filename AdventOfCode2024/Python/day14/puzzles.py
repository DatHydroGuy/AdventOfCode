def part1(robots):
    grid_width = 101
    grid_height = 103
    num_steps = 100
    final_positions = []
    for robot in robots:
        final_y = (robot['position'][0] + num_steps * robot['velocity'][0]) % grid_height
        final_x = (robot['position'][1] + num_steps * robot['velocity'][1]) % grid_width
        final_positions.append([final_y, final_x])
    half_width = grid_width // 2
    half_height = grid_height // 2
    top_left_quad = len([x for x in final_positions if 0 <= x[0] < half_height and 0 <= x[1] < half_width])
    top_right_quad = len([x for x in final_positions if 0 <= x[0] < half_height and half_width < x[1] < grid_width])
    bottom_left_quad = len([x for x in final_positions if half_height < x[0] < grid_height and 0 <= x[1] < half_width])
    bottom_right_quad = len([x for x in final_positions if half_height < x[0] < grid_height and half_width < x[1] < grid_width])
    return top_left_quad * top_right_quad * bottom_left_quad * bottom_right_quad


# original method was to plot the positions of the robots and manually find the tree shape. See images folder.
# def part2(robots):
#     from PIL import Image
#     grid_width = 101
#     grid_height = 103
#     for step in range(10000):
#         img = Image.new('RGB', (grid_height, grid_width), color='black')
#         array = img.load()
#         for robot in robots:
#             final_y = (robot['position'][0] + step * robot['velocity'][0]) % grid_height
#             final_x = (robot['position'][1] + step * robot['velocity'][1]) % grid_width
#             array[final_y, final_x] = (255, 255, 255)
#         img.save(f'day14//images//img{str(step).zfill(6)}.png')


# programmatic method involves looking for a quadrant with significantly more robots than the others
# obviously fails if the tree is bisected by a midline.
def part2(robots):
    grid_width = 101
    grid_height = 103
    num_steps = 10000
    for step in range(num_steps):
        final_positions = []
        for robot in robots:
            final_y = (robot['position'][0] + step * robot['velocity'][0]) % grid_height
            final_x = (robot['position'][1] + step * robot['velocity'][1]) % grid_width
            final_positions.append([final_y, final_x])
        half_width = grid_width // 2
        half_height = grid_height // 2
        top_left_quad = len([x for x in final_positions if 0 <= x[0] < half_height and 0 <= x[1] < half_width])
        top_right_quad = len([x for x in final_positions if 0 <= x[0] < half_height and half_width < x[1] < grid_width])
        bottom_left_quad = len([x for x in final_positions if half_height < x[0] < grid_height and 0 <= x[1] < half_width])
        bottom_right_quad = len([x for x in final_positions if half_height < x[0] < grid_height and half_width < x[1] < grid_width])
        quads = sorted([top_left_quad, top_right_quad, bottom_left_quad, bottom_right_quad])
        if quads[3] >= quads[2] * 2.5:
            return step


def parse_data(raw_data):
    robots = []
    for data_line in raw_data:
        position, velocity = data_line.strip().split(' ')
        p_label, p_values = position.split('=')
        v_label, v_values = velocity.split('=')
        p_x, p_y = p_values.split(',')
        v_x, v_y = v_values.split(',')
        robots.append({'position': [int(p_y), int(p_x)], 'velocity': [int(v_y), int(v_x)]})
    return robots


def run_puzzles(data):
    parsed_data = parse_data(data)
    answer1 = part1(parsed_data)
    print("DAY 14, PART 1 RESULT: ", answer1)
    parsed_data2 = parse_data(data)
    answer2 = part2(parsed_data2)
    print("DAY 14, PART 2 RESULT: ", answer2)
