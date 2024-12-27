import argparse
import os
import shutil


def copy_template_folder(day):
    template_path = os.path.join(os.path.dirname(__file__), 'day0x')
    new_path = os.path.join(os.path.dirname(__file__), f'day{day:02}')
    print(f'Copying {template_path} to {new_path}')
    shutil.copytree(template_path, new_path)
    return new_path


def rename_test_file(new_path, day_number):
    old_name = os.path.join(new_path, 'test_day0x.py')
    new_name = os.path.join(new_path, f'test_day{day_number:02}.py')
    print(f'Renaming {old_name} to {new_name}')
    os.rename(old_name, new_name)


def update_file(new_path, day_number, file_name):
    full_file_name = os.path.join(new_path, file_name)
    print(f'Updating {full_file_name}')

    # Read in the file
    with open(full_file_name, 'r') as file:
        filedata = file.read()

    # Replace the target string
    filedata = filedata.replace('0x', f'{day_number:02}')

    # Write the file out again
    with open(full_file_name, 'w') as file:
        file.write(filedata)


TEMPLATE = """import day0x


def run_aoc2024():
    day0x.puzzles.run_puzzles(read_file("day0x\\\\test_data.txt"))


def read_file(filename):
    with open(filename, "r") as f:
        data = f.readlines()
    return data


if __name__ == '__main__':
    run_aoc2024()
"""


def update_main_file(day_number):
    main_path = os.path.join(os.path.dirname(__file__), 'main.py')
    print(f'Updating main.py')

    # Replace the target string
    filedata = TEMPLATE.replace('0x', f'{day_number:02}')

    # Write the file out again
    with open(main_path, 'w') as file:
        file.write(filedata)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('day', type=int)
    args = parser.parse_args()
    print(f'Creating Advent Of Code 2024 Day{args.day:02}')
    new_path = copy_template_folder(args.day)
    rename_test_file(new_path, args.day)
    update_file(new_path, args.day, '__init__.py')
    update_file(new_path, args.day, 'puzzles.py')
    update_main_file(args.day)
