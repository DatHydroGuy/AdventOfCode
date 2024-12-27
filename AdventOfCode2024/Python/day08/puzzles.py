from collections import defaultdict


def get_antenna_locations(grid):
    locations = defaultdict(list)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] != '.':
                locations[grid[row][col]].append((row, col))
    return locations


def part1(grid):
    total = set()
    antenna_locations = get_antenna_locations(grid)
    for antenna_type in antenna_locations.keys():
        locations = antenna_locations[antenna_type]
        for location_index in range(len(locations)):
            for location_index2 in range(location_index + 1, len(locations)):
                location_diff_y = locations[location_index][0] - locations[location_index2][0]
                location_diff_x = locations[location_index][1] - locations[location_index2][1]
                antinode1 = (locations[location_index][0] + location_diff_y, locations[location_index][1] + location_diff_x)
                if 0 <= antinode1[0] < len(grid) and 0 <= antinode1[1] < len(grid[0]):
                    total.add(antinode1)
                antinode2 = (locations[location_index2][0] - location_diff_y, locations[location_index2][1] - location_diff_x)
                if 0 <= antinode2[0] < len(grid) and 0 <= antinode2[1] < len(grid[0]):
                    total.add(antinode2)
    return len(total)


def part2(grid):
    total = set()
    antenna_locations = get_antenna_locations(grid)
    for antenna_type in antenna_locations.keys():
        locations = antenna_locations[antenna_type]
        for location_index in range(len(locations)):
            for location_index2 in range(location_index + 1, len(locations)):
                location_diff_y = locations[location_index][0] - locations[location_index2][0]
                location_diff_x = locations[location_index][1] - locations[location_index2][1]
                antinode1_y = locations[location_index][0] + location_diff_y
                antinode1_x = locations[location_index][1] + location_diff_x
                while 0 <= antinode1_y < len(grid) and 0 <= antinode1_x < len(grid[0]):
                    total.add((antinode1_y, antinode1_x))
                    antinode1_y += location_diff_y
                    antinode1_x += location_diff_x

                antinode2_y = locations[location_index2][0] - location_diff_y
                antinode2_x = locations[location_index2][1] - location_diff_x
                while 0 <= antinode2_y < len(grid) and 0 <= antinode2_x < len(grid[0]):
                    total.add((antinode2_y, antinode2_x))
                    antinode2_y -= location_diff_y
                    antinode2_x -= location_diff_x

        # add in antenna locations with lengths > 1
        if len(locations) > 1:
            for location in locations:
                total.add(location)

    return len(total)


def parse_data(raw_data):
    return [[cell for cell in data_line.strip()] for data_line in raw_data]


def run_puzzles(data):
    parsed_data = parse_data(data)
    answer1 = part1(parsed_data)
    print("DAY 08, PART 1 RESULT: ", answer1)
    parsed_data2 = parse_data(data)
    answer2 = part2(parsed_data2)
    print("DAY 08, PART 2 RESULT: ", answer2)
