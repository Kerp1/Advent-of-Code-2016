use std::io::prelude::*;
use std::fs::File;
use std::io::Error;


pub fn read_file(file_name: String) -> Result<String, Error> {
    let mut file = File::open(file_name)?;

    let mut s = String::new();

    file.read_to_string(&mut s)?;

    Ok(s)
}