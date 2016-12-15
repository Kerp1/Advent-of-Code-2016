extern crate regex;

use common;
use regex::Regex;


#[derive(Debug)]
struct Marker {
    string: String,
    amount: usize,
    length: usize,
}


fn extract_marker(slice: &str, regex: &regex::Regex) -> Marker {
    let capture = regex.captures_iter(slice).next().unwrap();
    Marker {
        string: String::from(capture.at(0).unwrap_or("")),
        length: capture.at(1).unwrap_or("").parse().unwrap(),
        amount: capture.at(2).unwrap_or("").parse().unwrap(),
    }
}


pub fn task1() -> u32 {
    let data =
        common::read_file(String::from("input/day9.txt")).unwrap().replace(char::is_whitespace, "");

    let regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    let mut skip_amount = 0;
    let mut current_length = 0;
    for (i, character) in data.chars().enumerate() {
        if skip_amount > i {
            continue;
        }
        if character == '(' {
            let marker = extract_marker(data.split_at(i).1, &regex);
            current_length += marker.length * marker.amount;
            skip_amount = i + marker.string.len() + marker.length;
        }
    }

    current_length as u32

}
