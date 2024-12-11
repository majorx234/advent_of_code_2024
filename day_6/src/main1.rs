use aoc_helper::read_arg_file;
use std::io::BufRead;

enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

fn main() {
    let reader = read_arg_file().unwrap();

    let mut map_list = Vec::new();
    let mut visit_map = Vec::new();
    let mut obstacles = Vec::new();
    let mut pos: (usize, usize, Direction) = (0, 0, Direction::UP);
    for (y_idx, line) in reader.lines().enumerate() {
        let mut char_list = Vec::new();
        for (x_idx, elem) in line.unwrap().chars().enumerate() {
            if elem == '^' {
                pos = (y_idx, x_idx, Direction::UP);
            }
            if elem == '>' {
                pos = (y_idx, x_idx, Direction::RIGHT);
            }
            if elem == 'v' {
                pos = (y_idx, x_idx, Direction::DOWN);
            }
            if elem == '<' {
                pos = (y_idx, x_idx, Direction::LEFT);
            }
            if elem == '#' {
                obstacles.push((y_idx, x_idx));
            }
            char_list.push(elem);
        }
        visit_map.push(char_list.clone());
        map_list.push(char_list);
    }
    println!("just test start pos: {}", map_list[pos.0][pos.1]);
    let mut moving = true;
    let mut direction = pos.2;
    while moving {
        match direction {
            Direction::UP => {
                // find next obs
                let obs_distance = 100000;
                let mut next_obs: Option<&(usize, usize)> = None;
                for obs in &obstacles {
                    if obs.1 == pos.1 {
                        if obs.0 < pos.0 {
                            if obs_distance > pos.0 - obs.0 {
                                next_obs = Some(&obs);
                            }
                        }
                    }
                }
                if let Some(next_obs) = next_obs {
                    for x in next_obs.0..pos.0 {
                        visit_map[pos.1][x] = '#';
                    }
                    pos.0 = next_obs.0 - 1;
                    pos.2 = Direction::RIGHT;
                } else {
                    // no obstacle
                    for y in 0..pos.0 {
                        if visit_map[y][pos.1] == '.' {
                            visit_map[y][pos.1] = '#';
                        }
                    }
                    moving = false;
                }
            }
            Direction::RIGHT => {
                // find next obs
                let obs_distance = 100000;
                let mut next_obs: Option<&(usize, usize)> = None;
                for obs in &obstacles {
                    if obs.0 == pos.0 {
                        if obs.1 > pos.1 {
                            if obs_distance > obs.1 - pos.1 {
                                next_obs = Some(&obs);
                            }
                        }
                    }
                }
                if let Some(next_obs) = next_obs {
                    for x in pos.1..next_obs.1 {
                        visit_map[pos.0][x] = '#';
                    }
                    pos.1 = next_obs.1 - 1;
                    pos.2 = Direction::DOWN;
                } else {
                    // no obstacle
                    for x in pos.1..0 {
                        if visit_map[pos.0][x] == '.' {
                            visit_map[pos.0][x] = '#';
                        }
                    }
                    moving = false;
                }
            }
            Direction::DOWN => {
                // find next obs
                let obs_distance = 100000;
                let mut next_obs: Option<&(usize, usize)> = None;
                for obs in &obstacles {
                    if obs.1 == pos.1 {
                        if obs.0 > pos.0 {
                            if obs_distance > obs.0 - pos.0 {
                                next_obs = Some(&obs);
                            }
                        }
                    }
                }
                if let Some(next_obs) = next_obs {
                    for x in next_obs.0..pos.0 {
                        visit_map[pos.1][x] = '#';
                    }
                    pos.0 = next_obs.0 - 1;
                    pos.2 = Direction::LEFT;
                } else {
                    // no obstacle
                    for y in pos.0..visit_map.len() {
                        if visit_map[y][pos.1] == '.' {
                            visit_map[y][pos.1] = '#';
                        }
                    }
                    moving = false;
                }
            }
            Direction::LEFT => {
                // find next obs
                let obs_distance = 100000;
                let mut next_obs: Option<&(usize, usize)> = None;
                for obs in &obstacles {
                    if obs.0 == pos.0 {
                        if obs.1 < pos.1 {
                            if obs_distance > pos.1 - obs.1 {
                                next_obs = Some(&obs);
                            }
                        }
                    }
                }
                if let Some(next_obs) = next_obs {
                    for x in next_obs.1..pos.1 {
                        visit_map[pos.0][x] = '#';
                    }
                    pos.1 = next_obs.1 - 1;
                    pos.2 = Direction::UP;
                } else {
                    // no obstacle
                    for x in pos.1..visit_map[0].len() {
                        if visit_map[pos.0][x] == '.' {
                            visit_map[pos.0][x] = '#';
                        }
                    }
                    moving = false;
                }
            }
        }
    }
}
