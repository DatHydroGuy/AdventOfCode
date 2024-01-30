use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut num_list: HashMap<(u32, u32), u32> = HashMap::new();
    let mut sym_list: HashMap<(u32, u32), char> = HashMap::new();
    create_digit_and_symbol_hashmaps(input, &mut num_list, &mut sym_list);
    let mut total = 0;
    for ((nx, ny), num) in num_list.into_iter() {
        let num_len = num.to_string().len() as i32;
        for ((sx, sy), _sym) in sym_list.clone().into_iter() {
            if ny as i32 - 1 <= sy as i32
                && sy as i32 <= ny as i32 + 1
                && nx as i32 - 1 <= sx as i32
                && sx as i32 <= nx as i32 + num_len
            {
                total += num;
            }
        }
    }
    total.to_string()
}

fn create_digit_and_symbol_hashmaps(
    input: &str,
    num_list: &mut HashMap<(u32, u32), u32>,
    sym_list: &mut HashMap<(u32, u32), char>,
) {
    let mut prev_digit: u32 = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, chr)| {
            if chr.is_ascii_digit() {
                let Some(digit_val) = chr.to_digit(10) else {
                    panic!("How can you not parse a digit from something which passed the char.is_ascii_digit() check?")
                };
                if prev_digit == 0 {
                    num_list.insert((x as u32, y as u32), digit_val);
                } else {
                    // Find the HashMap entry for the previous x-value, and overwrite it with the new value
                    let x_offset = (prev_digit.ilog10() + 1) as usize;
                    *num_list
                        .get_mut(&((x - x_offset) as u32, y as u32))
                        .unwrap() = prev_digit * 10 + digit_val;
                }
                prev_digit = prev_digit * 10 + digit_val;
            } else if chr != '.' {
                sym_list.insert((x as u32, y as u32), chr);
                prev_digit = 0;
            } else {
                prev_digit = 0;
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let expected = 4361.to_string();
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, expected);
    }
}
