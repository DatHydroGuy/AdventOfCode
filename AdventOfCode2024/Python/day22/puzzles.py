from collections import defaultdict
from itertools import pairwise


def part1(secret_numbers):
    results = []
    for secret_number in secret_numbers:
        number_sequence = [secret_number] + [secret_number := process_secret_number(secret_number) for _ in range(2000)]
        results.append(number_sequence[-1])
    return sum(results)


def part2(secret_numbers):
    change_map = defaultdict(int)

    for secret_number in secret_numbers:
        number_sequence = [secret_number] + [secret_number := process_secret_number(secret_number) for _ in range(2000)]
        differences = [num2 % 10 - num1 % 10 for num1, num2 in pairwise(number_sequence)]
        seen = set()
        for idx in range(len(number_sequence) - 4):
            curr_key = tuple(differences[idx: idx + 4])
            if curr_key in seen:
                continue
            change_map[curr_key] += number_sequence[idx + 4] % 10
            seen.add(curr_key)

    return max(change_map.values())


def process_secret_number(secret_number):
    # multiply by 64
    value = secret_number << 6
    secret_number = mix_and_prune(secret_number, value)
    # integer divide by 32
    value = secret_number >> 5
    secret_number = mix_and_prune(secret_number, value)
    # multiply by 2048
    value = secret_number << 11
    secret_number = mix_and_prune(secret_number, value)
    return secret_number


def mix_and_prune(secret_number, value):
    # mix
    secret_number ^= value
    # prune
    secret_number &= 16777215
    return secret_number


def parse_data(raw_data):
    return [int(n.strip()) for n in raw_data]


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 22, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = part2(parsed_data)
    print(f"DAY 22, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
