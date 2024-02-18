use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (rock_pos, cube_pos, rows) = parse_lines(input).unwrap().1;
    let final_positions = move_rocks(rock_pos, cube_pos);
    let result = sum_weights(final_positions, rows);

    result.to_string()
}

fn sum_weights(positions: Vec<(usize, usize)>, num_rows: usize) -> u32 {
    positions
        .iter()
        .map(|&pos| (num_rows - pos.0) as u32)
        .sum::<u32>()
}

fn move_rocks(rocks: Vec<(usize, usize)>, cubes: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut final_pos = vec![];
    for rock in rocks {
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

fn parse_lines(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<(usize, usize)>, usize)> {
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

    Ok(("", (rocks, cubes, grid.len())))
}

fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, result) = many1(alt((tag("."), tag("O"), tag("#"))))(input)?;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = "136".to_string();
        let result = part1(
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
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected: (Vec<(usize, usize)>, Vec<(usize, usize)>, usize) = (
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
}
