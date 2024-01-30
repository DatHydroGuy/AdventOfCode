use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, line_ending, multispace1};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    result.to_string()
}

fn parse_lines(input: &str) -> IResult<&str, u64> {
    let (input, directions) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (_, vec_nodes) = separated_list1(line_ending, parse_line)(input)?;
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curr_nodes = vec![];
    let mut cycle_counts = vec![];
    for vec_node in vec_nodes {
        let temp_key = vec_node.keys().last().unwrap();
        if temp_key.ends_with('A') {
            curr_nodes.push(*temp_key);
            cycle_counts.push(0);
        }
        nodes.insert(temp_key, *vec_node.values().last().unwrap());
    }

    let mut step_count;
    for (idx, mut curr_node) in curr_nodes.clone().into_iter().enumerate() {
        step_count = 0;
        'outer: loop {
            for direction in directions.chars() {
                if direction == 'L' {
                    curr_node = &nodes.get(curr_node).unwrap().0;
                } else {
                    curr_node = &nodes.get(curr_node).unwrap().1;
                }
                step_count += 1;
                if curr_node.ends_with('Z') {
                    break 'outer;
                }
            }
        }
        cycle_counts[idx] = step_count;
    }

    // calculate the LCM of the numbers in cycle_counts
    let mut lcm = 1u64;
    cycle_counts.iter().for_each(|&x| {
        lcm = num::integer::lcm(lcm, x);
    });
    Ok(("", lcm))
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
        let expected = String::from("6");
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, expected);
    }
}
