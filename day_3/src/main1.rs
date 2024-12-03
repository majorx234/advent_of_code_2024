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

    let complete_size = raw_input.len();
    for i in 0..(complete_size - 12) {
        let window: &str = &raw_input[i..i + 12];
        let re = Regex::new(r"^mul\([0-9]+,[0-9]+\)").unwrap();
        if re.is_match(window) {
            let re_no1 = Regex::new(r"\([0-9]+,");
            let re_no2 = Regex::new(r",[0-9]+\)");
        }
    }
}
