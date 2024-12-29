def part1(data):
    return 0


def part2(data):
    return 0


def parse_data(raw_data):
    return 0


def parse_data2(raw_data):
    return 0


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 0x, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 0x, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
