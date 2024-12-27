import functools


def part1(data):
    for _ in range(25):
        data = evolve_stones(data)
        # print(data)
    return len(data)


def evolve_stones(data):
    data_len = len(data)
    for idx, stone in enumerate(data[::-1]):
        rev_idx = data_len - idx - 1
        stone_str = str(stone)
        if stone == 0:
            data[rev_idx] = 1
        elif len(stone_str) % 2 == 0:
            first_half = stone_str[:len(stone_str) // 2]
            second_half = stone_str[len(stone_str) // 2:]
            data[rev_idx] = int(first_half)
            data.insert(rev_idx + 1, int(second_half))
        else:
            data[rev_idx] = stone * 2024
    return data


@functools.lru_cache(maxsize=None)
def evolve_stone(stone):
    stone_str = str(stone)
    stone_len = len(stone_str)
    if stone == 0:
        return 1, None
    elif stone_len % 2 == 0:
        half_len = stone_len // 2
        return int(stone_str[:half_len]), int(stone_str[half_len:])
    else:
        return stone * 2024, None


@functools.lru_cache(maxsize=None)
def evolve_stone_recursively(stone, blinks):

    left, right = evolve_stone(stone)

    if blinks == 1:
        return 1 if right is None else 2

    num_stones = evolve_stone_recursively(left, blinks - 1)
    if right is not None:
        num_stones += evolve_stone_recursively(right, blinks - 1)

    return num_stones


def part2(data):
    total = 0
    for stone in data:
        total += evolve_stone_recursively(stone, 75)
    return total


def parse_data(data):
    return [int(n) for n in data[0].strip().split()]


def run_puzzles(data):
    data1 = parse_data(data)
    answer1 = part1(data1)
    print("DAY 11, PART 1 RESULT: ", answer1)
    parsed_data2 = parse_data(data)
    answer2 = part2(parsed_data2)
    print("DAY 11, PART 2 RESULT: ", answer2)
