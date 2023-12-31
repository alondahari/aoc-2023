use std::{
    cmp::{max, min},
    ops::Range,
};

use regex::Regex;

static INPUT_FILE: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq)]
struct Gear {
    line_idx: usize,
    char_idx: usize,
    adjacent_number: u32,
}

impl Gear {
    fn new(line_idx: usize, char_idx: usize, adjacent_number: u32) -> Self {
        Self {
            line_idx,
            char_idx,
            adjacent_number,
        }
    }

    fn push_or_tally(self, totals: &mut (u32, u32), gears_locations: &mut Vec<Gear>, num: u32) {
        if let Some(gear) = gears_locations
            .iter()
            .find(|gear| gear.line_idx == self.line_idx && gear.char_idx == self.char_idx)
        {
            totals.1 += num * gear.adjacent_number;
        } else {
            gears_locations.push(self)
        }
    }
}

fn has_special_char(s: &str) -> bool {
    Regex::new(r"[^\.\w\d]").unwrap().is_match(s)
}

fn prev_line<'a>(lines: &'a [&'a str], index: usize) -> Option<&'a str> {
    if index == 0 {
        None
    } else {
        Some(lines[index - 1])
    }
}

fn next_line<'a>(lines: &'a [&'a str], index: usize) -> Option<&'a str> {
    if index == lines.len() - 1 {
        None
    } else {
        Some(lines[index + 1])
    }
}

fn line_in_range<'a>(index_range: &'a Range<usize>, line: Option<&'a str>) -> &'a str {
    line.map_or("", |l| &l[index_range.clone()])
}

fn surrounding_index_range(start_index: usize, end_index: usize, lines_len: usize) -> Range<usize> {
    let start_index = max(start_index, 1) - 1;
    let end_index = min(end_index, lines_len - 1) + 1;
    start_index..end_index
}

fn find_gears(s: &str, start_index: usize) -> Vec<usize> {
    s.chars()
        .enumerate()
        .filter(|(_, c)| *c == '*')
        .map(|(i, _)| i + start_index)
        .collect()
}

fn process_number(
    num: u32,
    start_index: usize,
    end_index: usize,
    current_line_index: usize,
    lines: &Vec<&str>,
    gears_locations: &mut Vec<Gear>,
    totals: &mut (u32, u32),
) {
    let start_index = max(start_index, 1) - 1;
    let end_index = min(end_index, lines.len() - 1) + 1;
    let range = start_index..end_index;

    let prev_line = line_in_range(&range, prev_line(lines, current_line_index));
    let next_line = line_in_range(&range, next_line(lines, current_line_index));
    let cur_line = lines[current_line_index];

    if !has_special_char(&format!(
        "{}{}{}",
        prev_line,
        &cur_line[range.clone()],
        next_line
    )) {
        return;
    };

    find_gears(prev_line, start_index)
        .into_iter()
        .map(|i| Gear::new(current_line_index - 1, i, num))
        .for_each(|new_gear| {
            new_gear.push_or_tally(totals, gears_locations, num);
        });

    find_gears(next_line, start_index)
        .into_iter()
        .map(|i| Gear::new(current_line_index + 1, i, num))
        .for_each(|new_gear| {
            new_gear.push_or_tally(totals, gears_locations, num);
        });

    find_gears(&cur_line[start_index..start_index + 1], start_index)
        .into_iter()
        .map(|i| Gear::new(current_line_index, i, num))
        .for_each(|new_gear| {
            new_gear.push_or_tally(totals, gears_locations, num);
        });

    find_gears(&cur_line[end_index - 1..end_index], end_index - 1)
        .into_iter()
        .map(|i| Gear::new(current_line_index, i, num))
        .for_each(|new_gear| {
            new_gear.push_or_tally(totals, gears_locations, num);
        });

    totals.0 += num;
}

fn total_part_numbers() -> (u32, u32) {
    let mut totals = (0, 0);
    let mut current_num: Option<u32> = None;
    let mut current_num_start_index: Option<usize> = None;
    let mut gears_locations: Vec<Gear> = vec![];

    let lines = INPUT_FILE.lines().collect::<Vec<&str>>();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if j == 0 && current_num.is_some() {
                process_number(
                    current_num.unwrap(),
                    current_num_start_index.unwrap(),
                    line.len(),
                    i - 1,
                    &lines,
                    &mut gears_locations,
                    &mut totals,
                );

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
                process_number(
                    current_num.unwrap(),
                    current_num_start_index.unwrap(),
                    j,
                    i,
                    &lines,
                    &mut gears_locations,
                    &mut totals,
                );

                current_num_start_index = None;
                current_num = None;
            }
        }
    }

    totals
}

fn main() {
    let totals = total_part_numbers();
    println!("Total part numbers: {}", totals.0);
    println!("Total gear ratio: {}", totals.1);
}
