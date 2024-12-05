use aoc_helper::read_arg_file;
use std::io::BufRead;

fn check_for_xmas_samx(window: &[char]) -> u32 {
    if window[0] == 'X' && window[1] == 'M' && window[2] == 'A' && window[3] == 'S' {
        return 1;
    }
    if window[0] == 'S' && window[1] == 'A' && window[2] == 'M' && window[3] == 'X' {
        return 1;
    }
    0
}

fn check_for_x_mas(window: &[[char; 3]; 3]) -> u32 {
    if !(window[1][1] == 'A') {
        return 0;
    }
    if window[0][0] == 'M' && window[2][2] == 'S' || window[0][0] == 'S' && window[2][2] == 'M' {
        if window[0][2] == 'M' && window[2][0] == 'S' || window[0][2] == 'S' && window[2][0] == 'M'
        {
            return 1;
        }
    }
    0
}

fn size_of_row(i: usize, max_size: usize) -> usize {
    if i < max_size {
        i + 1
    } else {
        2 * max_size - i - 1
    }
}

fn diagonal_left_matrix_at(i: usize, j: usize, matrix: &[String], max_size: usize) -> char {
    // need matrix[x,y]

    let x = if i < max_size {
        i - j
    } else {
        (max_size - 1) - j
    };
    let y = if i < max_size {
        0 + j
    } else {
        (i + 1 - max_size) + j
    };
    matrix[x].chars().nth(y).unwrap()
}

fn diagonal_right_matrix_at(i: usize, j: usize, matrix: &[String], max_size: usize) -> char {
    // need matrix[x,y]

    let x = if i < max_size {
        0 + j
    } else {
        ((i + 1) - max_size) + j
    };
    let y = if i < max_size {
        max_size - (i + 1) + j
    } else {
        0 + j
    };
    //println!("i: {}, j: {}, x: {}, y: {}, max: {}", i, j, x, y, max_size);
    matrix[x].chars().nth(y).unwrap()
}

fn main() {
    let mut row_list = Vec::new();
    let reader = read_arg_file().unwrap();

    for line in reader.lines() {
        row_list.push(line.unwrap());
    }
    let vertical_size = row_list.len();
    let horizontal_size = row_list[0].len();

    // check row wise
    let mut sum_row: u32 = 0;
    for i in 0..vertical_size {
        for j in 0..(vertical_size - 3) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[i].chars().nth(j + k).unwrap();
            }
            sum_row += check_for_xmas_samx(&window);
        }
    }
    println!("sum_row: {}", sum_row);

    // check column wise
    let mut sum_col: u32 = 0;
    for j in 0..vertical_size {
        for i in 0..(vertical_size - 3) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[i + k].chars().nth(j).unwrap();
            }
            sum_col += check_for_xmas_samx(&window);
        }
    }
    println!("sum_col: {}", sum_col);

    // check diagonal
    let mut sum_diag_left: u32 = 0;
    for i in 0..(2 * vertical_size) {
        let row_size = size_of_row(i, vertical_size);
        if row_size >= 4 {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for j in 0..(row_size - 3) {
                for k in 0..4 {
                    window[k] = diagonal_left_matrix_at(i, j + k, &row_list, vertical_size);
                }
                sum_diag_left += check_for_xmas_samx(&window);
            }
        }
    }
    println!("sum_diag_left: {}", sum_diag_left);

    // check diagonal
    let mut sum_diag_right: u32 = 0;
    for i in 0..(2 * vertical_size) {
        let row_size = size_of_row(i, vertical_size);
        if row_size >= 4 {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for j in 0..(row_size - 3) {
                for k in 0..4 {
                    window[k] = diagonal_right_matrix_at(i, j + k, &row_list, vertical_size);
                }
                sum_diag_right += check_for_xmas_samx(&window);
            }
        }
    }
    println!("sum_diag_right: {}", sum_diag_right);
    println!(
        "task1: sum: {}",
        sum_row + sum_col + sum_diag_left + sum_diag_right
    );

    let mut sum_xmas: u32 = 0;
    for i in 1..vertical_size - 1 {
        for j in 1..(vertical_size - 1) {
            let mut window: [[char; 3]; 3] = [['0', '0', '0'], ['0', '0', '0'], ['0', '0', '0']];
            for k in 0..3 {
                for l in 0..3 {
                    window[k][l] = row_list[i + k - 1].chars().nth(j + l - 1).unwrap();
                }
            }
            sum_xmas += check_for_x_mas(&window);
        }
    }
    println!("task2: sum_xmas {}", sum_xmas);
}
