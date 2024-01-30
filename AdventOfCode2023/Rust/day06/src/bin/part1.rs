use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use std::iter::zip;

fn main() {
    let input = include_str!("input1.txt");
    let output: u32 = part1(input).iter().product();
    dbg!(output);
}

fn part1(input: &str) -> Vec<u32> {
    let result = parse_lines(input).unwrap().1;
    let td_pairs: Vec<(&u32, &u32)> = zip(&result[0], &result[1]).collect();
    let mut counts = vec![];
    for td_pair in td_pairs {
        let mut count = 0;
        for t in 0..=*td_pair.0 {
            if t * (td_pair.0 - t) > *td_pair.1 {
                count += 1;
            }
        }
        counts.push(count);
    }
    counts
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (_input, result) = separated_list1(line_ending, parse_line)(input)?;
    Ok(("", result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _result) = take_until(":")(input)?;
    let (input, _result) = tag(":")(input)?;
    let (input, _result) = space1(input)?;
    let (input, result) = separated_list1(space1, digit1)(input)?;
    Ok((input, result.iter().map(|&x| x.parse().unwrap()).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = vec![4, 8, 9];
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        // 288 = 4 * 8 * 9
        /*
        Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.
        In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.
        In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.
        To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).
         */
        let expected = Ok(("", vec![vec![7, 15, 30], vec![9, 40, 200]]));
        let result = parse_lines(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, expected);
    }
}
