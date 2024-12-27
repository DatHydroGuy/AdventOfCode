def part1(towels, patterns):
    legal_patterns = 0
    memo = {}
    for pattern in patterns:
        if count_pattern_constructions(towels, pattern, memo) > 0:
            legal_patterns += 1
    return legal_patterns, memo


def part2(towels, patterns, memo):
    legal_constructs = 0
    for pattern in patterns:
        legal_constructs += count_pattern_constructions(towels, pattern, memo)
    return legal_constructs


def count_pattern_constructions(all_towels, pattern, memo=None):
    if memo is None:
        memo = {}

    if pattern == "":
        return 1

    if pattern in memo:
        return memo[pattern]

    towels = [t for t in all_towels if t.startswith(pattern[0])]    # filter towels down to relevant ones

    total_constructions = 0
    for towel in towels:
        if pattern.startswith(towel):
            remainder = pattern[len(towel):]
            constructions = count_pattern_constructions(all_towels, remainder, memo)
            total_constructions += constructions

    memo[pattern] = total_constructions
    return total_constructions


def parse_data(raw_data):
    towels = []
    patterns = []
    towel_flag = True
    for row in raw_data:
        if len(row.strip()) == 0:
            towel_flag = False
            continue
        if towel_flag:
            towels = row.strip().split(', ')
        if not towel_flag:
            patterns.append(row.strip())
    return towels, patterns


def run_puzzles(data):
    import time
    start_time = time.time()
    towels, patterns = parse_data(data)
    answer1, memo = part1(towels, patterns)
    print(f"DAY 19, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
    start_time = time.time()
    answer2 = part2(towels, patterns, memo)
    print(f"DAY 19, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.2f} seconds)\033[0m")
