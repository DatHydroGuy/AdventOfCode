use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Default, PartialEq)]
enum CameFrom {
    N,
    E,
    S,
    W,
    #[default]
    Nothing,
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (grid_width, result) = parse_lines(input).unwrap().1;
    let (grid, start_pos) = build_grid(result, grid_width);
    let count = traverse_grid(grid, start_pos);

    count.to_string()
}

fn traverse_grid(grid: Vec<Vec<&str>>, start_pos: (usize, usize)) -> u32 {
    let mut curr_x = start_pos.0;
    let mut curr_y = start_pos.1;
    let mut counter = 0;
    let mut came_from = CameFrom::Nothing;

    loop {
        // println!("{curr_x}, {curr_y}");
        if curr_x == start_pos.0 && curr_y == start_pos.1 && counter > 0 {
            break;
        } else if grid[curr_y][curr_x] == "|" {
            // only look N and S
            if "|7F".contains(grid[curr_y - 1][curr_x]) && came_from != CameFrom::N {
                curr_y -= 1;
                came_from = CameFrom::S;
            } else if "|JL".contains(grid[curr_y + 1][curr_x]) && came_from != CameFrom::S {
                curr_y += 1;
                came_from = CameFrom::N;
            }
        } else if grid[curr_y][curr_x] == "-" {
            // only look E and W
            if "-J7".contains(grid[curr_y][curr_x + 1]) && came_from != CameFrom::E {
                curr_x += 1;
                came_from = CameFrom::W;
            } else if "-LF".contains(grid[curr_y][curr_x - 1]) && came_from != CameFrom::W {
                curr_x -= 1;
                came_from = CameFrom::E;
            }
        } else if grid[curr_y][curr_x] == "L" {
            // only look N and E
            if "|7F".contains(grid[curr_y - 1][curr_x]) && came_from != CameFrom::N {
                curr_y -= 1;
                came_from = CameFrom::S;
            } else if "-J7".contains(grid[curr_y][curr_x + 1]) && came_from != CameFrom::E {
                curr_x += 1;
                came_from = CameFrom::W;
            }
        } else if grid[curr_y][curr_x] == "F" {
            // only look E and S
            if "-J7".contains(grid[curr_y][curr_x + 1]) && came_from != CameFrom::E {
                curr_x += 1;
                came_from = CameFrom::W;
            } else if "|JL".contains(grid[curr_y + 1][curr_x]) && came_from != CameFrom::S {
                curr_y += 1;
                came_from = CameFrom::N;
            }
        } else if grid[curr_y][curr_x] == "7" {
            // only look S and W
            if "|JL".contains(grid[curr_y + 1][curr_x]) && came_from != CameFrom::S {
                curr_y += 1;
                came_from = CameFrom::N;
            } else if "-LF".contains(grid[curr_y][curr_x - 1]) && came_from != CameFrom::W {
                curr_x -= 1;
                came_from = CameFrom::E;
            }
        } else if grid[curr_y][curr_x] == "J" {
            // only look W and N
            if "-LF".contains(grid[curr_y][curr_x - 1]) && came_from != CameFrom::W {
                curr_x -= 1;
                came_from = CameFrom::E;
            } else if "|7F".contains(grid[curr_y - 1][curr_x]) && came_from != CameFrom::N {
                curr_y -= 1;
                came_from = CameFrom::S;
            }
        }
        counter += 1;
    }

    counter / 2
}

fn parse_lines(input: &str) -> IResult<&str, (usize, Vec<&str>)> {
    let grid_width = match input.find('\r') {
        None => input.find('\n').unwrap(),
        Some(x) => x,
    };
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

fn build_grid(char_array: Vec<&str>, grid_width: usize) -> (Vec<Vec<&str>>, (usize, usize)) {
    let grid_chars: Vec<_> = char_array.chunks(grid_width).collect();
    let grid_height = grid_chars.len() - 1;

    let mut grid2: Vec<Vec<&str>> = vec![];

    for &row in grid_chars.iter() {
        let mut grid_row: Vec<&str> = vec![];
        for &cell in row.iter() {
            grid_row.push(cell);
        }
        grid2.push(grid_row);
    }

    let mut grid: Vec<Vec<&str>> = vec![];
    let mut start_pos = (0, 0);

    for (iy, row) in grid2.iter().enumerate() {
        let mut grid_row: Vec<&str> = vec![];
        for (ix, &cell) in row.iter().enumerate() {
            if cell == "S" {
                start_pos = (ix, iy);
                let mut possibles = vec!["|", "-", "L", "F", "J", "7"];

                if start_pos.1 == 0
                    || (start_pos.1 > 0 && "-LJ.".contains(grid2[start_pos.1 - 1][start_pos.0]))
                {
                    "|LJ".chars().for_each(|c| {
                        match possibles.iter().position(|&x| x == c.to_string()) {
                            None => {}
                            Some(index) => {
                                possibles.remove(index);
                            }
                        };
                    });
                }
                if start_pos.1 == grid_height
                    || (start_pos.1 < grid_height
                        && "-7F.".contains(grid2[start_pos.1 + 1][start_pos.0]))
                {
                    "|7F".chars().for_each(|c| {
                        match possibles.iter().position(|&x| x == c.to_string()) {
                            None => {}
                            Some(index) => {
                                possibles.remove(index);
                            }
                        };
                    });
                }
                if start_pos.0 == 0
                    || (start_pos.0 > 0 && "|7J.".contains(grid2[start_pos.1][start_pos.0 - 1]))
                {
                    "-7J".chars().for_each(|c| {
                        match possibles.iter().position(|&x| x == c.to_string()) {
                            None => {}
                            Some(index) => {
                                possibles.remove(index);
                            }
                        };
                    });
                }
                if start_pos.0 == grid_width
                    || (start_pos.0 < grid_width
                        && "|FL.".contains(grid2[start_pos.1][start_pos.0 + 1]))
                {
                    "-FL".chars().for_each(|c| {
                        match possibles.iter().position(|&x| x == c.to_string()) {
                            None => {}
                            Some(index) => {
                                possibles.remove(index);
                            }
                        };
                    });
                }
                grid_row.push(possibles.first().unwrap());
            } else {
                grid_row.push(cell);
            }
        }
        grid.push(grid_row);
    }

    (grid, start_pos)
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
        assert_eq!(result, "4");
    }

    #[test]
    fn test_template2() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn test_build_grid() {
        let input = "L7|
FS|
L7L";

        let (grid_width, result) = parse_lines(input).unwrap().1;
        let result = build_grid(result, grid_width);
        assert_eq!(result.0[1][1], "J");
    }
}
