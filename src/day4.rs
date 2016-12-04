use common;

extern crate regex;

use regex::Regex;
use std::collections::HashMap;

struct Entry {
    name: String,
    sector_number: i32,
    checksum: String,
}

fn get_entries(data: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let re = Regex::new(r"(\D+?)(\d+?)\[(\D+?)\]").unwrap();

    for row in data.split_terminator("\n") {
        for capture in re.captures_iter(row) {
            let entry = Entry {
                name: String::from(capture.at(1).unwrap()),
                sector_number: capture.at(2).unwrap().parse::<i32>().unwrap(),
                checksum: String::from(capture.at(3).unwrap())
            };

            entries.push(entry);
        }
    }
    return entries;
}

fn get_checksum(name: &String) -> String {
    let name: String = name.chars().filter(|c| c.is_alphabetic()).collect();

    let map = name.chars().fold(HashMap::new(), |mut map, x| {
        {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        }
        map
    });

    let mut vec: Vec<(char, i32)> = map.into_iter().collect();

    vec.sort_by(|&(k1, v1), &(k2, v2)| {
        if v1 == v2 {
            k1.cmp(&k2)
        } else {
            v2.cmp(&v1)
        }
    });

    (*vec.split_at(5).0).iter().fold(String::new(), |mut str, &(c, _)| {
        str.push(c);
        str
    })
}

fn is_valid_entry(entry: &Entry) -> bool {
    get_checksum(&entry.name) == entry.checksum
}

pub fn task1() -> i32 {
    let data = common::read_file(String::from("input/day4.txt")).unwrap();

    let entries = get_entries(&data);
    let entries: Vec<i32> = entries.iter().filter(|e| is_valid_entry(e)).map(|entry| entry.sector_number).collect();

    entries.iter().sum::<i32>()
}
