static INPUT_FILE: &str = include_str!("./input.txt");

fn first_number_in_slice(slice: &str) -> Option<u32> {
    let char = slice.chars().next().unwrap();
    if char.is_ascii_digit() {
        return Some(char.to_digit(10).unwrap());
    };

    let spelt_out_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, num) in spelt_out_numbers.into_iter().enumerate() {
        if slice.starts_with(num) {
            return Some(u32::try_from(i).unwrap() + 1);
        };
    }
    None
}

fn main() {
    let mut total = 0;
    for line in INPUT_FILE.lines() {
        for i in 0..line.len() {
            if let Some(first) = first_number_in_slice(&line[i..]) {
                total += first * 10;
                break;
            }
        }
        for i in (0..line.len()).rev() {
            if let Some(last) = first_number_in_slice(&line[i..]) {
                total += last;
                break;
            }
        }
    }
    println!("{}", total);
}
