import day01
import day02
import day03
import day04
import day05
import day06
import day07
import day08
import day09
import day10
import day11
import day12
import day13
import day14
import day15
import day16
import day17
import day18
import day19
import day20
import day21
import day22
import day23
import day24
import day25


def run_aoc2024():
    day01.puzzles.run_puzzles(read_file("day01\\data.txt"))
    day02.puzzles.run_puzzles(read_file("day02\\data.txt"))
    day03.puzzles.run_puzzles(read_file("day03\\data.txt"))
    day04.puzzles.run_puzzles(read_file("day04\\data.txt"))
    day05.puzzles.run_puzzles(read_file("day05\\data.txt"))
    day06.puzzles.run_puzzles(read_file("day06\\data.txt"))
    day07.puzzles.run_puzzles(read_file("day07\\data.txt"))
    day08.puzzles.run_puzzles(read_file("day08\\data.txt"))
    day09.puzzles.run_puzzles(read_file("day09\\data.txt"))
    day10.puzzles.run_puzzles(read_file("day10\\data.txt"))
    day11.puzzles.run_puzzles(read_file("day11\\data.txt"))
    day12.puzzles.run_puzzles(read_file("day12\\data.txt"))
    day13.puzzles.run_puzzles(read_file("day13\\data.txt"))
    day14.puzzles.run_puzzles(read_file("day14\\data.txt"))
    day15.puzzles.run_puzzles(read_file("day15\\data.txt"))
    day16.puzzles.run_puzzles(read_file("day16\\data.txt"))
    day17.puzzles.run_puzzles(read_file("day17\\data.txt"))
    day18.puzzles.run_puzzles(read_file("day18\\data.txt"))
    day19.puzzles.run_puzzles(read_file("day19\\data.txt"))
    day20.puzzles.run_puzzles(read_file("day20\\data.txt"))
    day21.puzzles.run_puzzles(read_file("day21\\data.txt"))
    day22.puzzles.run_puzzles(read_file("day22\\data.txt"))
    day23.puzzles.run_puzzles(read_file("day23\\data.txt"))
    day24.puzzles.run_puzzles(read_file("day24\\data.txt"))
    day25.puzzles.run_puzzles(read_file("day25\\data.txt"))


def read_file(filename):
    with open(filename, "r") as f:
        data = f.readlines()
    return data


if __name__ == '__main__':
    run_aoc2024()
