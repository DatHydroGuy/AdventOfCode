use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashMap, HashSet};

// NON-DETERMINISTIC FINITE AUTOMATA METHOD

#[derive(Debug, PartialEq)]
struct Nfa {
    q: HashSet<u64>,                           // set of states
    sigma: HashSet<char>,                      // set of symbols
    delta: HashMap<(u64, char), HashSet<u64>>, // set of transition relations
    s: HashSet<u64>,                           // set of initial states
    f: HashSet<u64>,                           // set of final states
    sc: Vec<u64>,                              // state counter
}

impl Nfa {
    fn new(
        q: HashSet<u64>,
        sigma: HashSet<char>,
        delta: HashMap<(u64, char), HashSet<u64>>,
        s: HashSet<u64>,
        f: HashSet<u64>,
    ) -> Self {
        Self {
            q: q.clone(),
            sigma,
            delta,
            s,
            f,
            sc: vec![0; q.len()],
        }
    }

    fn do_delta(&mut self, q: u64, x: char, counts_snapshot: &Vec<u64>) -> HashSet<u64> {
        match self.delta.get(&(q, x)) {
            Some(set) => {
                let curr_set_count = counts_snapshot[q as usize];
                if set.len() == 1 && !set.contains(&q) {
                    self.sc[*set.iter().nth(0).unwrap() as usize] += curr_set_count;
                    self.sc[q as usize] -= curr_set_count;
                } else if set.len() > 1 {
                    let non_self_indices: Vec<_> = set.iter().filter(|&x| x != &q).collect();
                    for &non_self_index in non_self_indices {
                        self.sc[non_self_index as usize] += curr_set_count;
                    }
                    if !set.contains(&q) {
                        self.sc[q as usize] -= curr_set_count;
                    }
                } else if set.is_empty() {
                    self.sc[q as usize] -= curr_set_count;
                }
                set.clone()
            }
            None => HashSet::new(),
        }
    }

    fn run(&mut self, word: &str) -> bool {
        self.sc[0] += 1; // initialise the initial position
        let mut p = self.s.clone();
        for chr in word.chars() {
            let mut p_new: HashSet<_> = HashSet::new();
            let counts_snapshot = self.sc.clone();
            for q in &p {
                let next_states = self.do_delta(*q, chr, &counts_snapshot);
                p_new.extend(next_states);
            }
            p = p_new;
        }
        let intersection: HashSet<_> = self.f.intersection(&p).collect();
        !intersection.is_empty()
    }

