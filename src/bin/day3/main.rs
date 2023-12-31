use std::{
    cmp::{max, min},
    ops::Range,
};

use regex::Regex;

static INPUT_FILE: &str = include_str!("./input.txt");

fn has_special_char(s: &str) -> bool {
    Regex::new(r"[^\.\w\d]").unwrap().is_match(s)
}

fn adjacent_special_char(
    index_range: Range<usize>,
    current_line_index: usize,
    lines: &Vec<&str>,
) -> bool {
    // println!("{:?} {} {}", index_range, current_line_index, lines.len());
    let prev_line = if current_line_index == 0 {
        ""
    } else {
        &lines[current_line_index - 1][index_range.clone()]
    };

    let next_line = if current_line_index == lines.len() - 1 {
        ""
    } else {
        &lines[current_line_index + 1][index_range.clone()]
    };
    let cur_line = &lines[current_line_index][index_range];
    // println!("{} {} {}", prev_line, cur_line, next_line);
    let str = String::from("") + cur_line + prev_line + next_line;

    has_special_char(&str)
}

fn main() {
    let mut total = 0;
    let mut current_num: Option<u32> = None;
    let mut current_num_start_index: Option<usize> = None;

    let lines = INPUT_FILE.lines().collect::<Vec<&str>>();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if j == 0 && current_num.is_some() {
                let start_index = max(current_num_start_index.unwrap(), 1) - 1;
                if adjacent_special_char(start_index..line.len(), i - 1, &lines) {
                    total += current_num.unwrap();
                }

                current_num_start_index = None;
                current_num = None;
            }

            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                match current_num {
                    Some(num) => current_num = Some(num * 10 + digit),
                    None => {
                        current_num = Some(digit);
                        current_num_start_index = Some(j);
                    }
                }
            } else if (c == '.' || has_special_char(&c.to_string())) && current_num.is_some() {
                let start_index = max(current_num_start_index.unwrap(), 1) - 1;
                let end_index = min(j, lines.len() - 1) + 1;
                if adjacent_special_char(start_index..end_index, i, &lines) {
                    // println!("{}", current_num.unwrap());
                    total += current_num.unwrap();
                }

                current_num_start_index = None;
                current_num = None;
            }
        }
    }
    println!("{}", total);
}
