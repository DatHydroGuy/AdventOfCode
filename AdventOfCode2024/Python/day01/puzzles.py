def part1(data):
    first_list, second_list = data
    total = 0
    for i in range(len(first_list)):
        total += dist_between_numbers(first_list[i], second_list[i])
    return total


def dist_between_numbers(first_num, second_num):
    distance = abs(second_num - first_num)
    return distance


def part2(data):
    first_list, second_list = data
    total = 0
    for i in range(len(first_list)):
        total += first_list[i] * second_list.count(first_list[i])
    return total


def parse_data(raw_data):
    first_list, second_list = read_lists_from_raw_data(raw_data)
    first_list.sort()
    second_list.sort()
    return first_list, second_list


def read_lists_from_raw_data(raw_data):
    first_list = []
    second_list = []
    for data_line in raw_data:
        line_numbers = data_line.strip().split()
        first_list.append(int(line_numbers[0]))
        second_list.append(int(line_numbers[1]))
    return first_list, second_list


def parse_data2(raw_data):
    first_list, second_list = read_lists_from_raw_data(raw_data)
    return first_list, second_list


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 01, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data2(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 01, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
