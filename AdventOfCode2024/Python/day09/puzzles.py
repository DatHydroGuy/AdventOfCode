def part1(disk_map):
    disk_map = defragment_disk(disk_map)
    return calculate_disk_checksum(disk_map)


def calculate_disk_checksum(disk_map):
    pointer = 0
    total = 0
    for element in disk_map:
        if element[0] != -1:
            total += element[0] * sum_from_a_to_b(pointer - 1, pointer + element[1] - 1)
        pointer += element[1]
    return total


def sum_from_a_to_b(a, b):
    sum_to_a = a * (a + 1) * 0.5
    sum_to_b = b * (b + 1) * 0.5
    return sum_to_b - sum_to_a


def defragment_disk(disk_map):
    index_of_last_file = max([i for i, x in enumerate(disk_map) if x[0] != -1])
    for elem_idx, element in enumerate(disk_map):
        if elem_idx >= index_of_last_file:
            break

        if element[0] == -1:
            # if file at end of disk is smaller than current block of space
            if disk_map[index_of_last_file][1] < element[1]:
                disk_map.insert(elem_idx, [disk_map[index_of_last_file][0], disk_map[index_of_last_file][1]])
                disk_map[elem_idx + 1][1] -= disk_map[index_of_last_file + 1][1]
                disk_map[index_of_last_file][1] += disk_map[index_of_last_file + 1][1]
                disk_map.pop(index_of_last_file + 1)
            elif disk_map[index_of_last_file][1] > element[1]:
                disk_map[elem_idx][0] = disk_map[index_of_last_file][0]
                disk_map[index_of_last_file][1] -= element[1]
            else:
                disk_map[elem_idx][0] = disk_map[index_of_last_file][0]
                disk_map.pop(index_of_last_file)

            index_of_last_file = max([i for i, x in enumerate(disk_map) if x[0] != -1])
    consecutive_blank_spaces = sum([x[1] for x in disk_map if x[0] == -1])
    disk_map[index_of_last_file + 1][1] = consecutive_blank_spaces
    disk_map = disk_map[:index_of_last_file + 2]
    return disk_map


def part2(disk_map):
    disk_map = defragment_disk_contiguous(disk_map)
    return calculate_disk_checksum(disk_map)


def get_index_of_space_of_given_length(disk_map, length):
    indices = [i for i, x in enumerate(disk_map) if x[0] == -1 and x[1] >= length]
    return min(indices) if len(indices) > 0 else -1


def get_index_of_last_file(disk_map, before=-1):
    indices = [i for i, x in enumerate(disk_map) if x[0] != -1 and i < before]
    return max(indices) if len(indices) > 0 else -1


def defragment_disk_contiguous(disk_map):
    next_space_big_enough = 0
    index_of_last_file = len(disk_map)
    while True:
        index_of_last_file = get_index_of_last_file(disk_map, index_of_last_file)
        if index_of_last_file <= 0:
            break

        last_file_length = disk_map[index_of_last_file][1]
        temp_next_space_big_enough = get_index_of_space_of_given_length(disk_map, last_file_length)
        if temp_next_space_big_enough == -1 or temp_next_space_big_enough > index_of_last_file:
            continue
        else:
            next_space_big_enough = temp_next_space_big_enough

        element = disk_map[next_space_big_enough]

        # if file at end of disk is smaller than current block of space
        if disk_map[index_of_last_file][1] < element[1]:
            disk_map.insert(next_space_big_enough, [disk_map[index_of_last_file][0], disk_map[index_of_last_file][1]])
            disk_map[next_space_big_enough + 1][1] -= disk_map[index_of_last_file + 1][1]
            disk_map[index_of_last_file][1] += disk_map[index_of_last_file + 1][1]
            disk_map.pop(index_of_last_file + 1)
        elif disk_map[index_of_last_file][1] > element[1]:
            disk_map[next_space_big_enough][0] = disk_map[index_of_last_file][0]
            disk_map[index_of_last_file][1] -= element[1]
        else:
            disk_map[next_space_big_enough][0] = disk_map[index_of_last_file][0]
            if disk_map[index_of_last_file - 1][0] == -1:
                disk_map[index_of_last_file - 1][1] += last_file_length
                disk_map.pop(index_of_last_file)
            else:
                disk_map[index_of_last_file][0] = -1

    return disk_map


def parse_data(data):
    disk = []
    id_number = 0
    is_file = True
    for element in data[0]:
        if is_file:
            disk.append([id_number, int(element)])
            id_number += 1
        else:
            disk.append([-1, int(element)])  # -1 will represent space
        is_file = not is_file

    return disk


def run_puzzles(data):
    parsed_data = parse_data(data)
    answer1 = part1(parsed_data)
    print("DAY 09, PART 1 RESULT: ", answer1)
    parsed_data2 = parse_data(data)
    answer2 = part2(parsed_data2)
    print("DAY 09, PART 2 RESULT: ", answer2)
