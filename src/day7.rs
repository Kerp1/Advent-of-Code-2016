extern crate regex;

use common;
use regex::Regex;

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

pub fn task1() -> u32 {
    let data = common::read_file(String::from("input/day7.txt")).unwrap();
    let extract_bracket_content = Regex::new(r".+?\[(.+?)\].+?").unwrap();
    //let tls = Regex::new(r".+?([a-z])((?!\1)(\w{1}))([a-z])((?!\3)\1{1}).+?").unwrap();

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