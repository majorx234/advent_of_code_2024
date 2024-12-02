use aoc_helper::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, row) = separated_list1(tag(" "), complete::u32)(input)?;
    Ok((input, row))
}

struct State {
    pub increase: bool,
    pub decrease: bool,
    pub idx: usize,
    pub save: bool,
}

fn is_end(state: &State, row: &[u32]) -> bool {
    if state.idx >= row.len() {
        return true;
    }
    false
}

fn condition1(row: &[u32], idx: usize) -> bool {
    if idx >= row.len() - 1 {
        return true;
    };
    if row[idx].abs_diff(row[idx + 1]) > 3 {
        return false;
    }
    true
}

fn condition2(row: &[u32], state: &mut State) -> bool {
    if state.idx >= row.len() - 1 {
        return true;
    };
    if row[state.idx] > row[state.idx + 1] {
        state.decrease = true;
        if state.increase {
            return false;
        }
    } else if row[state.idx] < row[state.idx + 1] {
        state.increase = true;
        if state.decrease {
            return false;
        }
    } else {
        return false;
    }
    true
}

fn check_row(row: &Vec<u32>) -> bool {
    //for (idx, elem) in row.iter().enumerate() {}
    let mut state = State {
        increase: false,
        decrease: false,
        idx: 0,
        save: true,
    };
    while !is_end(&state, row) {
        if !condition1(row, state.idx) {
            state.save = false;
            break;
        }
        if !condition2(row, &mut state) {
            state.save = false;
            break;
        }
        state.idx += 1;
    }
    println!("check_row: {:?} is {}", row, state.save);
    state.save
}

fn main() {
    let mut row_list = Vec::new();
    let reader = read_arg_file().unwrap();
    for line in reader.lines() {
        if let Ok((_, row)) = parse_list(&line.unwrap()) {
            row_list.push(row);
        }
    }
    let mut sum_save = 0;
    for row in row_list.iter() {
        if check_row(row) {
            sum_save += 1;
        }
    }
    println!("sum_save = {}", sum_save);
}
