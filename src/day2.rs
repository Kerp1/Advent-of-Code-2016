use common;


pub fn task1() -> Vec<i32> {
    let data = common::read_file(String::from("input/day2.txt")).unwrap();
    let rows = data.split_terminator("\n");

    let mut current_number = 5;
    let mut numbers = Vec::new();

    for row in rows {
        for character in row.chars() {
            current_number = move_finger(character, current_number);
        }
        numbers.push(current_number);
    }
    return numbers;
}

fn move_finger(direction: char, current_number: i32) -> i32 {
    match direction {
        'U' => up(current_number),
        'D' => down(current_number),
        'L' => left(current_number),
        'R' => right(current_number),
        _ => panic!("Invalid direction")
    }
}

fn up(current_number: i32) -> i32 {
    match current_number {
        1 | 2 | 3 => current_number,
        _ => current_number - 3
    }
}

fn down(current_number: i32) -> i32 {
    match current_number {
        7 | 8 | 9 => current_number,
        _ => current_number + 3
    }
}

fn left(current_number: i32) -> i32 {
    match current_number {
        1 | 4 | 7 => current_number,
        _ => current_number - 1
    }
}

fn right(current_number: i32) -> i32 {
    match current_number {
        3 | 6 | 9 => current_number,
        _ => current_number + 1
    }
}
