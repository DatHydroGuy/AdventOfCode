def part1(rules_list, updates):
    total = 0
    for update in updates:
        rules = get_rules_for_update(rules_list, update)
        valid = validate_update_against_rules(rules, update)
        sub_total = 0
        if valid:
            sub_total = get_middle_element(update)
        total += sub_total
    return total


def get_rules_for_update(rules_list, update):
    valid_rules = []
    for element in update:
        valid_rules.extend([rule for rule in rules_list if element in rule])
    return set(valid_rules)


def remove_rules_not_in_update(rules_list, update):
    invalid_rules = [rule for rule in rules_list if rule[0] not in update or rule[1] not in update]
    return [rule for rule in rules_list if rule not in invalid_rules]


def validate_update_against_rules(rules, update):
    for rule in rules:
        if rule[0] not in update or rule[1] not in update:
            continue

        if update.index(rule[0]) > update.index(rule[1]):
            return False
    return True


def get_middle_element(update):
    return update[len(update)//2]


def part2(rules_list, updates):
    total = 0
    for update in updates:
        update_copy = update.copy()
        rules = get_rules_for_update(rules_list, update)
        rules = remove_rules_not_in_update(rules, update)
        sorted_rules = sorted(rules, key=lambda rule: (rule[0], rule[1]))
        valid = validate_update_against_rules(sorted_rules, update)
        if not valid:
            while 1 == 1:
                for rule in sorted_rules:
                    first = update.index(rule[0])
                    second = update.index(rule[1])
                    if first < second:
                        continue

                    update[first], update[second] = update[second], update[first]

                if update_copy == update:
                    break

                update_copy = update.copy()

            total += get_middle_element(update)

    return total


def parse_data(data):
    rules = []
    updates = []
    updating_rules = True
    for data_line in data:
        if len(data_line.strip()) == 0:
            updating_rules = False
            continue

        if updating_rules:
            rules.append(tuple([int(x) for x in data_line.strip().split('|')]))
        else:
            updates.append([int(x) for x in data_line.strip().split(',')])

    return rules, updates


def run_puzzles(data):
    import time

    rules, updates = parse_data(data)
    start_time = time.time()
    answer1 = part1(rules, updates)
    print(f"DAY 05, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = part2(rules, updates)
    print(f"DAY 05, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