    fn get_final_state_count(&self) -> u64 {
        self.sc[self.sc.len() - 2] + self.sc[self.sc.len() - 1]
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut result = 0;
    let results = parse_lines(input).unwrap().1;
    for (springs, groups) in results {
        let nodes = build_nodes(groups);
        let mut nfa = build_nfa(&nodes);
        let _ = nfa.run(&*springs);
        result += nfa.get_final_state_count();
    }
    result.to_string()
}

fn build_nfa(nodes: &String) -> Nfa {
    let mut delta: HashMap<(u64, char), HashSet<u64>> = HashMap::new();
    for (idx, chr) in nodes.chars().enumerate() {
        if chr == '.' {
            delta.insert((idx as u64, '.'), HashSet::from([idx as u64]));
            if idx == nodes.len() - 1 {
                delta.insert((idx as u64, '#'), HashSet::new());
                delta.insert((idx as u64, '?'), HashSet::from([idx as u64]));
            } else {
                delta.insert((idx as u64, '#'), HashSet::from([idx as u64 + 1]));
                delta.insert(
                    (idx as u64, '?'),
                    HashSet::from([idx as u64, idx as u64 + 1]),
                );
            }
        } else if chr == '#' {
            delta.insert((idx as u64, '?'), HashSet::from([idx as u64 + 1]));
            if nodes.chars().nth(idx + 1).unwrap() == '.' {
                delta.insert((idx as u64, '.'), HashSet::from([idx as u64 + 1]));
                delta.insert((idx as u64, '#'), HashSet::new());
            } else if nodes.chars().nth(idx + 1).unwrap() == '#' {
                delta.insert((idx as u64, '#'), HashSet::from([idx as u64 + 1]));
                delta.insert((idx as u64, '.'), HashSet::new());
            }
        } else if chr == '?' {
            delta.insert((idx as u64, '.'), HashSet::from([idx as u64 + 1]));
            delta.insert((idx as u64, '#'), HashSet::from([idx as u64 + 1]));
        }
    }
    Nfa::new(
        HashSet::from_iter(0..nodes.len() as u64),
        HashSet::from(['.', '#', '?']),
        delta,
        HashSet::from([0]),
        HashSet::from([nodes.len() as u64 - 2, nodes.len() as u64 - 1]),
    )
}

fn build_nodes(groups: Vec<u64>) -> String {
    let mut group_string = String::from(".");
    for group in groups {
        for _ in 0..group {
            group_string.push('#');
        }
        group_string.push('.');
    }
    group_string
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(String, Vec<u64>)>> {
    let (_, result) = separated_list1(line_ending, parse_line)(input)?;

    Ok(("", result))
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<u64>)> {
    let (input, springs) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (input, damaged) = separated_list1(tag(","), digit1)(input)?;
    let mut springs_unfolded = String::from(springs);
    let damaged_nums: Vec<_> = damaged.iter().map(|&x| x.parse().unwrap()).collect();
    let mut damaged_unfolded = damaged_nums.clone();
    for _ in 0..4 {
        springs_unfolded.push('?');
        springs_unfolded.push_str(&springs);
        damaged_unfolded.extend(&damaged_nums);
    }
    Ok((input, (springs_unfolded, damaged_unfolded)))
}

#[cfg(test)]
mod tests {
    use super::*;
    // 21 = 1 + 4 + 1 + 1 + 4 + 10

    #[test]
    fn test_run_nfa() {
        let expected = 4;
        let nodes = build_nodes(vec![1, 3]);
        let mut nfa = build_nfa(&nodes);
        let _ = nfa.run(".??..?##?");
        let result = nfa.get_final_state_count();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_nfa() {
        let expected = Nfa::new(
            HashSet::from([0, 1, 2, 3, 4, 5, 6]),
            HashSet::from(['.', '#', '?']),
            HashMap::from([
                ((0, '.'), HashSet::from([0])),
                ((0, '#'), HashSet::from([1])),
                ((0, '?'), HashSet::from([0, 1])),
                ((1, '.'), HashSet::from([2])),
                ((1, '#'), HashSet::new()),
                ((1, '?'), HashSet::from([2])),
                ((2, '.'), HashSet::from([2])),
                ((2, '#'), HashSet::from([3])),
                ((2, '?'), HashSet::from([2, 3])),
                ((3, '.'), HashSet::new()),
                ((3, '#'), HashSet::from([4])),
                ((3, '?'), HashSet::from([4])),
                ((4, '.'), HashSet::new()),
                ((4, '#'), HashSet::from([5])),
                ((4, '?'), HashSet::from([5])),
                ((5, '.'), HashSet::from([6])),
                ((5, '#'), HashSet::new()),
                ((5, '?'), HashSet::from([6])),
                ((6, '.'), HashSet::from([6])),
                ((6, '#'), HashSet::new()),
                ((6, '?'), HashSet::from([6])),
            ]),
            HashSet::from([0]),
            HashSet::from([5, 6]),
        );
        let result = build_nfa(&".#.###.".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_nodes_1() {
        let expected = ".#.#.###.".to_string();
        let result = build_nodes(vec![1, 1, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_nodes_2() {
        let expected = ".#.###.#.######.".to_string();
        let result = build_nodes(vec![1, 3, 1, 6]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let expected = 525152.to_string();
        let result = part2(
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
        let result = part2("???.### 1,1,3");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_2() {
        let expected = 16384.to_string();
        let result = part2(".??..??...?##. 1,1,3");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_3() {
        let expected = 1.to_string();
        let result = part2("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_4() {
        let expected = 16.to_string();
        let result = part2("????.#...#... 4,1,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_5() {
        let expected = 2500.to_string();
        let result = part2("????.######..#####. 1,6,5");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_6() {
        let expected = 506250.to_string();
        let result = part2("?###???????? 3,2,1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line1() {
        let expected = (
            "???.###????.###????.###????.###????.###".to_string(),
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
        );
        let result = parse_line("???.### 1,1,3");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line2() {
        let expected = (
            ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##."
                .to_string(),
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
        );
        let result = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line3() {
        let expected = (
            "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?"
                .to_string(),
            vec![1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6],
        );
        let result = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line4() {
        let expected = (
            "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".to_string(),
            vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1],
        );
        let result = parse_line("????.#...#... 4,1,1");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line5() {
        let expected = ("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.".to_string(), vec![1, 6, 5,1, 6, 5,1, 6, 5,1, 6, 5,1, 6, 5]);
        let result = parse_line("????.######..#####. 1,6,5");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_line6() {
        let expected = (
            "?###??????????###??????????###??????????###??????????###????????".to_string(),
            vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1],
        );
        let result = parse_line("?###???????? 3,2,1");
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_parse_lines() {
        let expected = vec![
            ("???.###????.###????.###????.###????.###".to_string(),
             vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]),
            (".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##."
                 .to_string(),
             vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]),
            ("?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?"
                 .to_string(),
             vec![1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6]),
            ("????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".to_string(),
             vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1]),
            ("????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.".to_string(), vec![1, 6, 5,1, 6, 5,1, 6, 5,1, 6, 5,1, 6, 5]),
            ("?###??????????###??????????###??????????###??????????###????????".to_string(),
             vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1]),
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
