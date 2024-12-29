def eval_combo_operand(operand, a, b, c):
    if 0 <= operand <= 3:
        return operand
    elif operand == 4:
        return a
    elif operand == 5:
        return b
    elif operand == 6:
        return c


def part1(a, b, c, prog, as_list=False):
    pointer = 0
    output = []
    while pointer < len(prog):
        op_code = prog[pointer]
        operand = prog[pointer + 1]

        if op_code == 0:
            combo = eval_combo_operand(operand, a, b, c)
            a = int(a / pow(2, combo))
            pointer += 2
        elif op_code == 1:
            b ^= operand
            pointer += 2
        elif op_code == 2:
            combo = eval_combo_operand(operand, a, b, c)
            b = combo % 8
            pointer += 2
        elif op_code == 3:
            if a != 0:
                pointer = operand
            else:
                pointer += 2
        elif op_code == 4:
            b ^= c
            pointer += 2
        elif op_code == 5:
            combo = eval_combo_operand(operand, a, b, c)
            value = combo % 8
            output.append(value)
            pointer += 2
        elif op_code == 6:
            combo = eval_combo_operand(operand, a, b, c)
            b = int(a / pow(2, combo))
            pointer += 2
        elif op_code == 7:
            combo = eval_combo_operand(operand, a, b, c)
            c = int(a / pow(2, combo))
            pointer += 2

        if pointer >= len(prog) * 2:
            break

    if as_list:
        return output
    else:
        return ','.join(str(x) for x in output) if len(output) > 1 else str(output[0])


def part2(_, b, c, prog):
    power = 15  # pow(8, 15) is the minimum number required to create a 16-digit output, so start from here
    digits = [0 for _ in range(power + 1)]
    backtrack_digits = [-1 for _ in range(power + 1)]   # used for when we can't get a match for all digits[]
    while power >= 0:
        match_found = False
        for digit in range(backtrack_digits[power] + 1, 8):     # start from previous attempt + 1
            a = sum(digits[i] * pow(8, i) for i in range(len(digits)))
            a += digit * pow(8, power)
            res = part1(a, b, c, prog, True)
            if len(res) > 1 and res[power] == prog[power]:
                digits[power] = digit
                backtrack_digits[power] = digit
                match_found = True
                break
        if not match_found:
            # need to reset current and previous digits to 0 so we don't ruin the calculation for a when we search from backtrack_digit + 1
            digits[power] = 0
            power += 1
            digits[power] = 0
        else:
            power -= 1
    return sum(digits[i] * pow(8, i) for i in range(len(digits)))


def parse_data(data):
    a = None
    b = None
    c = None
    prog = None
    for data_line in data:
        if data_line.startswith("Register A: "):
            a = int(data_line[12:])
        elif data_line.startswith("Register B: "):
            b = int(data_line[12:])
        elif data_line.startswith("Register C: "):
            c = int(data_line[12:])
        elif data_line.startswith("Program: "):
            prog = data_line[9:]
            prog = [int(x) for x in prog.split(",")]
    return a, b, c, prog


def run_puzzles(data):
    import time

    a, b, c, prog = parse_data(data)
    start_time = time.time()
    answer1 = part1(a, b, c, prog)
    print(f"DAY 17, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    a, b, c, prog = parse_data(data)
    start_time = time.time()
    answer2 = part2(a, b, c, prog)
    print(f"DAY 17, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
