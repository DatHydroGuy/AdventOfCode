use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug)]
struct Cell {
    character: char,
    x_pos: usize,
    y_pos: usize,
    exit_north: Option<bool>,
    exit_east: Option<bool>,
    exit_south: Option<bool>,
    exit_west: Option<bool>,
    entered_from: Option<char>,
}

impl Cell {
    fn new(character: char, x_pos: usize, y_pos: usize) -> Self {
        let mut en = None;
        let mut ee = None;
        let mut es = None;
        let mut ew = None;
        if character == '-' {
            ee = Some(false);
            ew = Some(false);
        } else if character == '|' {
            en = Some(false);
            es = Some(false);
        } else if character == 'L' {
            en = Some(false);
            ee = Some(false);
        } else if character == 'F' {
            ee = Some(false);
            es = Some(false);
        } else if character == '7' {
            ew = Some(false);
            es = Some(false);
        } else if character == 'J' {
            en = Some(false);
            ew = Some(false);
        }
        Self {
            character,
            x_pos,
            y_pos,
            exit_north: en,
            exit_east: ee,
            exit_south: es,
            exit_west: ew,
            entered_from: None,
        }
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (grid_width, result) = parse_lines(input).unwrap().1;
    let grid = build_grid(result, grid_width);

    dbg!(grid);

    //result.to_string()
    String::from("0")
}

fn parse_lines(input: &str) -> IResult<&str, (usize, Vec<&str>)> {
    let grid_width = input.find('\n').unwrap();
    let (input, result) = many1(terminated(
        alt((
            tag("."),
            tag("|"),
            tag("-"),
            tag("L"),
            tag("J"),
            tag("F"),
            tag("7"),
            tag("S"),
        )),
        multispace0,
    ))(input)?;

    Ok((input, (grid_width, result)))
}

fn build_grid(char_array: Vec<&str>, grid_width: usize) -> Vec<Vec<Cell>> {
    let grid_chars: Vec<_> = char_array.chunks(grid_width).collect();

    let mut grid = vec![];
    let mut start_pos = (0, 0);

    for (iy, &row) in grid_chars.iter().enumerate() {
        let mut grid_row = vec![];
        for (ix, &cell) in row.iter().enumerate() {
            let chr = cell.chars().next().unwrap();
            if chr == 'S' {
                start_pos = (ix, iy);
            }
            grid_row.push(Cell::new(chr, ix, iy));
        }
        grid.push(grid_row);
    }

    // Need to set the exits for the cell with an 'S' in it
    if grid[start_pos.1 - 1][start_pos.0].character == '|'
        || grid[start_pos.1 - 1][start_pos.0].character == 'F'
        || grid[start_pos.1 - 1][start_pos.0].character == '7'
    {
        grid[start_pos.1][start_pos.0].exit_north = Some(false);
    }

    if grid[start_pos.1 + 1][start_pos.0].character == '|'
        || grid[start_pos.1 + 1][start_pos.0].character == 'J'
        || grid[start_pos.1 + 1][start_pos.0].character == 'L'
    {
        grid[start_pos.1][start_pos.0].exit_south = Some(false);
    }

    if grid[start_pos.1][start_pos.0 - 1].character == '-'
        || grid[start_pos.1][start_pos.0 - 1].character == 'F'
        || grid[start_pos.1][start_pos.0 - 1].character == 'L'
    {
        grid[start_pos.1][start_pos.0].exit_west = Some(false);
    }

    if grid[start_pos.1][start_pos.0 + 1].character == '-'
        || grid[start_pos.1][start_pos.0 + 1].character == '7'
        || grid[start_pos.1][start_pos.0 + 1].character == 'J'
    {
        grid[start_pos.1][start_pos.0].exit_east = Some(false);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, "");
    }

    #[test]
    fn test_template2() {
        let Some(x) = Some(7) else { panic!("WARGH!") };
        assert_eq!(x, 7);
    }
}
