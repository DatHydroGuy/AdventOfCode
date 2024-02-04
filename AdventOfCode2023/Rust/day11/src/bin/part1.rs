use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let raw_data = parse_lines(input).unwrap().1;
    let grid = expand_universe(raw_data);
    let galaxy_positions = get_galaxy_positions(&grid);
    let result = sum_galaxy_distances(galaxy_positions);
    result.to_string()
}

fn sum_galaxy_distances(galaxy_positions: Vec<(usize, usize)>) -> u32 {
    let mut sum_total = 0;
    for (i, galaxy_position1) in galaxy_positions.iter().enumerate() {
        for (j, galaxy_position2) in galaxy_positions.iter().enumerate() {
            if j > i {
                let y_diff: i32 = (galaxy_position1.0 as i32 - galaxy_position2.0 as i32).abs();
                let x_diff: i32 = (galaxy_position1.1 as i32 - galaxy_position2.1 as i32).abs();
                sum_total += (x_diff + y_diff) as u32
            }
        }
    }
    sum_total
}

fn get_galaxy_positions(grid: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == "#" {
                positions.push((y, x));
            }
        }
    }
    positions
}

fn expand_universe(mut raw_data: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let new_row = vec!["."; raw_data[0].len()];
    for row in (0..raw_data.len()).rev() {
        if raw_data[row].iter().all(|&x| x == ".") {
            raw_data.insert(row + 1, new_row.clone());
        }
    }

    for col in (0..raw_data[0].len()).rev() {
        if raw_data.iter().all(|r| r[col] == ".") {
            for raw_datum in &mut raw_data {
                raw_datum.insert(col + 1, ".");
            }
        }
    }

    raw_data
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let grid_width = match input.find('\r') {
        None => input.find('\n').unwrap(),
        Some(x) => x,
    };

    let chars: Vec<&str>;
    (_, chars) = many1(terminated(alt((tag("."), tag("#"))), multispace0))(input)?;

    let mut result = vec![];

    for y in 0..(chars.len() / grid_width) {
        let mut grid_row = vec![];
        for x in 0..grid_width {
            grid_row.push(chars[y * grid_width + x]);
        }
        result.push(grid_row);
    }

    Ok(("", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374");
    }

    #[test]
    fn test_get_galaxy_positions() {
        let expected = vec![
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ];
        let raw_data = parse_lines(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        )
        .unwrap()
        .1;
        let grid = expand_universe(raw_data);
        let galaxy_positions = get_galaxy_positions(&grid);
        assert_eq!(galaxy_positions, expected);
    }

    #[test]
    fn test_expand_universe() {
        let expected = vec![
            vec![
                ".", ".", ".", ".", "#", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", "#", ".", ".", ".",
            ],
            vec![
                "#", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", "#", ".", ".", ".", ".",
            ],
            vec![
                ".", "#", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "#",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ],
            vec![
                ".", ".", ".", ".", ".", ".", ".", ".", ".", "#", ".", ".", ".",
            ],
            vec![
                "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", ".", ".", ".",
            ],
        ];

        let raw_data = parse_lines(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        )
        .unwrap()
        .1;
        let result = expand_universe(raw_data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = vec![
            vec![".", ".", ".", "#", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", ".", "#", ".", "."],
            vec!["#", ".", ".", ".", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", "#", ".", ".", "."],
            vec![".", "#", ".", ".", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "#"],
            vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", ".", "#", ".", "."],
            vec!["#", ".", ".", ".", "#", ".", ".", ".", ".", "."],
        ];
        let result = parse_lines(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result.unwrap().1, expected);
    }
}
