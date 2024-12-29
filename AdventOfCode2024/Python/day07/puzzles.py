def get_operator_combinations(num_operators, number_base):
    operator_combinations_length = pow(number_base, num_operators - 1)
    operator_combinations = []
    for i in range(operator_combinations_length):
        num_in_new_base = number_to_base(i, number_base)
        joined = ''.join([str(c) for c in num_in_new_base])
        operator_combinations.append(joined.zfill(num_operators - 1))
    operator_combinations = [[c for c in s.replace('0', '+').replace('1', '*').replace('2', '|')] for s in operator_combinations]
    return operator_combinations


def number_to_base(n, b):
    if n == 0:
        return [0]
    digits = []
    while n:
        digits.append(int(n % b))
        n //= b
    return digits[::-1]


def evaluate_equation(equation):
    total = equation[0]
    number_of_operations = int((len(equation) - 1) / 2)
    for element in range(number_of_operations):
        next_step = equation[(element + 1) * 2 - 1: (element + 1) * 2 + 1]
        if next_step[0] == '*':
            total *= next_step[1]
        elif next_step[0] == '+':
            total += next_step[1]
        elif next_step[0] == '|':
            total = total * pow(10, len(str(next_step[1]))) + next_step[1]
    return total


def run(data, number_of_operators):
    total = 0
    for data_line in data:
        result, values = data_line
        operator_combinations = get_operator_combinations(len(values), number_of_operators)
        for operator_combination in operator_combinations:
            rolling_sum = [int(values[0])]
            for op_idx, operator in enumerate(operator_combination, start=1):
                rolling_sum.append(operator)
                rolling_sum.append(int(values[op_idx]))
            if evaluate_equation(rolling_sum) == int(result):
                total += int(result)
                break
    return total


def parse_data(raw_data):
    first_parse = [r.strip().split(': ') for r in raw_data]
    for eq_idx, equation in enumerate(first_parse):
        test_value, numbers = equation
        numbers = numbers.split(' ')
        first_parse[eq_idx] = [test_value, numbers]
    return first_parse


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = run(parsed_data, 2)
    print(f"DAY 07, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = run(parsed_data, 3)
    print(f"DAY 07, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
