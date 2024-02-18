use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (rock_pos, cube_pos, rows, columns) = parse_lines(input).unwrap().1;
    let final_positions = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 1000000000);
    let result = sum_weights(final_positions, rows);

    result.to_string()
}

fn multi_tilt_n_times(
    rock_pos: &Vec<(usize, usize)>,
    cube_pos: &Vec<(usize, usize)>,
    rows: usize,
    columns: usize,
    num_times: u64,
) -> Vec<(usize, usize)> {
    let mut old_configs: Vec<Vec<(usize, usize)>> = vec![vec![(0usize, 0usize)]];
    let mut result = multi_tilt(&rock_pos, &cube_pos, rows, columns);
    if num_times == 1 {
        return result;
    }
    old_configs.push(result.clone());
    let cycle_length: u64;
    let cycle_0_index: u64;
    for n in 2..=num_times {
        result = multi_tilt(&result, &cube_pos, rows, columns);
        if old_configs.contains(&result) {
            cycle_0_index = old_configs.iter().position(|x| x == &result).unwrap() as u64;
            cycle_length = n - cycle_0_index;
            let repeating = num_times - cycle_0_index;
            let cycle_index = repeating % cycle_length;
            let index = cycle_index + cycle_0_index;
            result = old_configs[index as usize].clone();
            break;
        } else {
            old_configs.push(result.clone());
        }
    }
    result
}

fn multi_tilt(
    rock_pos: &Vec<(usize, usize)>,
    cube_pos: &Vec<(usize, usize)>,
    rows: usize,
    columns: usize,
) -> Vec<(usize, usize)> {
    let north_positions = tilt_north(&rock_pos, &cube_pos);
    let west_positions = tilt_west(north_positions, &cube_pos);
    let south_positions = tilt_south(west_positions, &cube_pos, rows);
    let mut east_positions = tilt_east(south_positions, &cube_pos, columns);
    east_positions.sort();
    east_positions
}

fn sum_weights(positions: Vec<(usize, usize)>, num_rows: usize) -> u64 {
    positions
        .iter()
        .map(|&pos| (num_rows - pos.0) as u64)
        .sum::<u64>()
}

fn tilt_north(rocks: &Vec<(usize, usize)>, cubes: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut final_pos = vec![];
    for rock in rocks.clone() {
        if rock.0 == 0 {
            final_pos.push(rock);
            // println!("{:?} -> {:?}", rock, rock);
        } else {
            for y in (0..rock.0).rev() {
                let temp = (y, rock.1);
                if cubes.contains(&temp) || final_pos.contains(&temp) {
                    final_pos.push((temp.0 + 1, temp.1));
                    // println!("{:?} -> ({}, {})", rock, temp.0 + 1, temp.1);
                    break;
                } else if temp.0 == 0 {
                    final_pos.push((temp.0, temp.1));
                    // println!("{:?} -> ({}, {})", rock, temp.0, temp.1);
                    break;
                }
            }
        }
    }
    final_pos
}

fn tilt_south(
    mut rocks: Vec<(usize, usize)>,
    cubes: &Vec<(usize, usize)>,
    rows: usize,
) -> Vec<(usize, usize)> {
    let mut final_pos = vec![];
    rocks.sort();
    for &rock in rocks.iter().rev() {
        if rock.0 == rows - 1 {
            final_pos.push(rock);
            // println!("{:?} -> {:?}", rock, rock);
        } else {
            for y in rock.0..rows {
                let temp = (y, rock.1);
                if cubes.contains(&temp) || final_pos.contains(&temp) {
                    final_pos.push((temp.0 - 1, temp.1));
                    // println!("{:?} -> ({}, {})", rock, temp.0 - 1, temp.1);
                    break;
                } else if temp.0 == rows - 1 {
                    final_pos.push((temp.0, temp.1));
                    // println!("{:?} -> ({}, {})", rock, temp.0, temp.1);
                    break;
                }
            }
        }
    }
    final_pos.sort();
    final_pos.reverse();
    final_pos
}

