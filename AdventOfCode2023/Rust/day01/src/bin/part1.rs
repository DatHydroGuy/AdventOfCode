use nom::character::complete::{alpha0, digit1, line_ending};
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    match parse_lines(input) {
        Ok(num) => {
            format!("{}", num.1)
        }
        Err(_) => String::from("Error: {}"),
    }
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    let (input, digit_pairs) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, digit_pairs.iter().sum()))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, digit_pairs) = many1(preceded(alpha0, digit1))(input)?;

    let digit_pair = format!(
        "{}{}",
        &digit_pairs.first().unwrap()[0..1],
        &digit_pairs.last().unwrap().chars().last().unwrap()
    );

    // Consume any remaining characters
    let (input, _) = alpha0(input)?;

    Ok((input, digit_pair.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let expected = Ok(("", 12));
        let result = parse_line("1abc2");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = Ok(("", 38));
        let result = parse_line("pqr3stu8vwx");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = Ok(("", 15));
        let result = parse_line("a1b2c3d4e5f");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = Ok(("", 77));
        let result = parse_line("treb7uchet");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line_double_number() {
        let expected = Ok(("", 75));
        let result = parse_line("7871three915");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = Ok(("", 142));
        let result = parse_lines(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, expected);
    }
}
