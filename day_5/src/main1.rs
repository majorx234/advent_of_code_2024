use aoc_helper::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
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
    let mut number2index = HashMap::new();
    let mut index2number = HashMap::new();
    let mut node_list = HashSet::new();
    let mut index = 0;
    for (n1, n2) in order_list.iter() {
        if !number2index.contains_key(&n1) {
            number2index.insert(n1, index);
            index2number.insert(index, n1);
            index += 1;
        }
        if !number2index.contains_key(&n2) {
            number2index.insert(n2, index);
            index2number.insert(index, n2);
            index += 1;
        }
    }
    println!("num elements: {} ", index);
    for node in 0..index {
        node_list.insert(node);
    }
    let mut adjecence_matrix = Vec::new();
    let mut inverse_adjecence_matrix = Vec::new();
    for _ in 0..index {
        adjecence_matrix.push(vec![false; index]);
        inverse_adjecence_matrix.push(vec![false; index])
    }
    for (n1, n2) in order_list.iter() {
        adjecence_matrix[number2index[&n1]][number2index[&n2]] = true;
        inverse_adjecence_matrix[number2index[&n2]][number2index[&n1]] = true;
    }

    //find ordering:
    let mut ordered_node_list = Vec::new();
    while !node_list.is_empty() {
        // check node with no incomming etches
        let mut no_incommming_edges: Option<u32> = None;
        for node in &node_list {
            let mut has_incomming_nodes = false;
            for node_incomming_edges in &inverse_adjecence_matrix[*node] {
                has_incomming_nodes |= node_incomming_edges;
            }
            if !has_incomming_nodes {
                no_incommming_edges = Some(*node as u32);
            }
        }
        if let Some(node_with_no_icomming_etches) = no_incommming_edges {
            ordered_node_list.push(node_with_no_icomming_etches);
            node_list.remove(&(node_with_no_icomming_etches as usize));
            for vec in inverse_adjecence_matrix.iter_mut() {
                vec[node_with_no_icomming_etches as usize] = false;
            }
        } else {
            println!("loop detected!");
            println!("{:?}", inverse_adjecence_matrix);
        }
    }
    let mut number2order_index = HashMap::new();
    for (order_index, index) in ordered_node_list.iter().enumerate() {
        number2order_index.insert(index2number[&(*index as usize)], order_index);
    }
    let mut count_ok = 0;
    for number_list in number_lists {
        let mut last_order_index = 0;
        let mut is_ok = true;
        for number in &number_list {
            if last_order_index > number2order_index[number] {
                is_ok = false;
                break;
            }
            last_order_index = number2order_index[number];
        }
        if is_ok {
            count_ok += number_list[number_list.len() / 2];
        }
    }
    println!("count_ok :{}", count_ok);
}
