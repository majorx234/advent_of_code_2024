use aoc_helper::read_arg_file;
use regex::Regex;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    let mut argit = args();
    let file_name = argit.nth(1).clone();

    let file_name = if let Some(file_name) = file_name {
        file_name
    } else {
        panic!("No filename argument given");
    };

    let raw_input: String = read_to_string(file_name).expect("could not read file");

    let mut sum = 0;
    let complete_size = raw_input.len();
    let re = Regex::new(r"^mul\(([0-9]+),([0-9]+)\)").unwrap();

    for i in 0..(complete_size - 12) {
        let window: &str = &raw_input[i..i + 12];
        if re.is_match(window) {
            let Some((_, [value1_raw, value2_raw])) =
                re.captures(window).map(|caps| caps.extract())
            else {
                return;
            };
            let value1 = value1_raw.parse::<u32>().unwrap();
            let value2 = value2_raw.parse::<u32>().unwrap();
            sum += value1 * value2;
        }
    }
    println!("sum: {}", sum);
}
