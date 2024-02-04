use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    result.to_string()
}

fn count_consecutives(input: &str) -> Vec<u32> {}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;
    dbg!(result);

    let winners = 0; //result.iter().sum();
    Ok(("", winners))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<u32>)> {
    // let (input, springs) = many1(alt((tag("."), tag("#"), tag("?"))))(input)?;
    let (input, springs) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (input, damaged) = separated_list1(tag(","), digit1)(input)?;
    Ok((
        input,
        (
            springs,
            damaged.iter().map(|&x| x.parse().unwrap()).collect(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    // 21 = 1 + 4 + 1 + 1 + 4 + 10

    #[test]
    fn test_parse_line1() {
        // let expected = (vec!["?", "?", "?", ".", "#", "#", "#"], vec![1, 1, 3]);
        let expected = ("???.###", vec![1, 1, 3]);
        let result = parse_line("???.### 1,1,3");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = (
            // vec![
            //     ".", "?", "?", ".", ".", "?", "?", ".", ".", ".", "?", "#", "#", ".",
            // ],
            ".??..??...?##.",
            vec![1, 1, 3],
        );
        let result = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = (
            // vec![
            //     "?", "#", "?", "#", "?", "#", "?", "#", "?", "#", "?", "#", "?", "#", "?",
            // ],
            "?#?#?#?#?#?#?#?",
            vec![1, 3, 1, 6],
        );
        let result = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = (
            // vec![
            //     "?", "?", "?", "?", ".", "#", ".", ".", ".", "#", ".", ".", ".",
            // ],
            "????.#...#...",
            vec![4, 1, 1],
        );
        let result = parse_line("????.#...#... 4,1,1");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line5() {
        let expected = (
            // vec![
            //     "?", "?", "?", "?", ".", "#", "#", "#", "#", "#", "#", ".", ".", "#", "#", "#",
            //     "#", "#", ".",
            // ],
            "????.######..#####.",
            vec![1, 6, 5],
        );
        let result = parse_line("????.######..#####. 1,6,5");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line6() {
        let expected = (
            // vec!["?", "#", "#", "#", "?", "?", "?", "?", "?", "?", "?", "?"],
            "?###????????",
            vec![3, 2, 1],
        );
        let result = parse_line("?###???????? 3,2,1");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = vec![
            ("???.###", vec![1, 1, 3]),
            (".??..??...?##.", vec![1, 1, 3]),
            ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]),
            ("????.#...#...", vec![4, 1, 1]),
            ("????.######..#####.", vec![1, 6, 5]),
            ("?###????????", vec![3, 2, 1]),
        ];
        let (_, result) = separated_list1(line_ending, parse_line)(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        )
        .unwrap();
        assert_eq!(result, expected);
    }
}
