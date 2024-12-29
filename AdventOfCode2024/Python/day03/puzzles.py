import re


def part1(data):
    total = 0
    for datum in data:
        first_number, second_number = datum.split(',')
        total += int(first_number) * int(second_number)
    return total


def part2(data):
    total = 0
    flag = 1
    for datum in data:
        if ',' in datum:
            first_number, second_number = datum.split(',')
            total += int(first_number) * int(second_number) * flag
        elif datum == "don't":
            flag = 0
        elif datum == 'do':
            flag = 1
    return total


def parse_data(raw_data):
    matches = []
    for data_line in raw_data:
        matches.extend(re.findall(r'mul\(\d{1,3},\d{1,3}\)', data_line))
    matches = [x.replace('mul(', '').replace(')', '') for x in matches]
    return matches


def parse_data2(raw_data):
    matches = []
    for data_line in raw_data:
        matches.extend(re.findall(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)", data_line))
    matches = [x.replace('mul', '').replace('(', '').replace(')', '') for x in matches]
    return matches


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 03, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data2(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 03, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
