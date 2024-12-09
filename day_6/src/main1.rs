use aoc_helper::read_arg_file;
use std::io::BufRead;

enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

fn main() {
    let mut map_list = Vec::new();
    let reader = read_arg_file().unwrap();

    let mut pos_start: (usize, usize, Direction) = (0, 0, Direction::UP);
    for (y_idx, line) in reader.lines().enumerate() {
        let mut char_list = Vec::new();
        for (x_idx, elem) in line.unwrap().chars().enumerate() {
            if elem == '^' {
                pos_start = (y_idx, x_idx, Direction::UP);
            }
            if elem == '>' {
                pos_start = (y_idx, x_idx, Direction::RIGHT);
            }
            if elem == 'v' {
                pos_start = (y_idx, x_idx, Direction::DOWN);
            }
            if elem == '<' {
                pos_start = (y_idx, x_idx, Direction::LEFT);
            }
            char_list.push(elem);
        }
        map_list.push(char_list);
    }
    println!(
        "just test start pos: {}",
        map_list[pos_start.0][pos_start.1]
    );
}
