use aoc_helper::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum READ_STATE {
    ORDER,
    NUMBERS,
}
fn parse_order(input: &str) -> IResult<&str, (u32, u32)> {
    let seperator = tag("|");
    let (input, (x, y)) = separated_pair(complete::u32, seperator, complete::u32)(input)?;
    Ok((input, (x, y)))
}

fn parse_number_lists(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, row) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, row))
}

fn main() {
    let mut order_list = Vec::new();
    let mut number_lists = Vec::new();
    let reader = read_arg_file().unwrap();
    let mut state = READ_STATE::ORDER;
    for line in reader.lines() {
        if line.as_ref().unwrap().len() < 2 {
            state = READ_STATE::NUMBERS;
            continue;
        }
        if state == READ_STATE::ORDER {
            if let Ok((_, row)) = parse_order(&line.unwrap()) {
                order_list.push(row);
            }
        } else if state == READ_STATE::NUMBERS {
            if let Ok((_, row)) = parse_number_lists(&line.unwrap()) {
                number_lists.push(row);
            }
        }
    }
    println!("order_list: {:?}", order_list);
    println!("number_lists: {:?}", number_lists);
}
