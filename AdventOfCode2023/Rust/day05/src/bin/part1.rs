use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    range: Range<i64>,
    offset: i64,
}

impl Mapping {
    fn new(src: i64, dest: i64, range: i64) -> Self {
        Self {
            range: dest..(dest + range),
            offset: src - dest,
        }
    }
}

#[derive(Debug)]
struct MultiMap {
    maps: Vec<Mapping>,
}

impl MultiMap {
    fn new(map_vec: Vec<Mapping>) -> Self {
        Self { maps: map_vec }
    }

    fn apply(&self, number: i64) -> Option<i64> {
        let mut result = None;
        for map in &self.maps {
            if map.range.contains(&number) {
                result = Some(number + map.offset);
                break;
            }
        }
        result
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    result.iter().min().unwrap().to_string()
}

fn parse_lines(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, seed_values) = parse_seed_values(input)?;
    let (_input, mapping_data) = many1(read_mapping_block)(input)?;

    let mut locations = vec![];
    for seed_value in seed_values {
        let mut curr_seed = seed_value;
        for mapping in &mapping_data {
            curr_seed = mapping.apply(curr_seed).unwrap_or(curr_seed)
        }
        locations.push(curr_seed);
    }
    Ok(("", locations))
}

fn parse_seed_values(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _result) = tag("seeds: ")(input)?;
    let (input, result) = separated_list1(space1, digit1)(input)?;
    let numbers: Vec<i64> = result.iter().map(|&x| x.parse::<i64>().unwrap()).collect();
    Ok((input, numbers))
}

fn read_mapping_block(input: &str) -> IResult<&str, MultiMap> {
    let (input, _result) = take_until(":")(input)?;
    let (input, _result) = tag(":")(input)?;
    let (input, _result) = line_ending(input)?;
    let (input, result) = separated_list1(line_ending, read_mapping_block_data)(input)?;
    let multi_map = MultiMap::new(result);
    Ok((input, multi_map))
}

fn read_mapping_block_data(input: &str) -> IResult<&str, Mapping> {
    let (input, result) = separated_list1(space1, digit1)(input)?;
    let numbers: Vec<i64> = result.iter().map(|&x| x.parse::<i64>().unwrap()).collect();
    let mapping = Mapping::new(numbers[0], numbers[1], numbers[2]);
    Ok((input, mapping))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let expected = Ok(("", vec![82, 43, 86, 35]));
        let result = parse_lines(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let expected = String::from("35");
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, expected);
    }
}
