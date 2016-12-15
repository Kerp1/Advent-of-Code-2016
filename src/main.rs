#![feature(unboxed_closures)]

extern crate regex;
extern crate crypto;

mod common;
mod day9;

fn main() {
    println!("day9::task1() = {:?}", day9::task1());
}
