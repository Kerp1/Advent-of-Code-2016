use common;
use std::collections::HashMap;
use std;


fn get_character_from_input_that_matches<F>(data : String, comparison: F) -> String
    where for<'r, 's> F: std::ops::FnMut<(&'r (&char, u32), &'s (&char, u32))>{

    let mut columns = std::iter::repeat(vec![]).take(8).collect::<Vec<_>>();
    for row in data.split_terminator("\n") {
        for (i, c) in row.chars().enumerate() {
            columns[i].push(c);
        }
    }
    let mut result = String::from("");

    for column in columns {
        let hash_map = column.iter().fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

        let mut vec: Vec<(&char, u32)> = hash_map.into_iter().collect();
        vec.sort_by(comparison);

        result.push(*vec[0].0);
    }
    result
}

pub fn task1() -> String {
    let data = common::read_file(String::from("input/day6.txt")).unwrap();
    let most_common = |&(_, v1), &(_, v2)| { v2.cmp(&v1)};
    get_character_from_input_that_matches(most_common)
}

pub fn task2() -> String {
    let data = common::read_file(String::from("input/day6.txt")).unwrap();
    let least_common = |&(_, v1), &(_, v2)| { v2.cmp(&v1)};
    get_character_from_input_that_matches(least_common)
}
