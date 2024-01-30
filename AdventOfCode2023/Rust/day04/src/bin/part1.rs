use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    result.to_string()
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    let (_, card_scores) = separated_list1(line_ending, parse_line)(input)?;
    let result = card_scores.iter().sum();
    Ok(("", result))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, _card_num) = parse_card_number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_nums) = parse_card_numbers(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, your_nums) = parse_card_numbers(input)?;

    let win_hash: HashSet<_> = winning_nums.iter().collect();
    let your_hash: HashSet<_> = your_nums.iter().collect();
    let intersect: HashSet<_> = win_hash.intersection(&your_hash).collect();
    if intersect.len() > 0 {
        Ok((input, 2_u32.pow(intersect.len() as u32 - 1)))
    } else {
        Ok((input, 0))
    }
}

fn parse_card_number(input: &str) -> IResult<&str, u32> {
    let (input, result) = separated_list1(space1, alphanumeric1)(input)?;
    Ok((input, result[1].parse().unwrap()))
}

fn parse_card_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = separated_list1(space1, digit1)(input)?;
    let number_values = digits.iter().map(|&x| x.parse().unwrap()).collect();

    Ok((input, number_values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let expected = Ok(("", 8));
        let result = parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = Ok(("", 2));
        let result = parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = Ok(("", 2));
        let result = parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = Ok(("", 1));
        let result = parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line5() {
        let expected = Ok(("", 0));
        let result = parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line6() {
        let expected = Ok(("", 0));
        let result = parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line7() {
        let expected = Ok(("", 0));
        let result = parse_line("Card   7: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = Ok(("", 13));
        let result = parse_lines(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, expected);
    }
}
