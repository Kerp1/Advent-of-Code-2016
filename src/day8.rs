extern crate regex;

use common;
use regex::Regex;

fn create_screen(width: usize, height: usize) -> Vec<Vec<u32>> {
    let mut screen = Vec::new();
    for i in 0..height {
        screen.push(Vec::new());
        for _ in 0..width {
            screen[i].push(0);
        }
    }
    screen
}

fn handle_rect(x: usize, y: usize, screen: &mut Vec<Vec<u32>>) {
    for i in 0..x {
        for j in 0..y {
            screen[j][i] = 1;
        }
    }
}

fn handle_rotation(rotation_type: &str, index: usize, amount: usize, screen: &mut Vec<Vec<u32>>) {
    if rotation_type == "column" {
        let mut new_column = vec![0; screen.len()];
        for i in 0..screen.len() {
            new_column[(i + amount) % screen.len()] = screen[i][index];
        }

        for i in 0..new_column.len() {
            screen[i][index] = new_column[i];
        }
    } else if rotation_type == "row" {
        let mut new_row = vec![0; screen[index].len()];
        for i in 0..screen[index].len() {
            new_row[(i + amount) % screen[index].len()] = screen[index][i];
        }
        screen[index] = new_row;
    }
}


pub fn task1() -> u32 {
    let data = common::read_file(String::from("input/day8.txt")).unwrap();
    let mut screen = create_screen(50, 6);

    let rect = Regex::new(r"rect\s+(\d+)x(\d+)").unwrap();
    let rotation = Regex::new(r"rotate\s+(row|column)\s+(x|y)=(\d+)\s+by\s+(\d+)").unwrap();

    for row in data.split_terminator("\n") {
        for capture in rect.captures_iter(row) {
            handle_rect(capture.at(1).unwrap().parse::<usize>().unwrap(),
                        capture.at(2).unwrap().parse::<usize>().unwrap(),
                        &mut screen);
        }
        for capture in rotation.captures_iter(row) {
            handle_rotation(capture.at(1).unwrap(),
                            capture.at(3).unwrap().parse::<usize>().unwrap(),
                            capture.at(4).unwrap().parse::<usize>().unwrap(),
                            &mut screen);
        }
    }
    screen.iter().map(|v: &Vec<u32>| v.iter().sum()).collect::<Vec<u32>>().iter().sum()
}
