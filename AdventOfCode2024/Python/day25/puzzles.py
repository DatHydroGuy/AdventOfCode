import numpy as np


def part1(locks, keys, max_sum):
    answer = 0
    for lock in locks:
        for key in keys:
            if all([lock[i] + key[i] <= max_sum for i in range(len(lock))]):
                answer += 1
    return answer


def part2(data):
    return None


def parse_data(raw_data):
    schematics = parse_schematics(raw_data)
    locks = np.array([l for l in schematics if all([c == '#' for c in l[0]])])
    for lock in locks:
        lock[lock == '#'] = 1
        lock[lock == '.'] = 0
    keys = np.array([l for l in schematics if all([c == '#' for c in l[-1]])])
    for key in keys:
        key[key == '#'] = 1
        key[key == '.'] = 0
    int_locks = [np.sum(lock[1:].astype('int32'), axis=0) for lock in locks]
    int_keys = [np.sum(key[:-1].astype('int32'), axis=0) for key in keys]
    max_sum = len(schematics[0]) - 2    # need to exclude the top and bottom rows
    return int_locks, int_keys, max_sum


def parse_schematics(raw_data):
    schematics = []
    schematic = []
    for row in raw_data:
        if row == "\n":
            schematics.append(schematic)
            schematic = []
            continue
        schematic.append([cell for cell in row.strip()])
    schematics.append(schematic)
    return schematics


def parse_data2(raw_data):
    return 0


def run_puzzles(data):
    locks, keys, max_sum = parse_data(data)
    answer1 = part1(locks, keys, max_sum)
    print(f"DAY 25, PART 1 RESULT: \033[92m{answer1}\033[0m")
    parsed_data2 = parse_data2(data)
    answer2 = part2(parsed_data2)
    print(f"DAY 25, PART 2 RESULT: \033[92m{answer2}\033[0m")
