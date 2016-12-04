use common;

extern crate regex;

use std::char;
use regex::Regex;
use std::collections::HashMap;

struct Entry {
    name: String,
    sector_number: u32,
    checksum: String,
}

fn get_entries(data: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let re = Regex::new(r"(\D+?)(\d+?)\[(\D+?)\]").unwrap();

    for row in data.split_terminator("\n") {
        for capture in re.captures_iter(row) {
            let entry = Entry {
                name: String::from(capture.at(1).unwrap()),
                sector_number: capture.at(2).unwrap().parse::<u32>().unwrap(),
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

    let mut vec: Vec<(char, u32)> = map.into_iter().collect();

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

fn decrypt_character(character: char, rotate: u32) -> char {
    let mut cnum = character as u32;
    if cnum == '-' as u32 {
        return ' '
    } else {
        ;
        cnum -= 'a' as u32;
        cnum = (cnum + rotate) % ('z' as u32 - 'a' as u32 + 1);
        ;
        cnum += 'a' as u32;
        char::from_u32(cnum).unwrap()
    }
}

fn decrypt_name(name: &String, rotate: u32) -> String {
    let mut decrypted_name = String::new();
    for char in name.chars() {
        decrypted_name.push(decrypt_character(char, rotate));
    }
    decrypted_name
}

pub fn task1() -> u32 {
    let data = common::read_file(String::from("input/day4.txt")).unwrap();

    let entries = get_entries(&data);
    let entries: Vec<u32> = entries.iter().filter(|e| is_valid_entry(e)).map(|entry| entry.sector_number).collect();

    entries.iter().sum::<u32>()
}

pub fn task2() -> u32 {
    let data = common::read_file(String::from("input/day4.txt")).unwrap();
    let entries = get_entries(&data);

    for entry in entries {
        let name = decrypt_name(&entry.name, entry.sector_number);
        if name == "northpole object storage " {
            return entry.sector_number;
        }
    }
    return 0;
}