from itertools import pairwise


def part1(reports):
    total = 0
    for report in reports:
        total += 1 if analyse_report(report) else 0
    return total


def analyse_report(report):
    analysis = [int(b) - int(a) for a, b in pairwise(report)]
    valid = all([1 <= abs(x) <= 3 for x in analysis]) and (
                all([x < 0 for x in analysis]) or all([x > 0 for x in analysis]))
    return valid


def part2(reports):
    total = 0
    for report in reports:
        if analyse_report(report):
            total += 1
            continue

        valid = False
        for idx in range(len(report)):
            dampened_report = report[:idx] + report[idx + 1:]
            valid |= analyse_report(dampened_report)

        total += 1 if valid else 0
    return total


def parse_data(raw_data):
    reports = [report.strip().split(' ') for report in raw_data]
    return reports


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 02, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = part2(parsed_data)
    print(f"DAY 02, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
