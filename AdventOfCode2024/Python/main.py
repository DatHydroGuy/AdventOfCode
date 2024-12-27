import day25


def run_aoc2024():
    day25.puzzles.run_puzzles(read_file("day25\\data.txt"))


def read_file(filename):
    with open(filename, "r") as f:
        data = f.readlines()
    return data


if __name__ == '__main__':
    run_aoc2024()
