use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    const VOID_SIZE: usize = 1000000;
    let raw_data = parse_lines(input).unwrap().1;
    let grid = expand_universe(raw_data);
    let galaxy_positions = get_galaxy_positions(&grid, VOID_SIZE);
    let result = sum_galaxy_distances(galaxy_positions);
    result.to_string()
}

fn sum_galaxy_distances(galaxy_positions: Vec<(usize, usize)>) -> u64 {
    let mut sum_total = 0;
    for (i, galaxy_position1) in galaxy_positions.iter().enumerate() {
        for (j, galaxy_position2) in galaxy_positions.iter().enumerate() {
            if j > i {
                let y_diff: i64 = (galaxy_position1.0 as i64 - galaxy_position2.0 as i64).abs();
                let x_diff: i64 = (galaxy_position1.1 as i64 - galaxy_position2.1 as i64).abs();
                sum_total += (x_diff + y_diff) as u64
            }
        }
    }
    sum_total
}

fn get_galaxy_positions(grid: &Vec<Vec<&str>>, void_size: usize) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = vec![];
    let mut y_voids = 0usize;
    for y in 0..grid.len() {
        if grid[y][0] == "x" {
            y_voids += void_size - 1
        }
        let mut x_voids = 0usize;
        for x in 0..grid[0].len() {
            if grid[0][x] == "x" {
                x_voids += void_size - 1
            }
            if grid[y][x] == "#" {
                positions.push((y + y_voids, x + x_voids));
            }
        }
    }
    positions
}

fn expand_universe(mut raw_data: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let new_row = vec!["x"; raw_data[0].len()];
    for row in (0..raw_data.len()).rev() {
        if raw_data[row].iter().all(|&x| x == ".") {
            raw_data[row] = new_row.clone();
        }
    }

    for col in (0..raw_data[0].len()).rev() {
        if raw_data.iter().all(|r| ".x".contains(r[col])) {
            for raw_datum in &mut raw_data {
                raw_datum[col] = "x";
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
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, "1030"); // void_size of 10: answer = 1030, void_size of 100: answer = 8410
    }

    #[test]
    fn test_get_galaxy_positions100() {
        let expected = vec![
            (0, 102),
            (1, 205),
            (2, 0),
            (103, 204),
            (104, 1),
            (105, 306),
            (206, 205),
            (207, 0),
            (207, 103),
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
        let galaxy_positions = get_galaxy_positions(&grid, 100);
        assert_eq!(galaxy_positions, expected);
    }

    #[test]
    fn test_get_galaxy_positions10() {
        let expected = vec![
            (0, 12),
            (1, 25),
            (2, 0),
            (13, 24),
            (14, 1),
            (15, 36),
            (26, 25),
            (27, 0),
            (27, 13),
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
        let galaxy_positions = get_galaxy_positions(&grid, 10);
        assert_eq!(galaxy_positions, expected);
    }

    #[test]
    fn test_expand_universe() {
        let expected = vec![
            vec![".", ".", "x", "#", ".", "x", ".", ".", "x", "."],
            vec![".", ".", "x", ".", ".", "x", ".", "#", "x", "."],
            vec!["#", ".", "x", ".", ".", "x", ".", ".", "x", "."],
            vec!["x", "x", "x", "x", "x", "x", "x", "x", "x", "x"],
            vec![".", ".", "x", ".", ".", "x", "#", ".", "x", "."],
            vec![".", "#", "x", ".", ".", "x", ".", ".", "x", "."],
            vec![".", ".", "x", ".", ".", "x", ".", ".", "x", "#"],
            vec!["x", "x", "x", "x", "x", "x", "x", "x", "x", "x"],
            vec![".", ".", "x", ".", ".", "x", ".", "#", "x", "."],
            vec!["#", ".", "x", ".", "#", "x", ".", ".", "x", "."],
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
