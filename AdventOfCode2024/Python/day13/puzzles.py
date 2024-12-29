from sympy import Eq, solve
from sympy.parsing.sympy_parser import parse_expr, standard_transformations, implicit_multiplication_application


def part1(games):
    a_cost = 3
    b_cost = 1
    total = 0
    for game in games:
        target_x = game['Prize'][0]
        target_y = game['Prize'][1]
        for a_press in range(100):
            for b_press in range(100):
                if a_press * game['ButtonA'][0] + b_press * game['ButtonB'][0] == target_x and a_press * game['ButtonA'][1] + b_press * game['ButtonB'][1] == target_y:
                    total += a_press * a_cost + b_press * b_cost
                    break
    return total


def part2(games):
    a_cost = 3
    b_cost = 1
    total = 0
    for game in games:
        target_x = game['Prize'][0] + 10000000000000
        target_y = game['Prize'][1] + 10000000000000
        eqs = [f"{game['ButtonA'][0]}a + {game['ButtonB'][0]}b = {target_x}", f"{game['ButtonA'][1]}a + {game['ButtonB'][1]}b = {target_y}"]
        transformations = (standard_transformations + (implicit_multiplication_application,))
        eqs_sympy = [Eq(parse_expr(e.split('=')[0], transformations=transformations),
                        parse_expr(e.split('=')[1], transformations=transformations))
                     for e in eqs]
        sol = solve(eqs_sympy)
        presses = []
        for s in sol:
            if sol[s].denominator == 1:
                presses.append(sol[s].numerator)
        if len(presses) == 2:
            total += presses[0] * a_cost + presses[1] * b_cost

    return total


def process_button(data):
    data = data.strip()
    _, distances = data.split(': ')
    x_dist, y_dist = distances.split(', ')
    _, x = x_dist.split('+')
    _, y = y_dist.split('+')
    return int(x), int(y)


def process_prize(data):
    data = data.strip()
    _, distances = data.split(': ')
    x_dist, y_dist = distances.split(', ')
    _, x = x_dist.split('=')
    _, y = y_dist.split('=')
    return int(x), int(y)


def parse_data(raw_data):
    games = []
    game = {'ButtonA': (), 'ButtonB': (), 'Prize': ()}
    for data_line in raw_data:
        if data_line.startswith('Button A'):
            game['ButtonA'] = process_button(data_line)
        elif data_line.startswith('Button B'):
            game['ButtonB'] = process_button(data_line)
        elif data_line.startswith('Prize'):
            game['Prize'] = process_prize(data_line)
        else:
            games.append(game)
            game = {'ButtonA': (), 'ButtonB': (), 'Prize': ()}
    games.append(game)
    return games


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 13, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    parsed_data2 = parse_data(data)
    start_time = time.time()
    answer2 = part2(parsed_data2)
    print(f"DAY 13, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
