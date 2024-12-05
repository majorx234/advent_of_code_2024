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

fn size_of_row(i: usize, max_size: usize) -> usize {
    if i < max_size {
        i + 1
    } else {
        2 * max_size - i
    }
}

fn diagonal_left_matrix_at(i: usize, j: usize, matrix: &[String], max_size: usize) -> char {
    // need matrix[x,y]

    let x = if i < max_size {
        i - j
    } else {
        (max_size - 1) - j
    };
    let y = if i <= max_size {
        0 + j
    } else {
        (i - max_size) + j
    };
    matrix[x].chars().nth(y).unwrap()
}

fn diagonal_right_matrix_at(i: usize, j: usize, matrix: &[String], max_size: usize) -> char {
    // need matrix[x,y]

    let x = if i < max_size {
        0 + j
    } else {
        (i - max_size) + j
    };
    let y = if i < max_size {
        max_size - (i + 1) + j
    } else {
        0 + j
    };
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
    let mut sum: u32 = 0;
    for i in 0..vertical_size {
        for j in 0..(vertical_size - 3) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[i].chars().nth(j + k).unwrap();
            }
            if check_for_xmas_samx(&window) == 1 {
                println!("window: {:?}, j:{} i: {}", window, i, j);
            }
            sum += check_for_xmas_samx(&window);
        }
    }
    println!("sum: {}", sum);

    // check column wise
    for j in 0..vertical_size {
        for i in 0..(vertical_size - 3) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[i + k].chars().nth(j).unwrap();
            }
            if check_for_xmas_samx(&window) == 1 {
                println!("window: {:?}, j:{} i: {}", window, i, j);
            }
            sum += check_for_xmas_samx(&window);
        }
    }
    println!("sum: {}", sum);

    // check diagonal
    for i in 0..(2 * vertical_size) {
        let row_size = size_of_row(i, vertical_size);
        if row_size >= 4 {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for j in 0..(row_size - 3) {
                for k in 0..4 {
                    window[k] = diagonal_left_matrix_at(i, j + k, &row_list, vertical_size);
                }
                println!("window: {:?}, j:{} i: {}", window, i, j);
                sum += check_for_xmas_samx(&window);
            }
        }
    }
    println!("sum: {}", sum);

    // check diagonal
    for i in 0..(2 * vertical_size) {
        let row_size = size_of_row(i, vertical_size);
        if row_size >= 4 {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for j in 0..(row_size - 3) {
                for k in 0..4 {
                    window[k] = diagonal_right_matrix_at(i, j + k, &row_list, vertical_size);
                }
                println!("window: {:?}, j:{} i: {}", window, i, j);
                sum += check_for_xmas_samx(&window);
            }
        }
    }
    println!("sum: {}", sum);
}
