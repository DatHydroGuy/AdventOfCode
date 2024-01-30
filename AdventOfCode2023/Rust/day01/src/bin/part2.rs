fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    format!("{}", parse_lines(input))
}

fn parse_lines(input: &str) -> u32 {
    let digit_pairs: Vec<u32> = input.lines().map(parse_line).collect();
    digit_pairs.iter().sum()
}

fn parse_line(input: &str) -> u32 {
    let mut digits = vec![];
    for start_index in 0..input.len() {
        let input_slice = &input[start_index..];
        let slice_first = &input[start_index..=start_index];
        if input_slice.starts_with("one") || slice_first == "1" {
            digits.push(1)
        } else if input_slice.starts_with("two") || slice_first == "2" {
            digits.push(2)
        } else if input_slice.starts_with("three") || slice_first == "3" {
            digits.push(3)
        } else if input_slice.starts_with("four") || slice_first == "4" {
            digits.push(4)
        } else if input_slice.starts_with("five") || slice_first == "5" {
            digits.push(5)
        } else if input_slice.starts_with("six") || slice_first == "6" {
            digits.push(6)
        } else if input_slice.starts_with("seven") || slice_first == "7" {
            digits.push(7)
        } else if input_slice.starts_with("eight") || slice_first == "8" {
            digits.push(8)
        } else if input_slice.starts_with("nine") || slice_first == "9" {
            digits.push(9)
        }
    }

    // let digit_pair = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    //
    // digit_pair.parse().unwrap()

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let expected = 29;
        let result = parse_line("two1nine");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = 83;
        let result = parse_line("eightwothree");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = 13;
        let result = parse_line("abcone2threexyz");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = 24;
        let result = parse_line("xtwone3four");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line5() {
        let expected = 42;
        let result = parse_line("4nineeightseven2");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line6() {
        let expected = 14;
        let result = parse_line("zoneight234");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line7() {
        let expected = 76;
        let result = parse_line("7pqrstsixteen");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line8() {
        let expected = 28;
        let result = parse_line("xtwoneight");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = 281;
        let result = parse_lines(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, expected);
    }
}
