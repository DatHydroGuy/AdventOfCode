use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, line_ending, multispace1};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;

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
    let (input, directions) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (_, vec_nodes) = separated_list1(line_ending, parse_line)(input)?;
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for vec_node in vec_nodes {
        nodes.insert(
            vec_node.keys().last().unwrap(),
            *vec_node.values().last().unwrap(),
        );
    }
    let mut curr_node = "AAA";
    let mut step_count = 0;
    'outer: loop {
        for direction in directions.chars() {
            if direction == 'L' {
                curr_node = &nodes.get(curr_node).unwrap().0;
            } else {
                curr_node = &nodes.get(curr_node).unwrap().1;
            }
            step_count += 1;
            if curr_node == "ZZZ" {
                break 'outer;
            }
        }
    }
    Ok(("", step_count))
}

fn parse_line(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let (input, node_name) = take_until(" ")(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, node_left) = take_until(", ")(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, node_right) = take_until(")")(input)?;
    let (input, _) = tag(")")(input)?;
    nodes.insert(node_name, (node_left, node_right));
    Ok((input, nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines1() {
        let expected = String::from("2");
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lines2() {
        let expected = String::from("6");
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, expected);
    }
}
