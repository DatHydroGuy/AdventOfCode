import numpy as np


def part1(locks, keys, max_sum):
    answer = 0
    for lock in locks:
        for key in keys:
            if all([lock[i] + key[i] <= max_sum for i in range(len(lock))]):
                answer += 1
    return answer


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


def run_puzzles(data):
    import time

    locks, keys, max_sum = parse_data(data)
    start_time = time.time()
    answer1 = part1(locks, keys, max_sum)
    print(f"DAY 25, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = "MERRY CHRISTMAS!"    # There is no part 2 :)
    print(f"DAY 25, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
