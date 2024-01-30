use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

// #[derive(Debug)]
// struct Game {
//     id: u32,
//     reds: u32,
//     greens: u32,
//     blues: u32,
// }
//
// impl Game {
//     fn new(id: u32, reds: u32, greens: u32, blues: u32) -> Self {
//         return Self {
//             id,
//             reds,
//             greens,
//             blues,
//         };
//     }
// }

fn main() {
    /*
    Determine which games would have been possible if the bag had been loaded
    with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the
    sum of the IDs of those games?
    */
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

// fn parse_line(input: &str) -> IResult<&str, Game> {
//     let (input, game_num) = parse_game_number(input)?;
//     let (input, _) = tag(": ")(input)?;
//     let (input, result) = parse_number_colour_groups(input)?;
//
//     let mut red_count = 0;
//     let mut green_count = 0;
//     let mut blue_count = 0;
//
//     result.iter().for_each(|game_data| {
//         game_data.iter().for_each(|hand_data| {
//             if hand_data[1] == "red" {
//                 red_count += hand_data[0].parse::<u32>().unwrap();
//             } else if hand_data[1] == "green" {
//                 green_count += hand_data[0].parse::<u32>().unwrap();
//             } else if hand_data[1] == "blue" {
//                 blue_count += hand_data[0].parse::<u32>().unwrap();
//             }
//         })
//     });
//     let g = Game::new(game_num, red_count, green_count, blue_count);
//
//     Ok((input, g))
// }

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, game_num) = parse_game_number(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, result) = parse_number_colour_groups(input)?;

    let mut valid = true;

    result.iter().for_each(|game_data| {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        game_data.iter().for_each(|hand_data| {
            if hand_data[1] == "red" {
                red_count += hand_data[0].parse::<u32>().unwrap();
            } else if hand_data[1] == "green" {
                green_count += hand_data[0].parse::<u32>().unwrap();
            } else if hand_data[1] == "blue" {
                blue_count += hand_data[0].parse::<u32>().unwrap();
            }
        });

        if red_count > 12 || green_count > 13 || blue_count > 14 {
            valid = false
        }
    });

    let mut g = 0;
    if valid {
        // dbg!(game_num);
        g = game_num;
    }

    Ok((input, g))
}

fn parse_game_number(input: &str) -> IResult<&str, u32> {
    let (input, result) = separated_list1(tag(" "), alphanumeric1)(input)?;
    Ok((input, result[1].parse().unwrap()))
}

fn parse_number_colour_groups(input: &str) -> IResult<&str, Vec<Vec<Vec<&str>>>> {
    let (input, result) = separated_list1(tag("; "), parse_number_colour_pairs)(input)?;
    Ok((input, result))
}

fn parse_number_colour_pairs(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, result) = separated_list1(tag(", "), parse_number_and_colour)(input)?;
    Ok((input, result))
}

fn parse_number_and_colour(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, result) = separated_list1(tag(" "), alphanumeric1)(input)?;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let expected = 1u32;

        let result = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = 2u32;

        let result = parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = 0u32;

        let result =
            parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = 0u32;

        let result =
            parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line5() {
        let expected = 5u32;

        let result = parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = 8u32;

        let result = parse_lines(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result.unwrap().1, expected);
    }
}
