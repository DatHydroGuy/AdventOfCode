use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    let mut results = vec![];
    for init_vec in result {
        let diffs = create_difference_pyramid(init_vec);
        let answer = backtrack_difference_pyramid(diffs);
        results.push(answer);
    }
    results.iter().sum::<i32>().to_string()
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;
    Ok(("", result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, result) = separated_list1(space1, parse_number)(input)?;
    Ok((input, result))
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (input, result) = opt(tag("-"))(input)?;
    let mut factor = 1;
    if result.is_some() {
        factor = -1;
    }
    let (input, result) = digit1(input)?;
    let result = result.parse::<i32>().unwrap() * factor;
    Ok((input, result))
}

fn create_difference_pyramid(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut difference_pyramid = vec![input];
    loop {
        let last_vec = difference_pyramid.last().unwrap();
        let mut new_vec = vec![];
        for (idx, &diffs) in last_vec.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            new_vec.push(diffs - last_vec[idx - 1]);
        }
        difference_pyramid.push(new_vec.clone());
        if new_vec.iter().all(|&x| x == 0) {
            break;
        }
    }
    difference_pyramid
}

fn backtrack_difference_pyramid(mut input: Vec<Vec<i32>>) -> i32 {
    input.reverse();
    let mut result = 0;
    input.iter().for_each(|x| {
        result += x.last().unwrap();
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = String::from("114");
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backtrack_difference_pyramid() {
        let expected = 18;
        let result = backtrack_difference_pyramid(vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_difference_pyramid() {
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        let result = create_difference_pyramid(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines1() {
        let expected = Ok((
            "",
            vec![vec![0, 3, 6, 9, 12, 15], vec![-3, 0, 3, 6, 9, 12, 15]],
        ));
        let result = parse_lines(
            "0 3 6 9 12 15
-3 0 3 6 9 12 15",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line1() {
        let expected = Ok(("", vec![0, 3, 6, 9, 12, 15]));
        let result = parse_line("0 3 6 9 12 15");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = Ok(("", vec![-3, 0, 3, 6, 9, 12, 15]));
        let result = parse_line("-3 0 3 6 9 12 15");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_number1() {
        let expected = Ok(("", 3));
        let result = parse_number("3");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_number2() {
        let expected = Ok(("", -3));
        let result = parse_number("-3");
        assert_eq!(result, expected);
    }
}
