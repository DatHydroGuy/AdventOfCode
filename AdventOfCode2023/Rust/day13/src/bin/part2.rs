use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0};
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let blocks = parse_lines(input).unwrap().1;
    let mut answer = 0;
    for mut block in blocks {
        let block_width = block[0].len();
        let block_height = block.len();
        let block_score = score_block(&block);
        let old_answer;
        if block_score.len() == 0 {
            old_answer = 0;
        } else {
            old_answer = block_score[0];
        }
        let mut new_answer: Vec<u32> = vec![];
        for row in 0..block_height {
            for col in 0..block_width {
                flip_character(&mut block, row, col);
                let block_scores: Vec<_> = score_block(&block);
                if block_scores.len() > 0 {
                    for block_score in block_scores {
                        if !new_answer.contains(&block_score) {
                            new_answer.push(block_score);
                        }
                    }
                }
                flip_character(&mut block, row, col);
            }
        }

        if new_answer.len() > 0 {
            answer += new_answer
                .iter()
                .filter(|&x| *x != old_answer)
                .collect::<Vec<&u32>>()[0];
        } else {
            answer += 0;
        }
    }
    answer.to_string()
}

fn flip_character(block: &mut Vec<String>, row: usize, col: usize) {
    if block[row].chars().nth(col).unwrap() == '.' {
        block[row].replace_range(col..(col + 1), "#");
    } else {
        block[row].replace_range(col..(col + 1), ".");
    }
}

fn score_block(block: &Vec<String>) -> Vec<u32> {
    let mut block_scores = vec![];
    let v = look_for_vertical_symmetry(&block);
    if v.len() > 0 {
        block_scores.extend(v.iter().map(|x| x + 1));
    }
    let h = look_for_horizontal_symmetry(&block);
    if h.len() > 0 {
        block_scores.extend(h.iter().map(|x| (x + 1) * 100));
    }
    block_scores
}

fn look_for_vertical_symmetry(block: &Vec<String>) -> Vec<u32> {
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

fn look_for_horizontal_symmetry(block: &Vec<String>) -> Vec<u32> {
    let mut lines_of_symmetry = vec![];
    let mut curr_pal_index;
    // edge case
    if block[0] == block[1] {
        lines_of_symmetry.push(0); // we'll be adding 1 to this later...
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
                            lines_of_symmetry.push(curr_pal_index as u32);
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
        lines_of_symmetry.push(block.len() as u32 - 2); // we'll be adding 1 to this later...
    }
    lines_of_symmetry
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
    fn test_part2() {
        let expected = 400.to_string();
        let result = part2(
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
    fn test_part2_1() {
        let expected = 300.to_string();
        let result = part2(
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
    fn test_part2_2() {
        let expected = 100.to_string();
        let result = part2(
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
    fn test_part2_3() {
        let expected = 2.to_string();
        let result = part2(
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
    fn test_part2_4() {
        let expected = 2.to_string();
        let result = part2(
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
    fn test_part2_5() {
        let expected = 700.to_string();
        let result = part2(
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
    fn test_part2_6() {
        let expected = 1400.to_string(); // 5 -> 300, 400 -> 100, 4 -> 400, 300 -> 600,
        let result = part2(
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
    fn test_part2_7() {
        let expected = 5.to_string();
        let result = part2(
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
    fn test_part2_8() {
        let expected = 10.to_string();
        let result = part2(
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
    fn test_part2_9() {
        let expected = 700.to_string();
        let result = part2(
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
    fn test_part2_10() {
        let expected = 700.to_string();
        let result = part2(
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
    fn test_part2_11() {
        let expected = 0.to_string();
        let result = part2(
            "#.##..##.
..#.##.#.
##..#...#
##......#
..#..#.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_12() {
        let expected = 200.to_string();
        let result = part2(
            "###.#.#...#.###
..##.#####.###.
..##.#####.###.
###.#.....#.###
###..#.#...####
###....##..##..
###..#.###..##.
...#.###.##..#.
#######...##.##
##.#..#.#..#.##
###..#.#..#....",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_13() {
        let expected = 1200.to_string();
        let result = part2(
            "#.#.#.#....
..##.##....
#.###.#..##
#.###.#..##
..##.##....
#.#.#.#....
...###.##.#
#..##...#..
.##.##..##.
.######.#..
#...#..####
#..#.#..##.
#..#.#..##.
#...#..##.#
.######.#..",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_14() {
        let expected = 10.to_string();
        let result = part2(
            ".####.###..##
#.##.#.#.##.#
.####........
#.##.####..##
#.##.###....#
#######..##..
.#..#........
..##..#..##..
......#..##..
#....##.####.
#.##.###.##.#
##..##.#....#
.####...#..#.
##..##.....#.
#....##.####.
.#..#.#.#..#.
##..##..#..#.",
        );
        assert_eq!(result, expected);
    }
}
