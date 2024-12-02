use aoc_helper::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

struct TwoLists {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
}

fn heapify(arr: &mut [u32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heapsort(arr: &mut [u32]) {
    let n = arr.len();

    // Build heap (rearrange array)
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // One by one extract an element from heap
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, value_left) = complete::u32(input)?;
    let (input, _) = tag("   ")(input)?;
    let (input, value_right) = complete::u32(input)?;
    Ok((input, (value_left, value_right)))
}

fn parse_two_lists(input: &str) -> IResult<&str, TwoLists> {
    let two_lists = TwoLists {
        left: Vec::new(),
        right: Vec::new(),
    };
    Ok((input, two_lists))
}
fn main() {
    let mut two_lists = TwoLists {
        left: Vec::new(),
        right: Vec::new(),
    };

    let reader = read_arg_file().unwrap();
    for line in reader.lines() {
        if let Ok((_, (value_left, value_right))) = parse_pair(&line.unwrap()) {
            two_lists.left.push(value_left);
            two_lists.right.push(value_right);
        }
    }
    heapsort(&mut two_lists.left);
    heapsort(&mut two_lists.right);
    let mut sum_diff = 0;
    for (value_left, value_right) in two_lists.left.iter().zip(two_lists.right.iter()) {
        sum_diff += value_left.abs_diff(*value_right);
    }
    println!("sum_diff: {}", sum_diff);
}
