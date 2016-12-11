extern crate regex;

use common;
use regex::Regex;
use std::str;

fn has_xyyx_pattern(data: &str) -> bool {
    let bytes = data.as_bytes();
    for i in 0..bytes.len() - 3 {
        if bytes[i] == bytes[i + 3] && bytes[i + 1] == bytes[i + 2] &&
            bytes[i] != bytes[i + 1] && bytes[i] == bytes[i + 3] {
            return true;
        }
    }
    false
}

fn get_xyx_patterns(data: &str) -> Vec<&str> {
    let mut patterns = Vec::new();
    let bytes = data.as_bytes();
    let mut inside_bracket = false;
    for i in 0..bytes.len() - 2 {
        if bytes[i] == '[' as u8 {
            inside_bracket = true
        } else if bytes[i] == ']' as u8 {
            inside_bracket = false;
        }
        if bytes[i] == bytes[i + 2] && bytes[i] != bytes[i + 1] && !inside_bracket {
            let s = str::from_utf8(&bytes[i..i + 3]).unwrap();
            patterns.push(s);
        }
    }
    patterns
}

fn xyx_to_yxy(pattern: &&str) -> String {
    let bytes = pattern.as_bytes();
    let mut my_string = String::from("");
    my_string.push(bytes[1] as char);
    my_string.push(bytes[0] as char);
    my_string.push(bytes[1] as char);
    my_string
}

pub fn task1() -> u32 {
    let data = common::read_file(String::from("input/day7.txt")).unwrap();
    let extract_bracket_content = Regex::new(r".+?\[(.+?)\].+?").unwrap();

    let mut tls_count = 0;
    for row in data.split_terminator("\n") {
        let mut supports_tls = has_xyyx_pattern(row);
        if supports_tls {
            for capture in extract_bracket_content.captures_iter(row) {
                if has_xyyx_pattern(capture.at(1).unwrap_or("")) {
                    supports_tls = false;
                    break;
                }
            }
        }
        if supports_tls {
            tls_count += 1;
        }
    }
    tls_count
}


pub fn task2() -> u32 {
    let data = common::read_file(String::from("input/day7.txt")).unwrap();
    let extract_bracket_content = Regex::new(r".*?\[(.+?)\].*?").unwrap();
    let mut ssl_count = 0;
    for row in data.split_terminator("\n") {
        let patterns = get_xyx_patterns(row).iter().map(xyx_to_yxy).collect::<Vec<String>>();
        for capture in extract_bracket_content.captures_iter(row) {
            if patterns.iter().any(|pattern| { capture.at(1).unwrap_or("").contains(pattern) }) {
                ssl_count += 1;
                break;
            }
        }
    }
    ssl_count
}
