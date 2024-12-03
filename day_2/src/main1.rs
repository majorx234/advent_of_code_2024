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

fn copy_vec(old: &[u32], mask_idx: usize) -> Vec<u32> {
    let mut new = Vec::new();
    for (idx, elem) in old.iter().enumerate() {
        if idx != mask_idx {
            new.push(*elem);
        }
    }
    new
}

fn check_row(row: &Vec<u32>) -> Result<(), usize> {
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
    if state.save {
        Ok(())
    } else {
        Err(state.idx)
    }
}

fn main() {
    let mut row_list = Vec::new();
    let reader = read_arg_file().unwrap();
    for line in reader.lines() {
        if let Ok((_, row)) = parse_list(&line.unwrap()) {
            row_list.push(row);
        }
    }
    let mut sum_save1 = 0;
    let mut sum_save1x = 0;
    for row in row_list.iter() {
        match check_row(row) {
            Ok(_) => sum_save1 += 1,
            Err(idx) => {
                let new_vec1 = copy_vec(row, idx);
                let new_vec2 = copy_vec(row, idx + 1);
                let save3 = if idx > 0 {
                    let new_vec3 = copy_vec(row, idx - 1);
                    check_row(&new_vec3).is_ok()
                } else {
                    false
                };
                if check_row(&new_vec1).is_ok() || check_row(&new_vec2).is_ok() || save3 {
                    sum_save1x += 1;
                }
            }
        }
    }
    println!(
        "sum_save1 = {}, sum_dave2 = {}",
        sum_save1,
        sum_save1 + sum_save1x
    );
}
