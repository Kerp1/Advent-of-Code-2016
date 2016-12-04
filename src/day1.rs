use common;

extern crate regex;

use std::collections::HashMap;
use regex::Regex;

enum Direction {
    North,
    South,
    West,
    East,
}

fn get_new_facing(current_dir: &Direction, turn_dir: &str) -> Direction {
    let new_dir = match turn_dir {
        "R" => {
            match *current_dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            }
        }
        "L" => {
            match *current_dir {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            }
        }
        _ => panic!("no"),
    };
    return new_dir;
}

fn update_position(mut x: i32, mut y: i32, current_dir: &Direction, distance: i32) -> (i32, i32) {
    x += match *current_dir {
        Direction::East => distance,
        Direction::West => -distance,
        _ => 0,
    };

    y += match *current_dir {
        Direction::North => distance,
        Direction::South => -distance,
        _ => 0,
    };

    (x, y)
}

pub fn task1() -> i32 {
    let mut currently_facing = Direction::North;

    let mut x = 0;
    let mut y = 0;

    let data = common::read_file(String::from("input/day1.txt")).unwrap();
    let re = Regex::new(r"(R|L)(\d+)").unwrap();
    let split = data.split(",");


    for split_string in split {
        for capture in re.captures_iter(split_string) {
            let direction = capture.at(1).unwrap();
            let distance = capture.at(2).unwrap_or("").parse::<i32>().unwrap();

            currently_facing = get_new_facing(&currently_facing, direction);
            let (x1, y1) = update_position(x, y, &currently_facing, distance);
            x = x1;
            y = y1;
        }
    }
    x.abs() + y.abs()
}

pub fn task2() -> i32 {
    let mut currently_facing = Direction::North;

    let mut x = 0;
    let mut y = 0;

    let data = common::read_file(String::from("input/day1.txt")).unwrap();
    let re = Regex::new(r"(R|L)(\d+)").unwrap();
    let split = data.split(",");

    let mut positions = HashMap::new();


    for split_string in split {
        for capture in re.captures_iter(split_string) {
            let direction = capture.at(1).unwrap();
            let distance = capture.at(2).unwrap_or("").parse::<i32>().unwrap();

            currently_facing = get_new_facing(&currently_facing, direction);
            let (end_x, end_y) = update_position(x, y, &currently_facing, distance);


            if x + end_x > 0 {
                for i in x..end_x {
                    if positions.insert((i, y), true) != None {
                        return i.abs() + end_y.abs();
                    }
                }
            } else {
                for i in (x..end_x).rev() {
                    if positions.insert((i, y), true) != None {
                        return i.abs() + end_y.abs();
                    }
                }
            }

            if y + end_y > 0 {
                for i in y..end_y {
                    if positions.insert((x, i), true) != None {
                        return end_x.abs() + i.abs();
                    }
                }
            } else {
                for i in (y..end_y).rev() {
                    if positions.insert((x, i), true) != None {
                        return end_x.abs() + i.abs();
                    }
                }
            }

            x = end_x;
            y = end_y;
        }
    }
    return 0;
}
