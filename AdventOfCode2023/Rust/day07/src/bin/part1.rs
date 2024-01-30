use nom::character::complete::{alphanumeric1, digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::{IResult, InputIter};
use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = parse_lines(input).unwrap().1;
    let ranked = rank_hands(result);
    let mut total = 0u64;
    for (i, r) in ranked.iter().enumerate() {
        total += (i + 1) as u64 * r.3;
    }

    total.to_string()
}

fn rank_hands(hands: Vec<Vec<&str>>) -> Vec<(u64, Vec<u8>, &str, u64)> {
    let mut hand_ranks = vec![];
    for hand in hands {
        let hand_data = rank_hand(hand);
        hand_ranks.push(hand_data);
    }
    hand_ranks.sort();
    hand_ranks
}

fn rank_hand(hand: Vec<&str>) -> (u64, Vec<u8>, &str, u64) {
    const CARD_POSITIONS: &str = "0123456789TJQKA";

    let cards = hand[0];
    let hand_data = cards.chars().fold(HashMap::new(), |mut acc, chr| {
        *acc.entry(chr).or_insert(0) += 1;
        acc
    });

    // Reverse-sort the character (card) counts, then build a score based on them
    let mut hand_group_counts: Vec<_> = hand_data.values().collect();
    hand_group_counts.sort();
    hand_group_counts.reverse();
    let mut score: u64 = 0;
    for (i, hand_group_count) in hand_group_counts.iter().enumerate() {
        score += (**hand_group_count) as u64 * 10u64.pow((4 - i) as u32);
    }
    let mut positions: Vec<u8> = vec![];
    for card in hand[0].chars() {
        positions.push(CARD_POSITIONS.position(|x| x == card).unwrap() as u8);
    }
    (score, positions, hand[0], hand[1].parse().unwrap())
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, result) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, cards) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = digit1(input)?;
    Ok((input, vec![cards, bid]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_hand1() {
        let expected = (21110, vec![3, 2, 10, 3, 13], "32T3K", 765);
        let result = rank_hand(vec!["32T3K", "765"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank_hand2() {
        let expected = (31100, vec![10, 5, 5, 11, 5], "T55J5", 684);
        let result = rank_hand(vec!["T55J5", "684"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank_hand3() {
        let expected = (22100, vec![13, 13, 6, 7, 7], "KK677", 28);
        let result = rank_hand(vec!["KK677", "28"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank_hand4() {
        let expected = (22100, vec![13, 10, 11, 11, 10], "KTJJT", 220);
        let result = rank_hand(vec!["KTJJT", "220"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank_hand5() {
        let expected = (31100, vec![12, 12, 12, 11, 14], "QQQJA", 483);
        let result = rank_hand(vec!["QQQJA", "483"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rank_hands() {
        let expected = vec![
            (21110, vec![3, 2, 10, 3, 13], "32T3K", 765),
            (22100, vec![13, 10, 11, 11, 10], "KTJJT", 220),
            (22100, vec![13, 13, 6, 7, 7], "KK677", 28),
            (31100, vec![10, 5, 5, 11, 5], "T55J5", 684),
            (31100, vec![12, 12, 12, 11, 14], "QQQJA", 483),
        ];
        let result = rank_hands(vec![
            vec!["32T3K", "765"],
            vec!["T55J5", "684"],
            vec!["KK677", "28"],
            vec!["KTJJT", "220"],
            vec!["QQQJA", "483"],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let expected = String::from("6440"); //765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5)
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, expected);
    }
}
