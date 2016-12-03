use common;


pub fn task1() -> Vec<i32> {
    let data = common::read_file(String::from("input/day2.txt")).unwrap();
    let rows = data.split_terminator("\n");

    let mut current_number = 5;
    let mut numbers = Vec::new();

    for row in rows {
        for character in row.chars() {
            current_number = move_finger(character, current_number, 1);
        }
        numbers.push(current_number);
    }
    return numbers;
}

pub fn task2() -> Vec<char> {
    let data = common::read_file(String::from("input/day2.txt")).unwrap();
    let rows = data.split_terminator("\n");

    let mut current_number = 5;
    let mut numbers = Vec::new();

    for row in rows {
        for character in row.chars() {
            current_number = move_finger(character, current_number, 2);
        }
        numbers.push(current_number);
    }
    return translate(numbers);
}

fn move_finger(direction: char, current_number: i32, task: i32) -> i32 {
    match direction {
        'U' => up(current_number, task),
        'D' => down(current_number, task),
        'L' => left(current_number, task),
        'R' => right(current_number, task),
        _ => panic!("Invalid direction")
    }
}

fn up(current_number: i32, task: i32) -> i32 {
    if task == 1 {
        match current_number {
            1 | 2 | 3 => current_number,
            _ => current_number - 3
        }
    } else {
        match current_number {
            1 | 2 | 4 | 5 | 9 => current_number,
            6 | 7 | 8 | 10 | 11 | 12 => current_number - 4,
            _ => current_number - 2,
        }
    }
}

fn down(current_number: i32, task: i32) -> i32 {
    if task == 1 {
        match current_number {
            7 | 8 | 9 => current_number,
            _ => current_number + 3
        }
    } else {
        match current_number {
            5 | 9 | 10 | 12 | 13 => current_number,
            2 | 3 | 4 | 6 | 7 | 8 => current_number + 4,
            _ => current_number + 2,
        }
    }
}

fn left(current_number: i32, task: i32) -> i32 {
    if task == 1 {
        match current_number {
            1 | 4 | 7 => current_number,
            _ => current_number - 1
        }
    } else {
        match current_number {
            1 | 2 | 5 | 10 | 13 => current_number,
            _ => current_number - 1
        }
    }
}

fn right(current_number: i32, task: i32) -> i32 {
    if task == 1 {
        match current_number {
            3 | 6 | 9 => current_number,
            _ => current_number + 1
        }
    } else {
        match current_number {
            1 | 4 | 9 | 12 | 13 => current_number,
            _ => current_number + 1
        }
    }
}

fn translate(numbers: Vec<i32>) -> Vec<char> {
    let mut characters = Vec::new();
    for number in numbers {
        let character = match number {
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            _ => number.to_string().chars().next().unwrap()
        };
        characters.push(character);
    }
    return characters;
}
