use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::separated_list1;

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
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;

    let winners = result.iter().sum();
    Ok(("", winners))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
	todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let result = part1("");
        assert_eq!(result, "");
    }
}
