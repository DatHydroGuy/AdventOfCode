use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
struct Game {
    id: u32,
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Game {
    fn new(id: u32, reds: u32, greens: u32, blues: u32) -> Self {
        return Self {
            id,
            reds,
            greens,
            blues,
        };
    }
}

fn main() {
    /*
    Determine the fewest number of red, green, and blue cubes required to play each game.
    Multiply those numbers together to get the "order" of each game. What is the
    sum of the orders of all games?
    */
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    result.to_string()
}

fn parse_lines(input: &str) -> IResult<&str, u32> {
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;

    let winners = result.iter().map(|g| g.reds * g.greens * g.blues).sum();
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

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, game_num) = parse_game_number(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, result) = parse_number_colour_groups(input)?;

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

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

        if red_count > max_red {
            max_red = red_count
        }
        if green_count > max_green {
            max_green = green_count
        }
        if blue_count > max_blue {
            max_blue = blue_count
        }
    });

    let g = Game::new(game_num, max_red, max_green, max_blue);

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
    fn test_parse_lines1() {
        let expected = 4 * 2 * 6;

        let result = parse_lines("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines2() {
        let expected = 1 * 3 * 4;

        let result =
            parse_lines("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines3() {
        let expected = 20 * 13 * 6;

        let result =
            parse_lines("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines4() {
        let expected = 14 * 3 * 15;

        let result =
            parse_lines("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines5() {
        let expected = 6 * 3 * 2;

        let result = parse_lines("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = 48 + 12 + 1560 + 630 + 36;

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
