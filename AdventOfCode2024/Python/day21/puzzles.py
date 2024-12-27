import functools


@functools.lru_cache(maxsize=None)
def num_keys(source_key, target_key):
    return {
            '7': {'7':'', '8': '>', '9': '>>', '4': 'v', '5': 'v>', '6': 'v>>', '1': 'vv', '2': 'vv>', '3': 'vv>>', '0': '>vvv', 'A': '>>vvv'},
            '8': {'7': '<', '8':'', '9': '>', '4': '<v', '5': 'v', '6': 'v>', '1': '<vv', '2': 'vv', '3': 'vv>', '0': 'vvv', 'A': 'vvv>'},
            '9': {'7': '<<', '8': '<', '9':'', '4': '<<v', '5': '<v', '6': 'v', '1': '<<vv', '2': '<vv', '3': 'vv', '0': '<vvv', 'A': 'vvv'},
            '4': {'7': '^', '8': '^>', '9': '^>>', '4':'', '5': '>', '6': '>>', '1': 'v', '2': 'v>', '3': 'v>>', '0': '>vv', 'A': '>>vv'},
            '5': {'7': '<^', '8': '^', '9': '^>', '4': '<', '5':'', '6': '>', '1': '<v', '2': 'v', '3': 'v>', '0': 'vv', 'A': 'vv>'},
            '6': {'7': '<<^', '8': '<^', '9': '^', '4': '<<', '5': '<', '6':'', '1': '<<v', '2': '<v', '3': 'v', '0': '<vv', 'A': 'vv'},
            '1': {'7': '^^', '8': '^^>', '9': '^^>>', '4': '^', '5': '^>', '6': '^>>', '1':'', '2': '>', '3': '>>', '0': '>v', 'A': '>>v'},
            '2': {'7': '<^^', '8': '^^', '9': '^^>', '4': '<^', '5': '^', '6': '^>', '1': '<', '2':'', '3': '>', '0': 'v', 'A': 'v>'},
            '3': {'7': '<<^^', '8': '<^^', '9': '^^', '4': '<<^', '5': '<^', '6': '^', '1': '<<', '2': '<', '3':'', '0': '<v', 'A': 'v'},
            '0': {'7': '^^^<', '8': '^^^', '9': '^^^>', '4': '^^<', '5': '^^', '6': '^^>', '1': '^<', '2': '^', '3': '^>', '0':'', 'A': '>'},
            'A': {'7': '^^^<<', '8': '<^^^', '9': '^^^', '4': '^^<<', '5': '<^^', '6': '^^', '1': '^<<', '2': '<^', '3': '^', '0': '<', 'A':''}
        }[source_key][target_key] + 'A'


@functools.lru_cache(maxsize=None)
def dir_keys(source_key, target_key):
    return {
             '^': {'^': '', 'A': '>', '<': 'v<', 'v': 'v', '>': 'v>'},
             'A': {'^': '<', 'A': '', '<': 'v<<', 'v': '<v', '>': 'v'},
             '<': {'^': '>^', 'A': '>>^', '<': '', 'v': '>', '>': '>>'},
             'v': {'^': '^', 'A': '^>', '<': '<', 'v': '', '>': '>'},
             '>': {'^': '<^', 'A': '^', '<': '<<', 'v': '<', '>': ''}
        }[source_key][target_key] + 'A'


def get_numeric_part(numpad_code):
    return int(numpad_code[:-1])


def build_num_sequence(keys, previous_key, result):
    if len(keys) == 0:
        return

    result.append(num_keys(previous_key, keys[0]))
    build_num_sequence(keys[1:], keys[0], result)


def build_dir_sequence(keys, previous_key, result):
    if len(keys) == 0:
        return

    result.append(dir_keys(previous_key, keys[0]))
    build_dir_sequence(keys[1:], keys[0], result)


def get_shortest_sequence(keys, depth, cache):
    if depth == 0:
        return len(keys)

    total = 0

    if (keys, depth) in cache:
        return cache[(keys, depth)]

    sub_keys = [s + 'A' for s in keys.split('A')][:-1]

    for sub_key in sub_keys:
        answer = []
        build_dir_sequence(sub_key, 'A', answer)
        sub_sequence = ''.join(answer)
        total += get_shortest_sequence(sub_sequence, depth - 1, cache)
    cache[(keys, depth)] = total
    return total


def solve(numpad_codes, num_intermediate_robots):
    result = 0
    answer = []

    for numpad_code in numpad_codes:
        sequence_length = 0
        number_part = get_numeric_part(numpad_code)

        # Initial conversion
        build_num_sequence(numpad_code, 'A', answer)
        keys = ''.join(answer)
        answer = []
        cache = {}

        # Process remaining conversions
        sub_keys = [s + 'A' for s in keys.split('A')][:-1]
        for sub_key in sub_keys:
            sequence_length += get_shortest_sequence(sub_key, num_intermediate_robots, cache)

        result += number_part * sequence_length

    return result


def parse_data(raw_data):
    return [code.strip() for code in raw_data]


def run_puzzles(data):
    import time
    start_time = time.time()
    parsed_data = parse_data(data)

    answer1 = solve(parsed_data, 2)
    print(f"DAY 21, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
    start_time = time.time()
    answer2 = solve(parsed_data, 25)
    print(f"DAY 21, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
