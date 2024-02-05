use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    let mut answer = 0u32;
    for r in result {
        answer += find_combinations(r.0, r.1).len() as u32;
    }
    answer.to_string()
}

fn find_combinations(string: &str, groups: Vec<u32>) -> Vec<String> {
    let num_of_wildcards = count_wildcards(string);
    let wildcard_replacements = get_wildcard_replacements(num_of_wildcards);
    let valid_answers = find_valid_replacements(wildcard_replacements, string, groups);
    valid_answers
}

fn find_valid_replacements(
    replacements: Vec<String>,
    scenario: &str,
    groups: Vec<u32>,
) -> Vec<String> {
    let mut valid = vec![];
    for replacement in replacements {
        let mut replacement_chars = replacement.chars();
        let x = scenario.split('?').fold(String::new(), |mut acc, part| {
            acc.push_str(part);
            if let Some(replacement) = replacement_chars.next() {
                acc.push(replacement);
            }
            acc
        });
        if count_consecutives(x.as_str()) == groups {
            valid.push(x);
        }
    }
    valid
}

fn get_wildcard_replacements(num_wildcards: u32) -> Vec<String> {
    let mut replacements = vec![];
    let end_num = 2_i32.pow(num_wildcards);
    for n in 0..end_num {
        let mut bin_str = format!("{:b}", n);
        while bin_str.len() < num_wildcards as usize {
            bin_str = "0".to_owned() + &*bin_str;
        }
        bin_str = bin_str.replace('0', ".").replace('1', "#");
        replacements.push(bin_str);
    }
    replacements
}

fn count_wildcards(string: &str) -> u32 {
    string
        .chars()
        .filter(|&c| c == '?')
        .collect::<Vec<_>>()
        .len() as u32
}

fn count_consecutives(input: &str) -> Vec<u32> {
    input
        .split('.')
        .filter(|&s| !s.is_empty())
        .map(|x| x.len() as u32)
        .collect()
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(&str, Vec<u32>)>> {
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;

    Ok(("", result))
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
    fn test_part1() {
        let expected = 21.to_string();
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_1() {
        let expected = 1.to_string();
        let result = part1("???.### 1,1,3");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_2() {
        let expected = 4.to_string();
        let result = part1(".??..??...?##. 1,1,3");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_3() {
        let expected = 1.to_string();
        let result = part1("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_4() {
        let expected = 1.to_string();
        let result = part1("????.#...#... 4,1,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_5() {
        let expected = 4.to_string();
        let result = part1("????.######..#####. 1,6,5");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_6() {
        let expected = 10.to_string();
        let result = part1("?###???????? 3,2,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations1() {
        let expected = vec![String::from("#.#.###")];
        let result = find_combinations("???.###", vec![1u32, 1, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations2() {
        let expected = vec![
            String::from("..#...#...###."),
            String::from("..#..#....###."),
            String::from(".#....#...###."),
            String::from(".#...#....###."),
        ];
        let result = find_combinations(".??..??...?##.", vec![1u32, 1, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations3() {
        let expected = vec![String::from(".#.###.#.######")];
        let result = find_combinations("?#?#?#?#?#?#?#?", vec![1u32, 3, 1, 6]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations4() {
        let expected = vec![String::from("####.#...#...")];
        let result = find_combinations("????.#...#...", vec![4u32, 1, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations5() {
        let expected = vec![
            String::from("...#.######..#####."),
            String::from("..#..######..#####."),
            String::from(".#...######..#####."),
            String::from("#....######..#####."),
        ];
        let result = find_combinations("????.######..#####.", vec![1u32, 6, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_combinations6() {
        let expected = vec![
            String::from(".###....##.#"),
            String::from(".###...##..#"),
            String::from(".###...##.#."),
            String::from(".###..##...#"),
            String::from(".###..##..#."),
            String::from(".###..##.#.."),
            String::from(".###.##....#"),
            String::from(".###.##...#."),
            String::from(".###.##..#.."),
            String::from(".###.##.#..."),
        ];
        let result = find_combinations("?###????????", vec![3u32, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_wildcard_replacements() {
        let expected = vec!["...", "..#", ".#.", ".##", "#..", "#.#", "##.", "###"];
        let result = get_wildcard_replacements(3);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards1() {
        let expected = 3u32;
        let result = count_wildcards("???.###");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards2() {
        let expected = 5u32;
        let result = count_wildcards(".??..??...?##.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards3() {
        let expected = 8u32;
        let result = count_wildcards("?#?#?#?#?#?#?#?");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards4() {
        let expected = 4u32;
        let result = count_wildcards("????.#...#...");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards5() {
        let expected = 4u32;
        let result = count_wildcards("????.######..#####.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_wildcards6() {
        let expected = 9u32;
        let result = count_wildcards("?###????????");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives1() {
        let expected = vec![1u32, 1, 3];
        let result = count_consecutives("#.#.###");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives2() {
        let expected = vec![1u32, 1, 3];
        let result = count_consecutives(".#...#....###.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives3() {
        let expected = vec![1u32, 3, 1, 6];
        let result = count_consecutives(".#.###.#.######");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives4() {
        let expected = vec![4u32, 1, 1];
        let result = count_consecutives("####.#...#...");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives5() {
        let expected = vec![1u32, 6, 5];
        let result = count_consecutives("#....######..#####.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_consecutives6() {
        let expected = vec![3u32, 2, 1];
        let result = count_consecutives(".###.##....#");
        assert_eq!(result, expected);
    }

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
        let (_, result) = parse_lines(
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
