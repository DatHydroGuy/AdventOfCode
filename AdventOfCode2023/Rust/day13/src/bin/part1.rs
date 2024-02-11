use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0};
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let blocks = parse_lines(input).unwrap().1;
    let mut vert_count = 0;
    let mut horiz_count = 0;
    for block in blocks {
        if let Some(v) = look_for_vertical_symmetry(&block) {
            vert_count += v + 1;
        } else if let Some(h) = look_for_horizontal_symmetry(&block) {
            horiz_count += h + 1;
        }
    }
    (100 * horiz_count + vert_count).to_string()
}

fn look_for_vertical_symmetry(block: &Vec<String>) -> Option<u32> {
    // Transpose block then perform look_for_horizontal_symmetry
    let block_t = transpose_block(block);
    look_for_horizontal_symmetry(&block_t)
}

fn transpose_block(block: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for i in 0..block[0].len() {
        let mut temp = String::new();
        block
            .iter()
            .for_each(|x| temp.push(x.chars().nth(i).unwrap()));
        result.push(temp);
    }
    result
}

fn look_for_horizontal_symmetry(block: &Vec<String>) -> Option<u32> {
    let mut curr_pal_index;
    // edge case
    if block[0] == block[1] {
        return Some(0); // we'll be adding 1 to this later...
    }
    for idx in 1..block.len() {
        if block[idx - 1] == block[idx] {
            // found a potential center for our palindrome
            curr_pal_index = idx - 1;
            for sub_idx in 1..idx {
                if idx + sub_idx < block.len() {
                    if block[idx - 1 - sub_idx] == block[idx + sub_idx] {
                        // only return valid answer if the reflection reaches the edge of the grid
                        if idx - 1 - sub_idx == 0 || idx + sub_idx == block.len() - 1 {
                            return Some(curr_pal_index as u32);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    // edge case
    if block[block.len() - 1] == block[block.len() - 2] {
        return Some(block.len() as u32 - 2); // we'll be adding 1 to this later...
    }
    None
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<String>>> {
    let (_, result) = many1(parse_block)(input)?;
    Ok(("", result))
}

fn parse_block(input: &str) -> IResult<&str, Vec<String>> {
    let (input, result) = separated_list1(line_ending, parse_line)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, String> {
    let (input, result) = many1(alt((tag("."), tag("#"))))(input)?;
    let result_vec = result.join("");
    Ok((input, result_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let expected = Ok(("", "#.##..##.".to_string()));
        let result = parse_line("#.##..##.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block() {
        let expected = Ok((
            "",
            vec![
                "#.##..##.".to_string(),
                "..#.##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ],
        ));
        let result = parse_block(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = Ok((
            "",
            vec![
                vec![
                    "#.##..##.".to_string(),
                    "..#.##.#.".to_string(),
                    "##......#".to_string(),
                    "##......#".to_string(),
                    "..#.##.#.".to_string(),
                    "..##..##.".to_string(),
                    "#.#.##.#.".to_string(),
                ],
                vec![
                    "#...##..#".to_string(),
                    "#....#..#".to_string(),
                    "..##..###".to_string(),
                    "#####.##.".to_string(),
                    "#####.##.".to_string(),
                    "..##..###".to_string(),
                    "#....#..#".to_string(),
                ],
            ],
        ));
        let result = parse_lines(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let expected = 405.to_string();
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_1() {
        let expected = 5.to_string();
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_2() {
        let expected = 400.to_string();
        let result = part1(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_3() {
        let expected = 100.to_string();
        let result = part1(
            "....##.....##.###
....##.....##.###
.##.###.####....#
..#.....#.....#..
#..###.#.....#..#
#..##...#....##.#
#..#.#...####.#.#
#..##.#.#..#.##..
.##.......#######
.##.#...##.#..#..
.##.#...#.####...
....#.#.#.#.#..#.
.......#.#...####
....###..#.#...##
....#....#..#...#
.##..#.##...#.##.
######.##.#..#...",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_4() {
        let expected = 9.to_string();
        let result = part1(
            "####.########.#
#####.#.##.#.##
##.#..........#
#####..####..##
....#.##..##.#.
....##.####.##.
#..##.#....#.##
.##...#....#...
#..#.#..##..#.#",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_5() {
        let expected = 1400.to_string();
        let result = part1(
            "##.##.####.##.#..
..#.#..#..##.#..#
.###...###.##.#..
.#..#.###.#....#.
###.##.#...##.##.
#####.##.###..###
#.##..#.#.#.#.#..
#.##..#.#.#.#.#..
#####.##.###..###
###.##.#...##.##.
.#.##.###.#....#.
.###...###.##.#..
..#.#..#..##.#..#
##.##.####.##.#..
##.##.####.##.#..",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_6() {
        let expected = 709.to_string();
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_7() {
        let expected = 1.to_string();
        let result = part1(
            "###.##.##
##.####.#
##.#..#.#
####..###
....##...
##.#..#.#
...#..#..
##..###.#
##......#
##......#
..#.##.#.
...#..#..
##.####.#
....##...
...####..
....##...
##.####.#",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_8() {
        let expected = 2.to_string();
        let result = part1(
            ".##.##...##...##.
#####..##..##..##
.....##..##..##..
.##.#.#.####.#.#.
.##...#.#..#.#...
....#..........#.
#..#..#......#..#
....###.....####.
.##...#.#..#.#...
.....#..####..#..
#..#...##..##...#
....#...#..#...#.
#..#.##########.#
#..##...####...##
#####.##.##.##.##",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_9() {
        let expected = 1600.to_string();
        let result = part1(
            "####.#.#......#
#.####.#..##.#.
###.###.###...#
####.#.##....##
.###..#.##..###
#....#....###.#
#...###..#.....
##..###..#.....
#....#....###.#
.###..#.##..###
####.#.##....##
###.###.###...#
#.####.#..##.#.
####.#.#......#
#...#.#.#..#.#.
#.##.##.#....##
#.##.##.#....##",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_10() {
        let expected = 1400.to_string();
        let result = part1(
            "##.##.####.##.#..
..#.#..#..##.#..#
.###...###.##.#..
.#..#.###.#....#.
###.##.#...##.##.
#####.##.###..###
#.##..#.#.#.#.#..
#.##..#.#.#.#.#..
#####.##.###..###
###.##.#...##.##.
.#.##.###.#....#.
.###...###.##.#..
..#.#..#..##.#..#
##.##.####.##.#..
##.##.####.##.#..",
        );
        assert_eq!(result, expected);
    }
}