fn tilt_west(rocks: Vec<(usize, usize)>, cubes: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut rocks_t: Vec<_> = rocks.iter().map(|&rock| (rock.1, rock.0)).collect();
    rocks_t.sort();
    let cubes_t = cubes.iter().map(|&cube| (cube.1, cube.0)).collect();
    let result_t = tilt_north(&rocks_t, &cubes_t);
    result_t
        .iter()
        .map(|&result| (result.1, result.0))
        .collect()
}

fn tilt_east(
    rocks: Vec<(usize, usize)>,
    cubes: &Vec<(usize, usize)>,
    columns: usize,
) -> Vec<(usize, usize)> {
    let mut rocks_t: Vec<_> = rocks.iter().map(|&rock| (rock.1, rock.0)).collect();
    rocks_t.sort();
    let cubes_t = cubes.iter().map(|&cube| (cube.1, cube.0)).collect();
    let result_t = tilt_south(rocks_t, &cubes_t, columns);
    result_t
        .iter()
        .map(|&result| (result.1, result.0))
        .collect()
}

fn parse_lines(
    input: &str,
) -> IResult<&str, (Vec<(usize, usize)>, Vec<(usize, usize)>, usize, usize)> {
    let (_, grid) = separated_list1(line_ending, parse_line)(input)?;

    let mut rocks = vec![];
    let mut cubes = vec![];
    for (iy, &ref y) in grid.iter().enumerate() {
        for (ix, &x) in y.iter().enumerate() {
            if x == "O" {
                rocks.push((iy, ix));
            }
            if x == "#" {
                cubes.push((iy, ix));
            }
        }
    }

    Ok(("", (rocks, cubes, grid.len(), grid[0].len())))
}

fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, result) = many1(alt((tag("."), tag("O"), tag("#"))))(input)?;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_tilt_n_times_1() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (2, 3),
            (2, 4),
            (3, 1),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (7, 6),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 4),
            (9, 3),
            (9, 4),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 1);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_n_times_2() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 9),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 3),
            (8, 4),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 9),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_n_times_3() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 9),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 4),
            (8, 9),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 9),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_17_times() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 9),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 4),
            (8, 9),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 9),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 17);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_1000000000_times() {
        let expected = 64;
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = multi_tilt_n_times(&rock_pos, &cube_pos, rows, columns, 1000000000);
        let load = sum_weights(result, rows);

        assert_eq!(load, expected);
    }

    #[test]
    fn test_multi_tilt_1() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (2, 3),
            (2, 4),
            (3, 1),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (7, 6),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 4),
            (9, 3),
            (9, 4),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = multi_tilt(&rock_pos, &cube_pos, rows, columns);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_2() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 9),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 3),
            (8, 4),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 9),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = multi_tilt(&rock_pos, &cube_pos, rows, columns);
        result = multi_tilt(&result, &cube_pos, rows, columns);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_2_and_north() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 1),
            (0, 2),
            (0, 7),
            (0, 8),
            (2, 4),
            (3, 4),
            (3, 5),
            (3, 6),
            (4, 3),
            (4, 4),
            (4, 6),
            (5, 3),
            (5, 8),
            (6, 2),
            (6, 7),
            (6, 9),
            (7, 9),
            (8, 9),
        ];
        let (rock_pos, cube_pos, _rows, _columns) = parse_lines(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        )
        .unwrap()
        .1;
        let mut result = tilt_north(&rock_pos, &cube_pos);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_2_and_north_west() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 6),
            (0, 7),
            (2, 0),
            (3, 4),
            (3, 5),
            (3, 6),
            (4, 0),
            (4, 1),
            (4, 2),
            (5, 3),
            (5, 8),
            (6, 0),
            (6, 6),
            (6, 7),
            (7, 0),
            (8, 8),
        ];
        let (rock_pos, cube_pos, _rows, _columns) = parse_lines(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        )
        .unwrap()
        .1;
        let mut result = tilt_north(&rock_pos, &cube_pos);
        result = tilt_west(result, &cube_pos);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_2_and_north_west_south() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 6),
            (3, 0),
            (4, 0),
            (4, 2),
            (4, 7),
            (5, 0),
            (5, 5),
            (6, 0),
            (6, 6),
            (7, 0),
            (7, 6),
            (7, 7),
            (8, 1),
            (8, 8),
            (9, 1),
            (9, 3),
            (9, 4),
            (9, 8),
        ];
        let (rock_pos, cube_pos, rows, _columns) = parse_lines(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        )
        .unwrap()
        .1;
        let mut result = tilt_north(&rock_pos, &cube_pos);
        result = tilt_west(result, &cube_pos);
        result = tilt_south(result, &cube_pos, rows);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_tilt_3() {
        let expected: Vec<(usize, usize)> = vec![
            (1, 8),
            (3, 2),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 9),
            (7, 7),
            (7, 8),
            (7, 9),
            (8, 4),
            (8, 9),
            (9, 2),
            (9, 3),
            (9, 4),
            (9, 9),
        ];
        let (rock_pos, cube_pos, rows, columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = multi_tilt(&rock_pos, &cube_pos, rows, columns);
        result = multi_tilt(&result, &cube_pos, rows, columns);
        result = multi_tilt(&result, &cube_pos, rows, columns);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tilt_north_west() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 6),
            (1, 0),
            (1, 1),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 7),
            (3, 0),
            (3, 4),
            (3, 5),
            (6, 0),
            (6, 6),
            (6, 7),
            (7, 0),
        ];
        let (rock_pos, cube_pos, _rows, _columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = tilt_north(&rock_pos, &cube_pos);
        result = tilt_west(result, &cube_pos);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected: (Vec<(usize, usize)>, Vec<(usize, usize)>, usize, usize) = (
            vec![
                (0, 0),
                (1, 0),
                (1, 2),
                (1, 3),
                (3, 0),
                (3, 1),
                (3, 4),
                (3, 9),
                (4, 1),
                (4, 7),
                (5, 0),
                (5, 5),
                (6, 2),
                (6, 6),
                (6, 9),
                (7, 7),
                (9, 1),
                (9, 2),
            ],
            vec![
                (0, 5),
                (1, 4),
                (1, 9),
                (2, 5),
                (2, 6),
                (3, 3),
                (4, 8),
                (5, 2),
                (5, 7),
                (5, 9),
                (6, 5),
                (8, 0),
                (8, 5),
                (8, 6),
                (8, 7),
                (9, 0),
                (9, 5),
            ],
            10usize,
            10usize,
        );
        let result = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_tilt_north() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (1, 0),
            (0, 2),
            (0, 3),
            (2, 0),
            (0, 1),
            (2, 4),
            (2, 9),
            (1, 1),
            (0, 7),
            (3, 0),
            (3, 5),
            (6, 2),
            (3, 6),
            (6, 9),
            (6, 7),
            (2, 1),
            (7, 2),
        ];
        let (rock_pos, cube_pos, _rows, _columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = tilt_north(&rock_pos, &cube_pos);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tilt_south() {
        let expected: Vec<(usize, usize)> = vec![
            (9, 9),
            (9, 4),
            (9, 2),
            (9, 1),
            (8, 2),
            (8, 1),
            (7, 7),
            (7, 6),
            (7, 1),
            (7, 0),
            (6, 0),
            (5, 5),
            (5, 0),
            (4, 9),
            (4, 7),
            (4, 2),
            (4, 0),
            (2, 3),
        ];
        let (rock_pos, cube_pos, rows, _columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let result = tilt_south(rock_pos, &cube_pos, rows);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tilt_west() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (1, 2),
            (3, 0),
            (3, 1),
            (3, 4),
            (3, 5),
            (4, 0),
            (4, 1),
            (5, 0),
            (5, 3),
            (6, 0),
            (6, 6),
            (6, 7),
            (7, 0),
            (9, 1),
            (9, 2),
        ];
        let (rock_pos, cube_pos, _rows, _columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = tilt_west(rock_pos, &cube_pos);
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tilt_east() {
        let expected: Vec<(usize, usize)> = vec![
            (0, 4),
            (1, 1),
            (1, 2),
            (1, 3),
            (3, 1),
            (3, 2),
            (3, 8),
            (3, 9),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 6),
            (6, 4),
            (6, 8),
            (6, 9),
            (7, 9),
            (9, 3),
            (9, 4),
        ];
        let (rock_pos, cube_pos, rows, _columns) = parse_lines(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
        .unwrap()
        .1;
        let mut result = tilt_east(rock_pos, &cube_pos, rows);
        result.sort();

        assert_eq!(result, expected);
    }
}
