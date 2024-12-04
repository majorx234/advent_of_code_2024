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

fn main() {
    let mut row_list = Vec::new();
    let reader = read_arg_file().unwrap();

    for line in reader.lines() {
        row_list.push(line.unwrap());
    }
    // check row wise

    let mut sum: u32 = 0;
    for i in 0..140 {
        for j in 0..(140 - 4) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[i].chars().nth(j + k).unwrap();
            }
            sum += check_for_xmas_samx(&window);
        }
    }
    println!("sum: {}", sum);

    // check column wise
    for i in 0..140 {
        for j in 0..(140 - 4) {
            let mut window: [char; 4] = ['0', '0', '0', '0'];
            for k in 0..4 {
                window[k] = row_list[j].chars().nth(i + k).unwrap();
            }
            sum += check_for_xmas_samx(&window);
        }
    }
    println!("sum: {}", sum);
}
